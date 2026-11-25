//! Preliminary venue eligibility for the approved attendance and evidence policy.

mod budget;
mod rules;

use std::{collections::BTreeMap, num::NonZeroU32};

use chrono::{NaiveDate, Weekday};

use crate::{
    dedup::{Deduplication, PotentialDuplicate, VenueEntity},
    domain::{Criterion, Currency, Eligibility, EventRequest, MinorUnits, Money, Venue},
};

pub use budget::{BudgetAssessment, BudgetGap, CostEvidence};
use rules::{accessibility, availability, breakouts, quality, seating, transport};

/// The result of one hard requirement on one source configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementOutcome {
    /// The source supports a preliminary pass.
    Satisfied,
    /// The source establishes a hard failure.
    Violated,
    /// More evidence is needed for a decision.
    Uncertain,
}

/// A hard requirement and the reason for its outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequirementAssessment {
    /// Business dimension being checked.
    pub criterion: Criterion,
    /// Outcome supported by the available evidence.
    pub outcome: RequirementOutcome,
    /// Explanation of the outcome.
    pub reason: &'static str,
}

/// Date-level evidence for one configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct ListingDate {
    /// Date being assessed.
    pub date: NaiveDate,
    /// Evidence for booking on this day, before checking other criteria.
    pub availability: RequirementAssessment,
    /// Verdict including all of the listing's hard checks.
    pub eligibility: Eligibility,
}

/// Checks for one complete source configuration, with its original evidence.
#[derive(Debug, PartialEq, Eq)]
pub struct ListingAssessment<'a> {
    /// Source listing and provenance.
    pub listing: &'a Venue,
    /// All date-independent requirement results, in criterion order.
    pub requirements: Vec<RequirementAssessment>,
    /// Comparable cost evidence and outstanding budget questions.
    pub budget: BudgetAssessment,
    /// Requested dates and their source-specific verdicts in calendar order.
    pub dates: Vec<ListingDate>,
}

/// Why a date needs confirmation at entity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationCause {
    /// One or more requirements need further evidence.
    ListingEvidence,
    /// Confirmed listings differ in whether they support this date.
    SourceDisagreement,
    /// The bookable space may duplicate another listing.
    PotentialDuplicate,
}

/// Combined outcome for a requested day across confirmed source configurations.
#[derive(Debug, PartialEq, Eq)]
pub struct ScenarioDate {
    /// Date being assessed.
    pub date: NaiveDate,
    /// Eligibility for this date, including material identity uncertainty.
    pub eligibility: Eligibility,
    /// Cause of an unresolved decision, if any.
    pub confirmation: Option<ConfirmationCause>,
}

/// Eligibility for one attendance scenario.
#[derive(Debug, PartialEq, Eq)]
pub struct ScenarioAssessment<'a> {
    /// Number of seated attendees.
    pub attendees: NonZeroU32,
    /// Verdict across the requested dates.
    pub eligibility: Eligibility,
    /// Per-configuration requirements and provenance.
    pub listings: Vec<ListingAssessment<'a>>,
    /// Per-date verdicts in calendar order.
    pub dates: Vec<ScenarioDate>,
}

/// Primary and expansion eligibility for one deduplicated entity.
#[derive(Debug, PartialEq, Eq)]
pub struct VenueAssessment<'a> {
    /// All source configurations and confirmed field conflicts.
    pub entity: &'a VenueEntity<'a>,
    /// Potential pairs for which this entity's bookable identity needs verification.
    pub potential_duplicates: Vec<&'a PotentialDuplicate<'a>>,
    /// Assessment for the primary attendance.
    pub primary: ScenarioAssessment<'a>,
    /// Independent assessment for the expansion attendance, when requested.
    pub expansion: Option<ScenarioAssessment<'a>>,
}

/// Returns the candidate-approved planning inputs.
///
/// The booking reference is left unspecified. A caller can supply one to
/// assess minimum notice without consulting the system clock.
#[must_use]
pub fn partner_day_request() -> EventRequest {
    EventRequest {
        seated_attendees: NonZeroU32::MIN.saturating_add(79),
        expansion_attendees: NonZeroU32::new(120),
        minimum_breakouts: 3,
        budget: Money {
            currency: Currency::Eur,
            minor_units: MinorUnits::new(1_500_000),
        },
        candidate_dates: [11, 16, 18]
            .into_iter()
            .filter_map(|day| NaiveDate::from_ymd_opt(2027, 3, day))
            .collect(),
        preferred_weekday: Some(Weekday::Thu),
        booking_reference_date: None,
    }
}

/// Evaluates each entity in deduplication order against an explicit request.
///
/// Every source configuration is assessed independently per date. Agreement
/// on a passing date is required across confirmed listings. Material potential
/// identity keeps an otherwise passing entity in follow-up.
#[must_use]
pub fn assess_venues<'a>(
    deduplication: &'a Deduplication<'a>,
    request: &EventRequest,
) -> Vec<VenueAssessment<'a>> {
    let mut dates = request.candidate_dates.clone();
    dates.sort_unstable();
    dates.dedup();

    let mut pairs_by_source = BTreeMap::<_, Vec<_>>::new();
    for (index, pair) in deduplication.potential_duplicates.iter().enumerate() {
        for listing in [pair.left, pair.right] {
            pairs_by_source
                .entry(first_source_index(listing))
                .or_default()
                .push((index, pair));
        }
    }
    deduplication
        .entities
        .iter()
        .map(|entity| {
            let mut matched_pairs: Vec<_> = entity
                .listings()
                .iter()
                .flat_map(|listing| {
                    pairs_by_source
                        .get(&first_source_index(listing))
                        .into_iter()
                        .flatten()
                        .filter(move |(_, pair)| {
                            std::ptr::eq(*listing, pair.left) || std::ptr::eq(*listing, pair.right)
                        })
                        .copied()
                })
                .collect();
            if entity.listings().len() > 1 {
                matched_pairs.sort_unstable_by_key(|(index, _)| *index);
            }
            matched_pairs.dedup_by_key(|(index, _)| *index);
            let potential_duplicates: Vec<_> =
                matched_pairs.into_iter().map(|(_, pair)| pair).collect();
            let assess = |attendees| {
                scenario(
                    entity,
                    request,
                    attendees,
                    &dates,
                    !potential_duplicates.is_empty(),
                )
            };
            let primary = assess(request.seated_attendees);
            let expansion = request.expansion_attendees.map(assess);
            VenueAssessment {
                entity,
                potential_duplicates,
                primary,
                expansion,
            }
        })
        .collect()
}

fn first_source_index(venue: &Venue) -> Option<usize> {
    venue.sources.first().map(|source| source.record_index)
}

fn scenario<'a>(
    entity: &VenueEntity<'a>,
    request: &EventRequest,
    attendees: NonZeroU32,
    dates: &[NaiveDate],
    uncertain_identity: bool,
) -> ScenarioAssessment<'a> {
    let listings: Vec<_> = entity
        .listings()
        .iter()
        .map(|listing| assess_listing(listing, request, attendees, dates))
        .collect();
    let dates: Vec<_> = dates
        .iter()
        .enumerate()
        .map(|(index, &date)| {
            let mut results = listings
                .iter()
                .filter_map(|listing| listing.dates.get(index));
            let (eligibility, confirmation) = match results.next() {
                Some(first) if results.all(|other| other.eligibility == first.eligibility) => {
                    match first.eligibility {
                        Eligibility::Eligible if uncertain_identity => (
                            Eligibility::RequiresConfirmation,
                            Some(ConfirmationCause::PotentialDuplicate),
                        ),
                        Eligibility::RequiresConfirmation => (
                            Eligibility::RequiresConfirmation,
                            Some(ConfirmationCause::ListingEvidence),
                        ),
                        verdict => (verdict, None),
                    }
                }
                _ => (
                    Eligibility::RequiresConfirmation,
                    Some(ConfirmationCause::SourceDisagreement),
                ),
            };
            ScenarioDate {
                date,
                eligibility,
                confirmation,
            }
        })
        .collect();
    let all_dates_rejected = if dates.is_empty() {
        listings.iter().all(|listing| {
            listing
                .requirements
                .iter()
                .any(|result| result.outcome == RequirementOutcome::Violated)
        })
    } else {
        dates
            .iter()
            .all(|day| day.eligibility == Eligibility::Rejected)
    };
    let eligibility = if dates
        .iter()
        .any(|day| day.eligibility == Eligibility::Eligible)
    {
        Eligibility::Eligible
    } else if all_dates_rejected {
        Eligibility::Rejected
    } else {
        Eligibility::RequiresConfirmation
    };

    ScenarioAssessment {
        attendees,
        eligibility,
        listings,
        dates,
    }
}

fn assess_listing<'a>(
    listing: &'a Venue,
    request: &EventRequest,
    attendees: NonZeroU32,
    dates: &[NaiveDate],
) -> ListingAssessment<'a> {
    let budget = budget::assess(listing, &request.budget, attendees.get());
    let requirements = vec![
        seating(&listing.capacity, attendees.get()),
        breakouts(listing, request.minimum_breakouts),
        budget.requirement,
        accessibility(listing, request.minimum_breakouts > 0),
        transport(&listing.station_access),
        quality(
            Criterion::Daylight,
            &listing.daylight,
            listing.daylight_qualification.as_deref(),
        ),
        quality(
            Criterion::Wifi,
            &listing.wifi,
            listing.wifi_qualification.as_deref(),
        ),
    ];
    let dates = dates
        .iter()
        .map(|&date| {
            let availability =
                availability(&listing.availability, date, request.booking_reference_date);
            let outcomes = requirements
                .iter()
                .map(|result| result.outcome)
                .chain([availability.outcome]);
            ListingDate {
                date,
                availability,
                eligibility: aggregate(outcomes),
            }
        })
        .collect();
    ListingAssessment {
        listing,
        requirements,
        budget,
        dates,
    }
}

fn aggregate(outcomes: impl IntoIterator<Item = RequirementOutcome>) -> Eligibility {
    let mut eligibility = Eligibility::Eligible;
    for outcome in outcomes {
        match outcome {
            RequirementOutcome::Violated => return Eligibility::Rejected,
            RequirementOutcome::Uncertain => eligibility = Eligibility::RequiresConfirmation,
            RequirementOutcome::Satisfied => {}
        }
    }
    eligibility
}

fn check(
    criterion: Criterion,
    outcome: RequirementOutcome,
    reason: &'static str,
) -> RequirementAssessment {
    RequirementAssessment {
        criterion,
        outcome,
        reason,
    }
}
