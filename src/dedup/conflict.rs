use crate::domain::{Evidence, Price, Venue};

use super::Conflict;

/// Source field containing unresolved, differing claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictField {
    /// Seating layouts, counts or ranges, or standing capacity.
    Capacity,
    /// Breakout count, range or qualification.
    Breakouts,
    /// Amount reported by either price model.
    PriceAmount,
    /// Currency of the quote or minimum spend.
    Currency,
    /// Unit the price covers.
    PriceBasis,
    /// Whether the source reports a firm quote, minimum or estimate.
    PriceKind,
    /// VAT inclusion.
    Vat,
    /// Reported spending floor.
    MinimumSpend,
    /// Catering inclusion.
    Catering,
    /// Wheelchair access.
    Accessibility,
    /// Calendar or booking restrictions.
    Availability,
    /// Nearest station.
    Station,
    /// Walk duration or its stated precision.
    Walk,
    /// Transport details derived from the source notes.
    TransportDetails,
    /// Natural light for the space.
    Daylight,
    /// Network quality or coverage.
    Wifi,
    /// Different descriptive source notes, which need manual interpretation.
    Notes,
}

pub(super) fn record<'a>(left: &'a Venue, right: &'a Venue, conflicts: &mut Vec<Conflict<'a>>) {
    use ConflictField as Field;

    let mut record = |field, differs| {
        if differs {
            conflicts.push(Conflict { field, left, right });
        }
    };

    record(
        Field::Capacity,
        known_differs(&left.capacity, &right.capacity),
    );
    record(
        Field::Breakouts,
        known_differs(&left.breakouts, &right.breakouts),
    );
    if let (Evidence::Known(left), Evidence::Known(right)) = (&left.price, &right.price) {
        record_price(left, right, &mut record);
    }
    record(
        Field::Catering,
        known_differs(&left.catering, &right.catering),
    );
    record(
        Field::Accessibility,
        known_differs(&left.accessibility, &right.accessibility),
    );
    record(
        Field::Availability,
        known_differs(&left.availability, &right.availability),
    );
    record(
        Field::Station,
        known_differs(&left.station_access.station, &right.station_access.station),
    );
    record(
        Field::Walk,
        known_differs(&left.station_access.walk, &right.station_access.walk),
    );
    record(
        Field::TransportDetails,
        supplied_differs(
            left.station_access.public_transport_details.as_ref(),
            right.station_access.public_transport_details.as_ref(),
        ),
    );
    record(
        Field::Daylight,
        known_differs(&left.daylight, &right.daylight)
            || supplied_differs(
                left.daylight_qualification.as_ref(),
                right.daylight_qualification.as_ref(),
            ),
    );
    record(
        Field::Wifi,
        known_differs(&left.wifi, &right.wifi)
            || supplied_differs(
                left.wifi_qualification.as_ref(),
                right.wifi_qualification.as_ref(),
            ),
    );
    record(
        Field::Notes,
        supplied_differs(left.notes.as_ref(), right.notes.as_ref()),
    );
}

fn record_price(left: &Price, right: &Price, record: &mut impl FnMut(ConflictField, bool)) {
    use ConflictField as Field;

    record(
        Field::PriceAmount,
        known_differs(&left.amount, &right.amount),
    );
    record(
        Field::Currency,
        known_differs(&left.currency, &right.currency),
    );
    record(Field::PriceBasis, known_differs(&left.basis, &right.basis));
    record(Field::PriceKind, left.kind != right.kind);
    record(Field::Vat, known_differs(&left.vat, &right.vat));
    record(
        Field::MinimumSpend,
        known_differs(&left.minimum_spend, &right.minimum_spend),
    );
}

fn known_differs<T: PartialEq>(left: &Evidence<T>, right: &Evidence<T>) -> bool {
    matches!((left, right), (Evidence::Known(left), Evidence::Known(right)) if left != right)
}

fn supplied_differs<T: PartialEq>(left: Option<&T>, right: Option<&T>) -> bool {
    matches!((left, right), (Some(left), Some(right)) if left != right)
}
