//! Deterministic recommendation ordering over authoritative eligibility assessments.

use std::cmp::{Ordering, Reverse};

use chrono::{Datelike, NaiveDate};

use crate::{
    domain::{
        Criterion, Currency, Eligibility, EventRequest, Evidence, MinorUnits, Quality, Venue,
    },
    evaluate::{CostEvidence, ListingAssessment, RequirementOutcome, VenueAssessment},
};

/// Supported transport convenience in order of preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransportPreference {
    /// Documented walk of at most 15 minutes from Utrecht Centraal.
    ShortCentraalWalk(u32),
    /// Longer documented walk from Utrecht Centraal.
    LongerCentraalWalk(u32),
    /// Other public transport evidence accepted during eligibility.
    DocumentedConnection,
    /// No supported convenience comparison.
    Unconfirmed,
}

/// Preliminary quality evidence in order of preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityPreference {
    /// Detailed positive evidence.
    Strong,
    /// Positive preliminary evidence.
    Acceptable,
    /// No supported quality comparison.
    Unconfirmed,
}

/// The evidence used to order an eligible recommendation.
#[derive(Debug, PartialEq, Eq)]
pub struct RankingFactors {
    /// Earliest supported date on the preferred weekday, or earliest supported date.
    pub recommended_date: Option<NaiveDate>,
    /// Whether the recommended date matches the preference.
    pub preferred_weekday: bool,
    /// Supported transport convenience.
    pub transport: TransportPreference,
    /// Daylight evidence strength.
    pub daylight: QualityPreference,
    /// Wi-Fi evidence strength.
    pub wifi: QualityPreference,
    /// Complete EUR cost when all confirmed source configurations agree.
    pub comparable_total: Option<MinorUnits>,
    /// Remaining EUR budget where only comparable cost is used.
    pub budget_headroom: Option<MinorUnits>,
    /// Independent expansion verdict, if evaluated.
    pub expansion: Option<Eligibility>,
}

impl RankingFactors {
    fn key(
        &self,
    ) -> (
        bool,
        TransportPreference,
        QualityPreference,
        QualityPreference,
        Reverse<Option<u64>>,
        u8,
    ) {
        (
            !self.preferred_weekday,
            self.transport,
            self.daylight,
            self.wifi,
            Reverse(self.budget_headroom.map(MinorUnits::get)),
            match self.expansion {
                Some(Eligibility::Eligible) => 0,
                Some(Eligibility::RequiresConfirmation) => 1,
                Some(Eligibility::Rejected) => 2,
                None => 3,
            },
        )
    }
}

/// One ranked entity, retaining both eligibility scenarios and all sources.
#[derive(Debug, PartialEq, Eq)]
pub struct Recommendation<'a> {
    /// One-based place in the eligible shortlist.
    pub rank: usize,
    /// Authoritative eligibility, source IDs and explanations.
    pub assessment: VenueAssessment<'a>,
    /// The compared evidence, in policy priority order.
    pub factors: RankingFactors,
}

/// Eligible, conditional and rejected recommendation entries.
#[derive(Debug, PartialEq, Eq)]
pub struct Shortlist<'a> {
    /// Ranked primary-scenario candidates.
    pub eligible: Vec<Recommendation<'a>>,
    /// Conditional primary-scenario candidates, in stable identity order.
    pub conditional: Vec<VenueAssessment<'a>>,
    /// Rejected primary-scenario candidates, in stable identity order.
    pub rejected: Vec<VenueAssessment<'a>>,
}

/// Ranks the primary scenario using the same event request used for eligibility.
///
/// Hard verdicts are preserved. All eligible entries are returned so the caller
/// can apply a presentation limit if required.
#[must_use]
pub fn rank_venues<'a>(
    assessments: Vec<VenueAssessment<'a>>,
    request: &EventRequest,
) -> Shortlist<'a> {
    let mut shortlist = Shortlist {
        eligible: Vec::new(),
        conditional: Vec::new(),
        rejected: Vec::new(),
    };
    for assessment in assessments {
        match assessment.primary.eligibility {
            Eligibility::Eligible => shortlist.eligible.push(Recommendation {
                rank: 0,
                factors: factors(&assessment, request),
                assessment,
            }),
            Eligibility::RequiresConfirmation => shortlist.conditional.push(assessment),
            Eligibility::Rejected => shortlist.rejected.push(assessment),
        }
    }
    shortlist.eligible.sort_unstable_by(|left, right| {
        left.factors
            .key()
            .cmp(&right.factors.key())
            .then_with(|| identity_order(&left.assessment, &right.assessment))
    });
    shortlist.conditional.sort_unstable_by(identity_order);
    shortlist.rejected.sort_unstable_by(identity_order);
    for (index, entry) in shortlist.eligible.iter_mut().enumerate() {
        entry.rank = index + 1;
    }
    shortlist
}

fn factors(assessment: &VenueAssessment<'_>, request: &EventRequest) -> RankingFactors {
    let is_preferred = |date: NaiveDate| Some(date.weekday()) == request.preferred_weekday;
    let recommended_date = assessment
        .primary
        .dates
        .iter()
        .filter(|date| date.eligibility == Eligibility::Eligible)
        .map(|date| date.date)
        .min_by_key(|&date| (!is_preferred(date), date));
    let listings = &assessment.primary.listings;
    let transport = listings
        .iter()
        .map(transport_preference)
        .max()
        .unwrap_or(TransportPreference::Unconfirmed);
    let quality = |select: fn(&Venue) -> &Evidence<Quality>| {
        listings
            .iter()
            .map(|listing| match select(listing.listing) {
                Evidence::Known(Quality::Strong) => QualityPreference::Strong,
                Evidence::Known(Quality::Acceptable) => QualityPreference::Acceptable,
                _ => QualityPreference::Unconfirmed,
            })
            .max()
            .unwrap_or(QualityPreference::Unconfirmed)
    };
    let comparable_total = if request.budget.currency == Currency::Eur {
        comparable_total(listings)
    } else {
        None
    };
    let budget_headroom = comparable_total.and_then(|total| {
        request
            .budget
            .minor_units
            .get()
            .checked_sub(total.get())
            .map(MinorUnits::new)
    });
    RankingFactors {
        recommended_date,
        preferred_weekday: recommended_date.is_some_and(is_preferred),
        transport,
        daylight: quality(|venue| &venue.daylight),
        wifi: quality(|venue| &venue.wifi),
        comparable_total,
        budget_headroom,
        expansion: assessment
            .expansion
            .as_ref()
            .map(|scenario| scenario.eligibility),
    }
}

fn transport_preference(assessment: &ListingAssessment<'_>) -> TransportPreference {
    use TransportPreference::{
        DocumentedConnection, LongerCentraalWalk, ShortCentraalWalk, Unconfirmed,
    };
    if !assessment.requirements.iter().any(|rule| {
        rule.criterion == Criterion::Transport && rule.outcome == RequirementOutcome::Satisfied
    }) {
        return Unconfirmed;
    }
    let access = &assessment.listing.station_access;
    match (&access.station, &access.walk) {
        (Evidence::Known(station), Evidence::Known(walk))
            if station.eq_ignore_ascii_case("Utrecht Centraal") =>
        {
            if walk.minutes <= 15 {
                ShortCentraalWalk(walk.minutes)
            } else {
                LongerCentraalWalk(walk.minutes)
            }
        }
        _ => DocumentedConnection,
    }
}

fn comparable_total(listings: &[ListingAssessment<'_>]) -> Option<MinorUnits> {
    let (first, remaining) = listings.split_first()?;
    let CostEvidence::Complete(total) = first.budget.cost else {
        return None;
    };
    remaining
        .iter()
        .all(|listing| listing.budget.cost == CostEvidence::Complete(total))
        .then_some(total)
}

fn identity_order(left: &VenueAssessment<'_>, right: &VenueAssessment<'_>) -> Ordering {
    let left = left.entity.listings();
    let right = right.entity.listings();
    left.iter()
        .map(|venue| identity(venue))
        .min()
        .cmp(&right.iter().map(|venue| identity(venue)).min())
        .then_with(|| {
            // When listing IDs collide, compare the fixed semantic fields before
            // using source position. Raw JSON permutations cannot reorder distinct claims.
            let mut left: Vec<_> = left.iter().map(|venue| configuration_key(venue)).collect();
            let mut right: Vec<_> = right.iter().map(|venue| configuration_key(venue)).collect();
            left.sort_unstable();
            right.sort_unstable();
            left.cmp(&right)
        })
        .then_with(|| {
            let source_index = |venues: &[&Venue]| {
                venues
                    .iter()
                    .flat_map(|venue| &venue.sources)
                    .map(|source| source.record_index)
                    .min()
            };
            source_index(left).cmp(&source_index(right))
        })
}

fn identity(venue: &Venue) -> (&str, &str, Option<&str>, Option<&str>) {
    let address = match &venue.address {
        Evidence::Known(address) => Some(address.as_str()),
        _ => None,
    };
    (
        venue.id.as_str(),
        &venue.name,
        venue.city.as_deref(),
        address,
    )
}

fn configuration_key(venue: &Venue) -> String {
    // These typed fields use stable scalar, string and sequence Debug forms.
    format!(
        "{:?}",
        (
            identity(venue),
            &venue.address,
            &venue.capacity,
            &venue.breakouts,
            &venue.price,
            &venue.catering,
            &venue.accessibility,
            &venue.availability,
            &venue.station_access,
            (
                &venue.daylight,
                &venue.daylight_qualification,
                &venue.wifi,
                &venue.wifi_qualification
            ),
            &venue.notes,
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparable_headroom_order_is_transitive_and_zero_is_real_evidence() {
        let factors = |headroom: Option<u64>| RankingFactors {
            recommended_date: None,
            preferred_weekday: false,
            transport: TransportPreference::DocumentedConnection,
            daylight: QualityPreference::Acceptable,
            wifi: QualityPreference::Acceptable,
            comparable_total: None,
            budget_headroom: headroom.map(MinorUnits::new),
            expansion: Some(Eligibility::Eligible),
        };
        let large = factors(Some(u64::MAX)).key();
        let zero = factors(Some(0)).key();
        let unknown = factors(None).key();
        assert!(large < zero);
        assert!(zero < unknown);
        assert!(large < unknown);
        assert_eq!(zero.cmp(&large), Ordering::Greater);
    }
}
