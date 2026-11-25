//! Venue evidence and event inputs shared by ingestion and assessment.

use std::num::NonZeroU32;

use chrono::{NaiveDate, Weekday};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VenueId(String);

impl VenueId {
    pub fn new(value: String) -> Option<Self> {
        if value.trim().is_empty() {
            return None;
        }

        Some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SourceRef {
    pub record_index: usize,
    pub venue_id: Option<VenueId>,
    pub field: Option<String>,
}

/// The state of a listing claim before applying any shortlist rules.
#[derive(Debug, PartialEq, Eq)]
pub enum Evidence<T> {
    Known(T),
    /// The source omitted the field.
    Missing,
    /// The source supplied no usable claim, such as a null or on-request price.
    Unknown,
    /// The source made a claim that cannot be represented as valid evidence.
    Invalid {
        raw: String,
    },
    /// Trace the disagreeing claims through their source records.
    Conflicting,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EventRequest {
    pub seated_attendees: NonZeroU32,
    pub expansion_attendees: Option<NonZeroU32>,
    pub minimum_breakouts: u32,
    pub budget: Money,
    pub candidate_dates: Vec<NaiveDate>,
    pub preferred_weekday: Option<Weekday>,
    pub booking_reference_date: Option<NaiveDate>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Venue {
    pub id: VenueId,
    pub name: String,
    pub city: Option<String>,
    pub sources: Vec<SourceRef>,
    pub capacity: Evidence<Capacity>,
    pub breakouts: Evidence<Breakouts>,
    pub price: Evidence<Price>,
    pub catering: Evidence<CateringStatus>,
    pub accessibility: Evidence<Accessibility>,
    pub availability: Evidence<Availability>,
    pub station_access: Evidence<StationAccess>,
    pub daylight: Evidence<Quality>,
    pub wifi: Evidence<Quality>,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Capacity {
    pub seated: Vec<SeatedCapacity>,
    pub standing: Option<NonZeroU32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatingLayout {
    Unspecified,
    Seated,
    Theatre,
    Cabaret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeatedCapacity {
    pub layout: SeatingLayout,
    bounds: CountRange,
}

impl SeatedCapacity {
    pub fn new(layout: SeatingLayout, minimum: NonZeroU32, maximum: NonZeroU32) -> Option<Self> {
        Some(Self {
            layout,
            bounds: CountRange::new(minimum.get(), maximum.get())?,
        })
    }

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
    pub fn new(minimum: u32, maximum: u32) -> Option<Self> {
        if minimum > maximum {
            return None;
        }

        Some(Self { minimum, maximum })
    }

    pub fn minimum(self) -> u32 {
        self.minimum
    }

    pub fn maximum(self) -> u32 {
        self.maximum
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Breakouts {
    pub count: CountRange,
    pub qualification: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Currency {
    Eur,
    Usd,
    Other(String),
}

/// An exact nonnegative monetary amount in the currency's minor units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinorUnits(u64);

impl MinorUnits {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub fn checked_mul(self, quantity: u32) -> Option<Self> {
        self.0.checked_mul(u64::from(quantity)).map(Self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Money {
    pub currency: Currency,
    pub minor_units: MinorUnits,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    pub amount: Evidence<MinorUnits>,
    pub currency: Evidence<Currency>,
    pub basis: Evidence<PriceBasis>,
    pub kind: PriceKind,
    pub vat: Evidence<VatStatus>,
    pub minimum_spend: Evidence<MinorUnits>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceBasis {
    PerPerson,
    PerDay,
    PerPartDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceKind {
    Quoted,
    KnownMinimum,
    Estimate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VatStatus {
    Included,
    Excluded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CateringStatus {
    Included,
    Excluded,
    Partial,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Accessibility {
    /// A positive listing claim, subject to booking verification.
    ReportedAccessible,
    Inaccessible,
    Partial(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Availability {
    pub rules: Vec<AvailabilityRule>,
    pub minimum_notice_days: Option<u32>,
    pub preferred_weekday: Option<Weekday>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AvailabilityRule {
    pub dates: DateRange,
    /// No weekday restriction when absent.
    pub weekdays: Option<Vec<Weekday>>,
    pub status: DateStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateStatus {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

impl DateRange {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Option<Self> {
        if start > end {
            return None;
        }

        Some(Self { start, end })
    }

    pub fn start(self) -> NaiveDate {
        self.start
    }

    pub fn end(self) -> NaiveDate {
        self.end
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StationAccess {
    pub station: String,
    pub walk: Evidence<WalkTime>,
    pub public_transport_details: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalkTime {
    pub minutes: u32,
    pub approximate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Unacceptable,
    Partial,
    Acceptable,
    Strong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    Eligible,
    RequiresConfirmation,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Criterion {
    Identity,
    Seating,
    Breakouts,
    Budget,
    Accessibility,
    Availability,
    Transport,
    Daylight,
    Wifi,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Finding {
    pub criterion: Criterion,
    pub message: String,
    pub sources: Vec<SourceRef>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assessment {
    pub eligibility: Eligibility,
    pub reasons: Vec<Finding>,
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
            sources: Vec::new(),
            capacity: Evidence::Missing,
            breakouts: Evidence::Missing,
            price: Evidence::Unknown,
            catering: Evidence::Known(CateringStatus::Included),
            accessibility: Evidence::Missing,
            availability: Evidence::Missing,
            station_access: Evidence::Missing,
            daylight: Evidence::Missing,
            wifi: Evidence::Missing,
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
