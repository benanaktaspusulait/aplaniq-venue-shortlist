//! Venue evidence and event inputs shared by ingestion and assessment.

use std::num::NonZeroU32;

use chrono::{NaiveDate, Weekday};

/// Stable identifier supplied by a venue listing.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VenueId(String);

impl VenueId {
    /// Creates an identifier if the source text is nonblank.
    #[must_use]
    pub fn new(value: String) -> Option<Self> {
        if value.trim().is_empty() {
            return None;
        }

        Some(Self(value))
    }

    /// Borrows the identifier exactly as supplied.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Location of a record or field in the input export.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceRef {
    /// Zero-based position in the original venues array.
    pub record_index: usize,
    /// Listing identifier, when the source supplied a valid one.
    pub venue_id: Option<VenueId>,
    /// Field path relative to the row, or absent for the whole row.
    pub field: Option<String>,
}

/// The state of a listing claim before applying any shortlist rules.
#[derive(Debug, PartialEq, Eq)]
pub enum Evidence<T> {
    /// The field supplied a supported typed claim.
    Known(T),
    /// The source omitted the field.
    Missing,
    /// The source supplied no usable claim, such as a null or on-request price.
    Unknown,
    /// The source made a claim that cannot be represented as valid evidence.
    Invalid {
        /// Original value that could not be interpreted.
        raw: String,
    },
    /// Trace the disagreeing claims through their source records.
    Conflicting,
}

/// Event constraints and date preferences shared by the assessment pipeline.
#[derive(Debug, PartialEq, Eq)]
pub struct EventRequest {
    /// Seated attendance for the primary scenario.
    pub seated_attendees: NonZeroU32,
    /// Optional seated attendance for the expansion scenario.
    pub expansion_attendees: Option<NonZeroU32>,
    /// Minimum number of simultaneous breakout rooms.
    pub minimum_breakouts: u32,
    /// Working budget ceiling and comparison currency.
    pub budget: Money,
    /// Calendar days to consider.
    pub candidate_dates: Vec<NaiveDate>,
    /// Weekday preference used when several days qualify.
    pub preferred_weekday: Option<Weekday>,
    /// Reference used to assess minimum booking notice.
    pub booking_reference_date: Option<NaiveDate>,
}

/// Normalised venue evidence before identity or eligibility decisions.
#[derive(Debug, PartialEq, Eq)]
pub struct Venue {
    /// Identifier of the normalised venue.
    pub id: VenueId,
    /// Display name from the listing.
    pub name: String,
    /// Reported city, when present.
    pub city: Option<String>,
    /// Source address evidence.
    pub address: Evidence<String>,
    /// Source records that supplied the venue facts.
    pub sources: Vec<SourceRef>,
    /// Plenary capacity evidence by layout.
    pub capacity: Evidence<Capacity>,
    /// Breakout count and any qualification.
    pub breakouts: Evidence<Breakouts>,
    /// Price evidence, which may contain incomplete components.
    pub price: Evidence<Price>,
    /// Catering evidence independent of the quote.
    pub catering: Evidence<CateringStatus>,
    /// Additional mandatory cost evidence from the listing notes.
    pub additional_charges: AdditionalCharges,
    /// Source claims about wheelchair access.
    pub accessibility: Evidence<Accessibility>,
    pub(crate) room_access_restrictions: Vec<RoomAccessRestriction>,
    /// Reported booking calendar and restrictions.
    pub availability: Evidence<Availability>,
    /// Transport evidence and station details.
    pub station_access: StationAccess,
    /// Daylight evidence for event spaces.
    pub daylight: Evidence<Quality>,
    /// Reported scope of a qualified natural-light claim.
    pub daylight_qualification: Option<String>,
    /// Network quality and coverage evidence.
    pub wifi: Evidence<Quality>,
    /// Reported scope of a qualified network claim.
    pub wifi_qualification: Option<String>,
    /// Additional source notes retained as text.
    pub notes: Option<String>,
}

/// Seated and standing capacities reported for the plenary.
#[derive(Debug, PartialEq, Eq)]
pub struct Capacity {
    /// Reported seated capacities by room layout.
    pub seated: Vec<SeatedCapacity>,
    /// Reported standing count, separate from seating.
    pub standing: Option<NonZeroU32>,
}

/// Room arrangement associated with a seated count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatingLayout {
    /// The source gave a capacity without naming a layout.
    Unspecified,
    /// Seating is explicit and the arrangement is unspecified.
    Seated,
    /// Theatre seating arrangement.
    Theatre,
    /// Cabaret seating arrangement.
    Cabaret,
}

/// Positive, inclusive seated capacity bounds for one layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeatedCapacity {
    /// Seating arrangement for these counts.
    pub layout: SeatingLayout,
    bounds: CountRange,
}

impl SeatedCapacity {
    /// Creates seated bounds if the positive minimum does not exceed the maximum.
    #[must_use]
    pub fn new(layout: SeatingLayout, minimum: NonZeroU32, maximum: NonZeroU32) -> Option<Self> {
        Some(Self {
            layout,
            bounds: CountRange::new(minimum.get(), maximum.get())?,
        })
    }

    /// Returns the inclusive lower and upper seated counts.
    #[must_use]
    pub fn bounds(self) -> CountRange {
        self.bounds
    }
}

/// An inclusive count range. Zero is valid for breakout inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountRange {
    minimum: u32,
    maximum: u32,
}

impl CountRange {
    /// Creates inclusive bounds if the minimum does not exceed the maximum.
    #[must_use]
    pub fn new(minimum: u32, maximum: u32) -> Option<Self> {
        if minimum > maximum {
            return None;
        }

        Some(Self { minimum, maximum })
    }

    /// Returns the inclusive lower bound.
    #[must_use]
    pub fn minimum(self) -> u32 {
        self.minimum
    }

    /// Returns the inclusive upper bound.
    #[must_use]
    pub fn maximum(self) -> u32 {
        self.maximum
    }
}

/// Listed count of rooms and any caveat about their use.
#[derive(Debug, PartialEq, Eq)]
pub struct Breakouts {
    /// Inclusive bounds on the listed room count.
    pub count: CountRange,
    /// Source caveat about which rooms are included.
    pub qualification: Option<String>,
}

/// Mandatory charges outside the quoted price.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct AdditionalCharges {
    /// Known minimum additional amount in EUR, if supported.
    pub minimum_eur: Option<MinorUnits>,
    /// Whether the charges, their scope or their VAT treatment need confirmation.
    pub unresolved: bool,
}

/// Currency of a monetary claim without conversion.
#[derive(Debug, PartialEq, Eq)]
pub enum Currency {
    /// Euro.
    Eur,
    /// US dollar, without a conversion to euro.
    Usd,
    /// Another reported currency code without an assumed minor-unit scale.
    Other(String),
}

/// An exact nonnegative monetary amount in the currency's minor units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinorUnits(u64);

impl MinorUnits {
    /// Wraps a nonnegative minor-unit count without rescaling it.
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw minor-unit count.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Multiplies the amount by a quantity, returning no amount on overflow.
    pub fn checked_mul(self, quantity: u32) -> Option<Self> {
        self.0.checked_mul(u64::from(quantity)).map(Self)
    }
}

/// An exact amount paired with its currency.
#[derive(Debug, PartialEq, Eq)]
pub struct Money {
    /// Currency in which the amount is denominated.
    pub currency: Currency,
    /// Exact count of the currency’s minor units.
    pub minor_units: MinorUnits,
}

/// Independently reported facts about a rate and any spending floor.
#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    /// Amount for the given pricing basis.
    pub amount: Evidence<MinorUnits>,
    /// Currency reported for the rate and spending floor.
    pub currency: Evidence<Currency>,
    /// Unit to which the amount applies.
    pub basis: Evidence<PriceBasis>,
    /// Whether the amount is a quote, lower bound or estimate.
    pub kind: PriceKind,
    /// Source evidence for tax inclusion.
    pub vat: Evidence<VatStatus>,
    /// Spending floor in the reported currency, if supplied.
    pub minimum_spend: Evidence<MinorUnits>,
}

/// Unit to which a quoted amount applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceBasis {
    /// Rate for one attendee.
    PerPerson,
    /// Rate for one day.
    PerDay,
    /// Rate for the stated part-day unit.
    PerPartDay,
}

/// Strength and interpretation of the monetary evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceKind {
    /// Stated rate or quote, possibly with unresolved components.
    Quoted,
    /// Supported lower bound on the cost.
    KnownMinimum,
    /// Indicative amount without a firm quote.
    Estimate,
}

/// Whether the quoted amount already includes VAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VatStatus {
    /// VAT is included in the amount.
    Included,
    /// VAT is additional to the amount.
    Excluded,
}

/// Catering inclusion reported independently of a price.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CateringStatus {
    /// Catering is listed as included.
    Included,
    /// Catering is listed as excluded.
    Excluded,
    /// The source describes only partial inclusion.
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RoomAccessRestriction {
    Plenary,
    Breakouts,
    SomePlenarySpace,
    SomeBreakouts,
}

/// Wheelchair access reported by a listing.
#[derive(Debug, PartialEq, Eq)]
pub enum Accessibility {
    /// A positive listing claim, subject to booking verification.
    ReportedAccessible,
    /// The source explicitly reports unavailable access.
    Inaccessible,
    /// The source qualifies access. The text describes the limits.
    Partial(String),
}

/// Calendar evidence for a single listing.
#[derive(Debug, PartialEq, Eq)]
pub struct Availability {
    /// More specific exceptions precede the general rule for this listing.
    pub rules: Vec<AvailabilityRule>,
    /// Minimum booking lead time in days, when stated.
    pub minimum_notice_days: Option<u32>,
    /// Weekday preferred by the venue, when stated.
    pub preferred_weekday: Option<Weekday>,
}

/// Availability claim over a date range and optional weekdays.
#[derive(Debug, PartialEq, Eq)]
pub struct AvailabilityRule {
    /// Inclusive range to which the claim applies.
    pub dates: DateRange,
    /// No weekday restriction when absent.
    pub weekdays: Option<Vec<Weekday>>,
    /// Availability reported for the matching days.
    pub status: DateStatus,
}

/// Positive or negative availability claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateStatus {
    /// The source reports availability, subject to booking.
    Available,
    /// The source reports a closure or exclusion.
    Unavailable,
}

/// Inclusive, non-reversed range of calendar dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

impl DateRange {
    /// Creates an inclusive range if the start is no later than the end.
    #[must_use]
    pub fn new(start: NaiveDate, end: NaiveDate) -> Option<Self> {
        if start > end {
            return None;
        }

        Some(Self { start, end })
    }

    /// Returns the first day in the range.
    #[must_use]
    pub fn start(self) -> NaiveDate {
        self.start
    }

    /// Returns the last day in the range.
    #[must_use]
    pub fn end(self) -> NaiveDate {
        self.end
    }
}

/// Station, walking and transit evidence, recorded independently.
#[derive(Debug, PartialEq, Eq)]
pub struct StationAccess {
    /// Nearest station claim, when supported.
    pub station: Evidence<String>,
    /// Walking time evidence, independent of the station name.
    pub walk: Evidence<WalkTime>,
    /// Original text describing transport alternatives or limitations.
    pub public_transport_details: Option<String>,
}

/// Published walking time to the station.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalkTime {
    /// Walking duration in whole minutes.
    pub minutes: u32,
    /// Whether the source qualified the duration as approximate.
    pub approximate: bool,
}

/// Strength or limitation of an amenity claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    /// Explicitly adverse evidence.
    Unacceptable,
    /// Limited quality or coverage.
    Partial,
    /// Positive evidence for preliminary assessment.
    Acceptable,
    /// Detailed positive evidence about quality or facilities.
    Strong,
}

/// Preliminary outcome under the event requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    /// Evidence satisfies the applicable hard requirements.
    Eligible,
    /// Hard requirement evidence remains unresolved.
    RequiresConfirmation,
    /// At least one hard requirement fails for the scenario.
    Rejected,
}

/// Business dimension that a reason or warning relates to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Criterion {
    /// Venue identity or source conflict.
    Identity,
    /// Seated plenary fit.
    Seating,
    /// Usable breakout rooms.
    Breakouts,
    /// Cost against the working ceiling.
    Budget,
    /// Wheelchair access.
    Accessibility,
    /// Event dates and booking restrictions.
    Availability,
    /// Station and public transport access.
    Transport,
    /// Natural light in the event spaces.
    Daylight,
    /// Network quality and coverage.
    Wifi,
}

/// Human-readable explanation tied to a criterion and its sources.
#[derive(Debug, PartialEq, Eq)]
pub struct Finding {
    /// Business dimension explained by this finding.
    pub criterion: Criterion,
    /// Explanation suitable for a reader of the assessment.
    pub message: String,
    /// Records or fields supporting the explanation.
    pub sources: Vec<SourceRef>,
}

/// Eligibility and its supporting reasons for one scenario.
#[derive(Debug, PartialEq, Eq)]
pub struct Assessment {
    /// Outcome for this scenario.
    pub eligibility: Eligibility,
    /// Findings that explain the outcome.
    pub reasons: Vec<Finding>,
    /// Caveats to retain alongside the outcome.
    pub warnings: Vec<Finding>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn venue_id_rejects_blank_without_rewriting_nonblank_ids() {
        assert_eq!(VenueId::new(String::new()), None);
        assert_eq!(VenueId::new(" \t".to_owned()), None);
        let id = VenueId::new("local-A".to_owned()).unwrap();
        assert_eq!(id.as_str(), "local-A");
    }

    #[test]
    fn seating_preserves_layout_and_inclusive_bounds() {
        let lower = NonZeroU32::new(80).unwrap();
        let upper = NonZeroU32::new(120).unwrap();
        let seating = SeatedCapacity::new(SeatingLayout::Cabaret, lower, upper).unwrap();

        assert_eq!(seating.layout, SeatingLayout::Cabaret);
        assert_eq!(seating.bounds().minimum(), 80);
        assert_eq!(seating.bounds().maximum(), 120);
        assert!(SeatedCapacity::new(SeatingLayout::Cabaret, upper, lower).is_none());
    }

    #[test]
    fn breakout_count_allows_explicit_zero_and_rejects_reversed_bounds() {
        let no_rooms = CountRange::new(0, 0).unwrap();

        assert_eq!(no_rooms.minimum(), 0);
        assert_eq!(no_rooms.maximum(), 0);
        assert!(CountRange::new(6, 4).is_none());
        assert_ne!(Evidence::Known(no_rooms), Evidence::Missing);
    }

    #[test]
    fn catering_can_be_known_when_the_price_is_on_request() {
        let venue = Venue {
            id: VenueId::new("sample".to_owned()).unwrap(),
            name: "Sample venue".to_owned(),
            city: None,
            address: Evidence::Missing,
            sources: Vec::new(),
            capacity: Evidence::Missing,
            breakouts: Evidence::Missing,
            price: Evidence::Unknown,
            catering: Evidence::Known(CateringStatus::Included),
            additional_charges: AdditionalCharges::default(),
            accessibility: Evidence::Missing,
            room_access_restrictions: Vec::new(),
            availability: Evidence::Missing,
            station_access: StationAccess {
                station: Evidence::Missing,
                walk: Evidence::Missing,
                public_transport_details: None,
            },
            daylight: Evidence::Missing,
            daylight_qualification: None,
            wifi: Evidence::Missing,
            wifi_qualification: None,
            notes: None,
        };

        assert_eq!(venue.price, Evidence::Unknown);
        assert_eq!(venue.catering, Evidence::Known(CateringStatus::Included));
    }

    #[test]
    fn invalid_minimum_spend_is_distinct_from_an_absent_floor() {
        let price = Price {
            amount: Evidence::Known(MinorUnits::new(9_500)),
            currency: Evidence::Known(Currency::Eur),
            basis: Evidence::Known(PriceBasis::PerPerson),
            kind: PriceKind::Quoted,
            vat: Evidence::Known(VatStatus::Excluded),
            minimum_spend: Evidence::Invalid {
                raw: "-100".to_owned(),
            },
        };

        assert_ne!(price.minimum_spend, Evidence::Missing);
        assert_eq!(
            price.minimum_spend,
            Evidence::Invalid {
                raw: "-100".to_owned()
            }
        );
    }

    #[test]
    fn price_multiplication_preserves_minor_units() {
        let amount = MinorUnits::new(7_250);

        assert_eq!(amount.checked_mul(80).map(MinorUnits::get), Some(580_000));
        assert_eq!(amount.checked_mul(120).map(MinorUnits::get), Some(870_000));
    }

    #[test]
    fn price_multiplication_reports_overflow() {
        assert_eq!(MinorUnits::new(u64::MAX).checked_mul(2), None);
    }

    #[test]
    fn date_range_supports_single_days_and_rejects_reversed_dates() {
        let start = NaiveDate::from_ymd_opt(2027, 3, 11).unwrap();
        let end = NaiveDate::from_ymd_opt(2027, 3, 18).unwrap();
        let single_day = DateRange::new(start, start).unwrap();
        let multiple_days = DateRange::new(start, end).unwrap();

        assert_eq!(single_day.start(), single_day.end());
        assert_eq!(multiple_days.start(), start);
        assert_eq!(multiple_days.end(), end);
        assert!(DateRange::new(end, start).is_none());
    }
}
