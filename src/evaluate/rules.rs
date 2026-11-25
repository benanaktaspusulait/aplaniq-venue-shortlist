use chrono::{Datelike, NaiveDate};

use crate::domain::{
    Accessibility, Availability, Capacity, Criterion, DateStatus, Evidence, Quality, SeatingLayout,
    StationAccess, Venue,
};

use super::{
    RequirementAssessment,
    RequirementOutcome::{Satisfied, Uncertain, Violated},
    check,
};

pub(super) fn seating(capacity: &Evidence<Capacity>, attendees: u32) -> RequirementAssessment {
    let (outcome, reason) = match capacity {
        Evidence::Known(capacity) => {
            let supported = capacity
                .seated
                .iter()
                .find(|seats| seats.bounds().minimum() >= attendees);
            if let Some(seats) = supported {
                let reason = if seats.layout == SeatingLayout::Unspecified {
                    "The listed plenary count supports the headcount. Confirm the seated layout before booking."
                } else {
                    "A documented seated layout supports the headcount."
                };
                (Satisfied, reason)
            } else if capacity
                .seated
                .iter()
                .any(|seats| seats.bounds().maximum() >= attendees)
            {
                (
                    Uncertain,
                    "Confirm the layout and seated capacity within the reported range.",
                )
            } else {
                (
                    Violated,
                    "No listed seated layout fits the headcount. Standing capacity does not count.",
                )
            }
        }
        Evidence::Invalid { .. } => (
            Uncertain,
            "The reported capacity is invalid. Verify usable seats.",
        ),
        Evidence::Conflicting => (Uncertain, "Resolve conflicting seating evidence."),
        Evidence::Unknown | Evidence::Missing => (Uncertain, "Confirm seated plenary capacity."),
    };
    check(Criterion::Seating, outcome, reason)
}

pub(super) fn breakouts(venue: &Venue, minimum: u32) -> RequirementAssessment {
    use Criterion::Breakouts;
    if minimum == 0 {
        return check(
            Breakouts,
            Satisfied,
            "This event requires no breakout rooms.",
        );
    }
    let Evidence::Known(rooms) = &venue.breakouts else {
        return check(
            Breakouts,
            Uncertain,
            "Confirm the number of usable simultaneous breakout rooms.",
        );
    };
    if rooms.count.maximum() < minimum {
        return check(Breakouts, Violated, "Too few breakout rooms are listed.");
    }
    let notes = venue
        .notes
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let weather_dependent = notes.contains("outdoor breakout")
        && (notes.contains("weather") || notes.contains("outdoor only"));
    let known_indoor_or_unqualified = match rooms.qualification.as_deref() {
        Some(detail) if detail.eq_ignore_ascii_case("one is the lounge") => {
            rooms.count.minimum().saturating_sub(1)
        }
        Some(_) => 0,
        None => rooms.count.minimum(),
    };
    if known_indoor_or_unqualified >= minimum && !weather_dependent {
        check(
            Breakouts,
            Satisfied,
            "The listing supports enough simultaneous breakout rooms.",
        )
    } else {
        check(
            Breakouts,
            Uncertain,
            "Confirm how many listed breakout rooms are usable at the same time.",
        )
    }
}

pub(super) fn accessibility(venue: &Venue) -> RequirementAssessment {
    use Criterion::Accessibility as Rule;
    let scoped_claim = match &venue.accessibility {
        Evidence::Known(Accessibility::Partial(text)) => Some(text.as_str()),
        _ => None,
    };
    let scope = [venue.notes.as_deref(), scoped_claim]
        .into_iter()
        .flatten()
        .flat_map(|text| text.split([',', ';', '.', '\n']))
        .filter_map(room_access_restriction)
        .fold(None, |worst, next| match (worst, next) {
            (Some(Violated), _) | (_, Violated) => Some(Violated),
            _ => Some(Uncertain),
        });

    let (outcome, reason) = if scope == Some(Violated) {
        (
            Violated,
            "A required event space is explicitly inaccessible.",
        )
    } else if scope == Some(Uncertain)
        && venue.accessibility != Evidence::Known(Accessibility::Inaccessible)
    {
        (
            Uncertain,
            "Confirm which restricted rooms are required and whether accessible alternatives exist.",
        )
    } else {
        match &venue.accessibility {
            Evidence::Known(Accessibility::Inaccessible) => (
                Violated,
                "The listing explicitly reports no wheelchair access.",
            ),
            Evidence::Known(Accessibility::ReportedAccessible) => (
                Satisfied,
                "The listing reports wheelchair access. Verify step-free routes, rooms and facilities before booking.",
            ),
            Evidence::Known(Accessibility::Partial(_)) => (
                Uncertain,
                "Confirm step-free access to the plenary and required breakout rooms.",
            ),
            Evidence::Missing => (
                Uncertain,
                "No wheelchair-access listing field was supplied.",
            ),
            Evidence::Unknown | Evidence::Invalid { .. } | Evidence::Conflicting => (
                Uncertain,
                "Confirm wheelchair access and resolve the listing's unclear access evidence.",
            ),
        }
    };
    check(Rule, outcome, reason)
}

fn room_access_restriction(clause: &str) -> Option<super::RequirementOutcome> {
    let clause = clause.to_ascii_lowercase();
    let event_space = clause.contains("plenary") || clause.contains("breakout");
    let restriction = [
        "stairs only",
        "stairs-only",
        "only accessible by stairs",
        "inaccessible",
        "not wheelchair accessible",
        "no step-free access",
    ]
    .iter()
    .any(|claim| clause.contains(claim));
    if !event_space || !restriction || clause.contains("not inaccessible") {
        return None;
    }
    let limited_scope = [
        "some ",
        "one ",
        "optional ",
        "except ",
        "spare ",
        "alternative ",
    ]
    .iter()
    .any(|qualifier| clause.contains(qualifier));
    Some(if limited_scope { Uncertain } else { Violated })
}

pub(super) fn availability(
    availability: &Evidence<Availability>,
    date: NaiveDate,
    reference: Option<NaiveDate>,
) -> RequirementAssessment {
    use Criterion::Availability as Rule;
    if reference.is_some_and(|reference| reference > date) {
        return check(
            Rule,
            Violated,
            "The event date precedes the supplied booking reference.",
        );
    }
    let Evidence::Known(schedule) = availability else {
        return check(
            Rule,
            Uncertain,
            "Confirm availability on the requested date.",
        );
    };
    let mut matching_rules = schedule.rules.iter().filter(|rule| {
        (rule.dates.start()..=rule.dates.end()).contains(&date)
            && rule
                .weekdays
                .as_ref()
                .is_none_or(|days| days.contains(&date.weekday()))
    });
    let Some(first) = matching_rules.next() else {
        return check(
            Rule,
            Uncertain,
            "The listing does not establish whether this date is available.",
        );
    };
    let specificity = |rule: &crate::domain::AvailabilityRule| {
        (
            (rule.dates.end() - rule.dates.start()).num_days(),
            rule.weekdays.as_ref().map_or(7, Vec::len),
        )
    };
    let (mut best, mut status) = (specificity(first), Some(first.status));
    for rule in matching_rules {
        let scope = specificity(rule);
        if scope < best {
            (best, status) = (scope, Some(rule.status));
        } else if scope == best && status != Some(rule.status) {
            status = None;
        }
    }
    let (outcome, reason) = match status {
        Some(DateStatus::Unavailable) => (Violated, "The listing explicitly excludes this date."),
        None => (
            Uncertain,
            "Resolve conflicting availability claims for this date.",
        ),
        Some(DateStatus::Available) => match (schedule.minimum_notice_days, reference) {
            (Some(_), None) => (
                Uncertain,
                "Supply a booking reference date to check the required lead time.",
            ),
            (Some(notice), Some(reference))
                if (date - reference).num_days() < i64::from(notice) =>
            {
                (
                    Violated,
                    "The booking does not meet the stated minimum notice.",
                )
            }
            _ => (
                Satisfied,
                "The listing supports this date. Check the actual event slot before booking.",
            ),
        },
    };
    check(Rule, outcome, reason)
}

pub(super) fn transport(access: &StationAccess) -> RequirementAssessment {
    let detail = access
        .public_transport_details
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let no_public_route = [
        "no public transport",
        "no public transit",
        "taxi only",
        "taxi-only",
    ]
    .iter()
    .any(|claim| detail.contains(claim));
    let rail_or_bus = [
        "by train",
        "by bus",
        "by tram",
        "tram stop",
        "bus stop",
        "bus from",
        "tram from",
    ]
    .iter()
    .any(|claim| detail.contains(claim));
    let extra_cost_route = detail.contains("shuttle") && detail.contains("on request");
    let origin_concern = (detail.contains("eindhoven") || detail.contains("amsterdam"))
        && (detail.contains("far for")
            || detail.contains("long journey")
            || detail.contains("but ~"));
    let local_walk = matches!(
        (&access.station, &access.walk),
        (Evidence::Known(station), Evidence::Known(_))
            if station.to_ascii_lowercase().starts_with("utrecht ")
    );
    let (outcome, reason) = if no_public_route && !rail_or_bus && !extra_cost_route {
        (
            Violated,
            "The listing explicitly rules out practical public transport.",
        )
    } else if origin_concern || no_public_route || (extra_cost_route && !local_walk) {
        (
            Uncertain,
            "Confirm a practical rail or connecting route for attendees.",
        )
    } else if local_walk || rail_or_bus {
        (
            Satisfied,
            "The listing documents a walk from a Utrecht station or a connecting public transport route.",
        )
    } else {
        (
            Uncertain,
            "Confirm a practical rail or connecting route for attendees.",
        )
    };
    check(Criterion::Transport, outcome, reason)
}

pub(super) fn quality(
    criterion: Criterion,
    evidence: &Evidence<Quality>,
    qualifier: Option<&str>,
) -> RequirementAssessment {
    let qualifier = qualifier.unwrap_or_default().to_ascii_lowercase();
    let explicitly_adverse = match criterion {
        Criterion::Daylight => [
            "plenary has no windows",
            "no windows in plenary",
            "windowless plenary",
            "plenary is windowless",
        ]
        .iter()
        .any(|claim| qualifier.contains(claim)),
        Criterion::Wifi => [
            "poor wifi in plenary",
            "unreliable wifi in plenary",
            "plenary wifi is poor",
        ]
        .iter()
        .any(|claim| qualifier.contains(claim)),
        _ => false,
    };
    let (outcome, reason) = match evidence {
        Evidence::Known(Quality::Unacceptable) => (
            Violated,
            "The listing explicitly reports unacceptable quality.",
        ),
        Evidence::Known(Quality::Partial) if explicitly_adverse => (
            Violated,
            "The qualification explicitly reports unacceptable quality in the required plenary.",
        ),
        Evidence::Known(Quality::Acceptable | Quality::Strong) => (
            Satisfied,
            "The listing supplies positive preliminary quality evidence.",
        ),
        Evidence::Known(Quality::Partial) => (
            Uncertain,
            "Confirm the quality and scope in the required event spaces.",
        ),
        _ => (Uncertain, "Confirm quality in the required event spaces."),
    };
    check(criterion, outcome, reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SeatedCapacity, SeatingLayout};
    use std::num::NonZeroU32;

    #[test]
    fn seating_uses_both_range_bounds_and_never_uses_standing() {
        let capacity = Evidence::Known(Capacity {
            seated: vec![
                SeatedCapacity::new(
                    SeatingLayout::Theatre,
                    NonZeroU32::new(80).unwrap(),
                    NonZeroU32::new(120).unwrap(),
                )
                .unwrap(),
            ],
            standing: NonZeroU32::new(150),
        });
        assert_eq!(seating(&capacity, 80).outcome, Satisfied);
        assert_eq!(seating(&capacity, 120).outcome, Uncertain);
        assert_eq!(seating(&capacity, 121).outcome, Violated);
        assert_eq!(seating(&Evidence::Conflicting, 80).outcome, Uncertain);
    }
}
