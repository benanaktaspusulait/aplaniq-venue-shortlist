use chrono::{Datelike, NaiveDate};

use crate::domain::{
    Accessibility, Availability, Capacity, Criterion, DateStatus, Evidence, Quality,
    RoomAccessRestriction, SeatingLayout, StationAccess, Venue,
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
            } else if capacity.unresolved_layouts {
                (
                    Uncertain,
                    "Confirm whether an unresolved layout supports the headcount.",
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

pub(super) fn accessibility(venue: &Venue, requires_breakouts: bool) -> RequirementAssessment {
    use Criterion::Accessibility as Rule;
    use RoomAccessRestriction::{Breakouts, Plenary, SomeBreakouts, SomePlenarySpace};
    let scope = venue
        .room_access_restrictions
        .iter()
        .filter_map(|restriction| match restriction {
            Plenary => Some(Violated),
            Breakouts if requires_breakouts => Some(Violated),
            SomePlenarySpace => Some(Uncertain),
            SomeBreakouts if requires_breakouts => Some(Uncertain),
            Breakouts | SomeBreakouts => None,
        })
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
    let taxi_only = ["taxi only", "taxi-only"]
        .iter()
        .any(|claim| detail.contains(claim));
    let no_public_route = taxi_only
        || ["no public transport", "no public transit"]
            .iter()
            .any(|claim| detail.contains(claim));
    let taxi_from_station = taxi_only
        || ["by taxi from station", "by taxi from the station"]
            .iter()
            .any(|claim| detail.contains(claim));
    let rail_or_bus = detail
        .split(['.', ';', ',', '\n'])
        .flat_map(|clause| clause.split(" but "))
        .any(affirmative_public_connection);
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
    let (outcome, reason) = if no_public_route
        && !rail_or_bus
        && !extra_cost_route
        && (!local_walk || taxi_from_station)
    {
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

fn affirmative_public_connection(clause: &str) -> bool {
    let mut start = 0;
    for (separator, conjunction) in clause.match_indices(" and ") {
        let next = separator + conjunction.len();
        let explicit_subject = clause.get(next..).is_some_and(|tail| {
            [
                "guests ",
                "attendees ",
                "visitors ",
                "people ",
                "there ",
                "it ",
            ]
            .iter()
            .any(|subject| tail.starts_with(subject))
        });
        if explicit_subject {
            if clause
                .get(start..separator)
                .is_some_and(positive_transport_assertion)
            {
                return true;
            }
            start = next;
        }
    }
    clause
        .get(start..)
        .is_some_and(positive_transport_assertion)
}

fn positive_transport_assertion(clause: &str) -> bool {
    let negated = clause.split_whitespace().any(|word| {
        matches!(word, "no" | "not" | "without" | "cannot" | "never") || word.ends_with("n't")
    });
    !negated
        && [
            "by train",
            "by bus",
            "by tram",
            "tram stop",
            "bus stop",
            "bus from",
            "tram from",
        ]
        .iter()
        .any(|claim| clause.contains(claim))
}

pub(super) fn quality(
    criterion: Criterion,
    evidence: &Evidence<Quality>,
    qualifier: Option<&str>,
) -> RequirementAssessment {
    let (outcome, reason) = match evidence {
        Evidence::Known(Quality::Unacceptable) if qualifier.is_some() => (
            Violated,
            "The qualification explicitly reports unacceptable quality in the required plenary.",
        ),
        Evidence::Known(Quality::Unacceptable) => (
            Violated,
            "The listing explicitly reports unacceptable quality.",
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
    fn conflicting_exact_dates_override_a_broad_rule_without_selecting_a_winner() {
        use crate::domain::{AvailabilityRule, DateRange};

        let begins = NaiveDate::from_ymd_opt(2027, 3, 1).unwrap();
        let ends = NaiveDate::from_ymd_opt(2027, 3, 31).unwrap();
        let day = NaiveDate::from_ymd_opt(2027, 3, 11).unwrap();
        for (include_conflict, reverse, expected) in [
            (false, false, Violated),
            (false, true, Violated),
            (true, false, Uncertain),
            (true, true, Uncertain),
        ] {
            let mut rules = vec![
                AvailabilityRule {
                    dates: DateRange::new(begins, ends).unwrap(),
                    weekdays: None,
                    status: DateStatus::Available,
                },
                AvailabilityRule {
                    dates: DateRange::new(day, day).unwrap(),
                    weekdays: None,
                    status: DateStatus::Unavailable,
                },
            ];
            if include_conflict {
                rules.push(AvailabilityRule {
                    dates: DateRange::new(day, day).unwrap(),
                    weekdays: None,
                    status: DateStatus::Available,
                });
            }
            if reverse {
                rules.reverse();
            }
            let schedule = Evidence::Known(Availability {
                rules,
                minimum_notice_days: None,
                preferred_weekday: None,
            });
            assert_eq!(availability(&schedule, day, None).outcome, expected);
        }
    }

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
            unresolved_layouts: false,
        });
        assert_eq!(seating(&capacity, 80).outcome, Satisfied);
        assert_eq!(seating(&capacity, 120).outcome, Uncertain);
        assert_eq!(seating(&capacity, 121).outcome, Violated);
        assert_eq!(seating(&Evidence::Conflicting, 80).outcome, Uncertain);
    }
}
