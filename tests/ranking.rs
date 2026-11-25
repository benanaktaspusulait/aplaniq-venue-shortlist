//! Recommendation checks over the complete ingestion and eligibility pipeline.

use std::path::Path;

use chrono::NaiveDate;
use serde_json::{Value, json};
use venue_shortlist::{
    dedup::deduplicate,
    domain::{Eligibility, MinorUnits},
    evaluate::{CostEvidence, VenueAssessment, partner_day_request},
    ingest::{load_venues, parse_venues},
    rank::{QualityPreference, Shortlist, TransportPreference, rank_venues},
};

fn venue(id: &str) -> Value {
    json!({
        "id": id, "name": format!("Venue {id}"), "city": "Utrecht",
        "capacity": 120, "breakout_rooms": 3,
        "price": {"type":"per_day","amount":10000,"currency":"EUR","vat":"incl"},
        "wheelchair_accessible": true, "catering_included": true,
        "availability": "Thu 11 March", "nearest_station": "Utrecht CS", "walk_minutes": 10,
        "daylight": "yes", "wifi": "fiber"
    })
}

fn with_shortlist(rows: &[Value], inspect: impl FnOnce(&Shortlist<'_>)) {
    let report = parse_venues(&json!({"venues": rows}).to_string(), Some(2027)).unwrap();
    let groups = deduplicate(&report);
    let request = partner_day_request();
    inspect(&rank_venues(&groups, &request));
}

fn id<'a>(entry: &'a VenueAssessment<'_>) -> &'a str {
    entry.entity.listings()[0].id.as_str()
}

fn eligible_ids<'a>(report: &'a Shortlist<'_>) -> Vec<&'a str> {
    report
        .eligible
        .iter()
        .map(|entry| id(&entry.assessment))
        .collect()
}

#[test]
fn preserves_verdicts_and_accepts_an_empty_shortlist() {
    let mut cheap = venue("inaccessible");
    cheap["wheelchair_accessible"] = json!(false);
    cheap["price"]["amount"] = json!(10);
    let mut budget = venue("over-budget");
    budget["price"]["amount"] = json!(15000.01);
    budget["wifi"] = json!("fiber 1 Gbps");
    let mut unknown = venue("missing-price");
    unknown["price"] = Value::Null;
    let mut incomplete = venue("incomplete");
    incomplete["price"]["vat"] = json!("excl");
    incomplete["price"]["amount"] = json!(1);
    let mut unknown_wifi = venue("unknown-wifi");
    unknown_wifi["wifi"] = Value::Null;
    let mut unsupported = venue("unsupported");
    unsupported["price"]["currency"] = json!("USD");
    let cases = [
        cheap,
        budget,
        unknown,
        incomplete,
        unknown_wifi,
        unsupported,
    ];

    with_shortlist(&cases, |report| {
        assert_eq!(report.eligible.as_slice(), []);
        assert_eq!(
            report.conditional.iter().map(id).collect::<Vec<_>>(),
            ["incomplete", "missing-price", "unknown-wifi", "unsupported",]
        );
        assert_eq!(
            report.rejected.iter().map(id).collect::<Vec<_>>(),
            ["inaccessible", "over-budget"]
        );
        assert!(
            report
                .conditional
                .iter()
                .all(|entry| entry.primary.eligibility == Eligibility::RequiresConfirmation)
        );
        assert!(
            report
                .rejected
                .iter()
                .all(|entry| entry.primary.eligibility == Eligibility::Rejected)
        );
    });
    with_shortlist(&[venue("eligible")], |report| {
        assert_eq!(eligible_ids(report), ["eligible"]);
        assert_eq!(report.eligible[0].rank, 1);
    });
    with_shortlist(&[], |report| {
        assert!(
            report.eligible.is_empty()
                && report.conditional.is_empty()
                && report.rejected.is_empty()
        );
    });
}

#[test]
fn supported_thursday_wins_before_transport_and_uncertain_or_excluded_thursdays_do_not_count() {
    let mut thursday = venue("thu");
    thursday["walk_minutes"] = json!(25);
    let mut uncertain_thursday = venue("tue_unknown_thu");
    uncertain_thursday["availability"] = json!("Tue 16 March");
    uncertain_thursday["walk_minutes"] = json!(2);
    let mut excluded_thursday = venue("tue_excluded_thu");
    excluded_thursday["availability"] = json!("Tue only March 2027");
    excluded_thursday["walk_minutes"] = json!(3);
    with_shortlist(
        &[excluded_thursday, uncertain_thursday, thursday],
        |report| {
            assert_eq!(
                eligible_ids(report),
                ["thu", "tue_unknown_thu", "tue_excluded_thu"]
            );
            assert!(report.eligible[0].factors.preferred_weekday);
            for recommendation in &report.eligible[1..] {
                assert!(!recommendation.factors.preferred_weekday);
                assert_eq!(
                    recommendation.factors.recommended_date,
                    NaiveDate::from_ymd_opt(2027, 3, 16)
                );
            }
        },
    );
}

#[test]
fn transport_and_amenities_precede_cost() {
    let mut close = venue("close");
    close["walk_minutes"] = json!(2);
    close["price"]["amount"] = json!(15000);
    let mut strong_daylight = venue("daylight");
    strong_daylight["daylight"] = json!("yes, skylights");
    let mut strong_wifi = venue("wifi");
    strong_wifi["wifi"] = json!("fiber 1 Gbps");
    strong_wifi["price"]["amount"] = json!(100);
    let mut no_walk = venue("bus");
    no_walk["walk_minutes"] = Value::Null;
    no_walk["notes"] = json!("15 min by bus from Utrecht Centraal.");
    let mut other_station = venue("other");
    other_station["nearest_station"] = json!("Amsterdam Centraal");
    other_station["walk_minutes"] = json!(1);
    with_shortlist(
        &[other_station, no_walk, strong_wifi, strong_daylight, close],
        |report| {
            assert_eq!(eligible_ids(report), ["close", "daylight", "wifi", "bus"]);
            assert_eq!(
                report.eligible[0].factors.transport,
                TransportPreference::ShortCentraalWalk(2)
            );
            assert_eq!(
                report.eligible[0].factors.budget_headroom,
                Some(MinorUnits::new(0))
            );
            assert_eq!(
                report.eligible[1].factors.daylight,
                QualityPreference::Strong
            );
            assert_eq!(report.eligible[2].factors.wifi, QualityPreference::Strong);
            assert_eq!(
                report.eligible[3].factors.transport,
                TransportPreference::DocumentedConnection
            );
            assert_eq!(
                report.conditional.iter().map(id).collect::<Vec<_>>(),
                ["other"]
            );
        },
    );
}

#[test]
fn cost_presence_headroom_and_expansion_have_separate_keys() {
    let mut cheaper = venue("cheap");
    cheaper["price"]["amount"] = json!(9000);
    cheaper["capacity"] = json!(85);
    let both = venue("both");
    let mut uncertain = venue("unknown-expansion");
    uncertain["capacity"] = json!("80-120");
    let mut expansion_cost = venue("expansion-cost");
    expansion_cost["price"] =
        json!({"type":"per_person","amount":125,"currency":"EUR","vat":"incl"});
    let mut conflict_left = venue("different-quotes");
    let mut conflict_right = conflict_left.clone();
    conflict_left["price"]["amount"] = json!(5);
    conflict_right["price"]["amount"] = json!(20);
    with_shortlist(
        &[
            both,
            uncertain,
            expansion_cost,
            cheaper,
            conflict_left,
            conflict_right,
        ],
        |report| {
            assert_eq!(
                eligible_ids(report),
                [
                    "cheap",
                    "both",
                    "expansion-cost",
                    "unknown-expansion",
                    "different-quotes"
                ]
            );
            assert_eq!(
                report.eligible[0].factors.budget_headroom,
                Some(MinorUnits::new(600_000))
            );
            assert_eq!(
                report.eligible[0].factors.expansion,
                Some(Eligibility::Rejected)
            );
            assert_eq!(
                report.eligible[1].factors.expansion,
                Some(Eligibility::Eligible)
            );
            assert_eq!(
                report.eligible[2].factors.expansion,
                Some(Eligibility::Eligible)
            );
            assert_eq!(
                report.eligible[3].factors.expansion,
                Some(Eligibility::RequiresConfirmation)
            );
            assert_eq!(report.eligible[4].factors.budget_headroom, None);
            assert_eq!(report.eligible[4].assessment.entity.listings().len(), 2);
        },
    );
    let mut over_only_at_expansion = venue("per-head");
    over_only_at_expansion["price"] =
        json!({"type":"per_person","amount":130,"currency":"EUR","vat":"incl"});
    with_shortlist(&[over_only_at_expansion], |report| {
        assert_eq!(
            report.eligible[0].factors.expansion,
            Some(Eligibility::Rejected)
        );
        let expansion = report.eligible[0].assessment.expansion.as_ref().unwrap();
        assert_eq!(
            expansion.listings[0].budget.cost,
            CostEvidence::Complete(MinorUnits::new(1_560_000))
        );
    });
}

#[test]
fn duplicate_warnings_and_stable_ties_survive_raw_input_permutations() {
    let mut peer_a = venue("peer-a");
    let mut peer_b = venue("peer-b");
    peer_a["name"] = json!("Assembly Hall");
    peer_b["name"] = json!("Assembly Hall");
    let mut separate = venue("west");
    separate["name"] = json!("Assembly Hall - West");
    let rows = [venue("b"), peer_b, separate, venue("a"), peer_a, venue("c")];
    let orderings = [[0, 1, 2, 3, 4, 5], [5, 4, 3, 2, 1, 0], [4, 2, 1, 5, 0, 3]];
    for order in orderings {
        with_shortlist(&order.map(|index| rows[index].clone()), |report| {
            assert_eq!(eligible_ids(report), ["a", "b", "c", "west"]);
            assert_eq!(
                report.conditional.iter().map(id).collect::<Vec<_>>(),
                ["peer-a", "peer-b"]
            );
            assert!(
                report
                    .conditional
                    .iter()
                    .all(|entry| entry.potential_duplicates.len() == 1)
            );
        });
    }
}

#[test]
fn repeated_ids_with_distinct_claims_have_a_stable_secondary_key() {
    let mut cheaper = venue("reused");
    cheaper["price"]["amount"] = json!(50);
    cheaper["address"] = Value::Null;
    let mut dearer = cheaper.clone();
    dearer["price"]["amount"] = json!(200);
    for rows in [[cheaper.clone(), dearer.clone()], [dearer, cheaper]] {
        with_shortlist(&rows, |report| {
            let ordered_costs: Vec<_> = report
                .conditional
                .iter()
                .map(|entry| entry.primary.listings[0].budget.cost)
                .collect();
            assert_eq!(
                ordered_costs,
                [
                    // Equivalent identity falls back to the numeric evidence in the quote.
                    CostEvidence::Complete(MinorUnits::new(5_000)),
                    CostEvidence::Complete(MinorUnits::new(20_000)),
                ]
            );
        });
    }
}

#[test]
fn original_export_produces_the_verified_shortlist() {
    let report = load_venues(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json"),
        Some(2027),
    )
    .unwrap();
    let groups = deduplicate(&report);
    let request = partner_day_request();
    let run = || rank_venues(&groups, &request);
    let result = run();
    assert_eq!(result, run());
    assert_eq!(eligible_ids(&result), ["v019", "v028", "v021"]);
    assert_eq!(result.conditional.len(), 19);
    assert_eq!(result.rejected.len(), 8);
    for entry in &result.eligible {
        let facts = &entry.factors;
        println!(
            "{} {} | {:?} | {:?}",
            entry.rank,
            id(&entry.assessment),
            facts.recommended_date,
            facts
        );
    }
}

#[test]
fn a_new_request_reassesses_eligibility_before_ranking() {
    let input = parse_venues(&json!({"venues":[venue("fits")]}).to_string(), Some(2027)).unwrap();
    let groups = deduplicate(&input);
    let mut request = partner_day_request();
    let report = rank_venues(&groups, &request);
    assert_eq!(eligible_ids(&report), ["fits"]);
    assert_eq!(
        report.eligible[0].factors.budget_headroom,
        Some(MinorUnits::new(500_000))
    );
    request.budget.minor_units = MinorUnits::new(999_999);
    request.expansion_attendees = None;
    let report = rank_venues(&groups, &request);
    assert_eq!(report.eligible.as_slice(), []);
    assert_eq!(report.rejected.len(), 1);
    assert!(report.rejected[0].expansion.is_none());
}
