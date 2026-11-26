//! Eligibility checks from ingestion and deduplication through both scenarios.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use chrono::NaiveDate;
use serde_json::{Value, json};
use venue_shortlist::{
    dedup::deduplicate,
    domain::{
        Criterion,
        Eligibility::{self, Eligible, Rejected, RequiresConfirmation},
        MinorUnits,
    },
    evaluate::{
        BudgetGap, ConfirmationCause, CostEvidence, RequirementAssessment,
        RequirementOutcome::{Satisfied, Uncertain, Violated},
        ScenarioAssessment, VenueAssessment, assess_venues, partner_day_request,
    },
    ingest::{load_venues, parse_venues},
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

fn with_rows(rows: &[Value], inspect: impl FnOnce(&[VenueAssessment<'_>])) {
    let report = parse_venues(&json!({"venues": rows}).to_string(), Some(2027)).unwrap();
    let deduplication = deduplicate(&report);
    inspect(&assess_venues(&deduplication, &partner_day_request()));
}

fn rule<'a>(
    scenario: &'a ScenarioAssessment<'_>,
    criterion: Criterion,
) -> &'a RequirementAssessment {
    scenario.listings[0]
        .requirements
        .iter()
        .find(|rule| rule.criterion == criterion)
        .unwrap()
}

#[test]
fn seats_and_rooms_are_checked_at_both_boundaries() {
    for (capacity, rooms, primary, expansion) in [
        (json!(79), 3, Rejected, Rejected),
        (json!(80), 3, Eligible, Rejected),
        (json!(119), 3, Eligible, Rejected),
        (json!(120), 3, Eligible, Eligible),
        (json!("79 seated / 150 standing"), 3, Rejected, Rejected),
        (json!(120), 2, Rejected, Rejected),
        (Value::Null, 3, RequiresConfirmation, RequiresConfirmation),
        (json!("80-120"), 3, Eligible, RequiresConfirmation),
    ] {
        let mut row = venue("boundary");
        row["capacity"] = capacity;
        row["breakout_rooms"] = rooms.into();
        with_rows(&[row], |result| {
            assert_eq!(result[0].primary.eligibility, primary);
            assert_eq!(result[0].expansion.as_ref().unwrap().eligibility, expansion);
        });
    }
}

#[test]
fn budget_keeps_exact_totals_floors_and_unknowns() {
    for (amount, outcome, cost) in [
        (json!(15000), Satisfied, 1_500_000),
        (json!(15000.01), Violated, 1_500_001),
        (json!(10000), Satisfied, 1_000_000),
    ] {
        let mut row = venue("boundary");
        row["price"]["amount"] = amount;
        with_rows(&[row], |result| {
            let budget = &result[0].primary.listings[0].budget;
            assert_eq!(budget.requirement.outcome, outcome);
            assert_eq!(budget.cost, CostEvidence::Complete(MinorUnits::new(cost)));
        });
    }

    let mut row = venue("rate");
    row["price"] = json!({"type":"per_person","amount":150,"currency":"EUR","vat":"incl"});
    with_rows(&[row.clone()], |result| {
        assert_eq!(
            result[0].primary.listings[0].budget.cost,
            CostEvidence::Complete(MinorUnits::new(1_200_000))
        );
        let expansion = result[0].expansion.as_ref().unwrap();
        assert_eq!(
            expansion.listings[0].budget.cost,
            CostEvidence::Complete(MinorUnits::new(1_800_000))
        );
        assert_eq!(expansion.eligibility, Rejected);
    });

    row["price"]["minimum_spend"] = json!(15000.01);
    row["price"]["vat"] = json!("excl");
    with_rows(&[row], |result| {
        let budget = &result[0].primary.listings[0].budget;
        assert_eq!(
            budget.cost,
            CostEvidence::Minimum(MinorUnits::new(1_500_001))
        );
        assert_eq!(budget.requirement.outcome, Violated);
        assert!(budget.gaps.contains(&BudgetGap::Vat));
    });
}

#[test]
fn incomplete_and_incomparable_quotes_and_overflow_are_explicit() {
    for (path, value, gap, cost) in [
        (
            "price",
            Value::Null,
            BudgetGap::Amount,
            CostEvidence::Unknown,
        ),
        (
            "price.vat",
            json!("unknown"),
            BudgetGap::Vat,
            CostEvidence::Minimum(MinorUnits::new(1_000_000)),
        ),
        (
            "catering_included",
            json!(false),
            BudgetGap::Catering,
            CostEvidence::Minimum(MinorUnits::new(1_000_000)),
        ),
        (
            "price.currency",
            json!("USD"),
            BudgetGap::Currency,
            CostEvidence::Unknown,
        ),
        (
            "notes",
            json!("Mandatory AV fee applies."),
            BudgetGap::MandatoryFees,
            CostEvidence::Minimum(MinorUnits::new(1_000_000)),
        ),
    ] {
        let mut row = venue("quote");
        if let Some((parent, field)) = path.split_once('.') {
            row[parent][field] = value;
        } else {
            row[path] = value;
        }
        with_rows(&[row], |result| {
            let budget = &result[0].primary.listings[0].budget;
            assert_eq!(budget.requirement.outcome, Uncertain);
            assert!(budget.gaps.contains(&gap));
            assert_eq!(budget.cost, cost);
        });
    }
    let mut row = venue("overflow");
    row["price"] =
        json!({"type":"per_person","amount":"184467440737095516.15","currency":"EUR","vat":"incl"});
    with_rows(&[row], |result| {
        let budget = &result[0].primary.listings[0].budget;
        assert_eq!(budget.requirement.outcome, Violated);
        assert!(budget.gaps.contains(&BudgetGap::Overflow));
    });
    let mut row = venue("partday");
    row["price"] = json!("€4.200 per dagdeel ex. BTW");
    with_rows(&[row], |result| {
        let budget = &result[0].primary.listings[0].budget;
        assert_eq!(
            budget.cost,
            CostEvidence::Incomplete(MinorUnits::new(420_000))
        );
        assert!(budget.gaps.contains(&BudgetGap::Basis));
    });
}

#[test]
fn accessibility_needs_room_scope_and_keeps_explicit_failures() {
    for (access, notes, outcome) in [
        (json!(true), "", Satisfied),
        (json!(false), "", Violated),
        (json!("ground floor only"), "", Uncertain),
        (json!("unknown"), "", Uncertain),
        (Value::Null, "", Uncertain),
        (
            json!(true),
            "Plenary on ground floor, breakouts on 1st floor (stairs only).",
            Violated,
        ),
        (
            json!(true),
            "Some breakout rooms are stairs-only.",
            Uncertain,
        ),
        (json!("breakout rooms are stairs-only"), "", Violated),
        (
            json!("yes, except all breakout rooms are stairs-only"),
            "",
            Violated,
        ),
        (json!("yes, except plenary is stairs-only"), "", Violated),
        (
            json!("yes, except some breakout rooms are stairs-only"),
            "",
            Uncertain,
        ),
    ] {
        let mut row = venue("access");
        row["wheelchair_accessible"] = access;
        row["notes"] = notes.into();
        with_rows(&[row], |result| {
            assert_eq!(
                rule(&result[0].primary, Criterion::Accessibility).outcome,
                outcome
            );
        });
    }
}

#[test]
fn availability_distinguishes_exclusions_from_unconfirmed_days() {
    for (source, expected, overall) in [
        ("Thu 11 March", [Satisfied, Uncertain, Uncertain], Eligible),
        (
            "March 2027: Thursdays only",
            [Satisfied, Violated, Satisfied],
            Eligible,
        ),
        (
            "Thursdays only March 2027",
            [Satisfied, Violated, Satisfied],
            Eligible,
        ),
        (
            "March 2027: Tuesday only",
            [Violated, Satisfied, Violated],
            Eligible,
        ),
        (
            "Tue only March 2027",
            [Violated, Satisfied, Violated],
            Eligible,
        ),
        (
            "March: Thursdays only, except 18 March",
            [Satisfied, Violated, Violated],
            Eligible,
        ),
        ("fully booked Q1 2027", [Violated; 3], Rejected),
        (
            "March: Thursdays only, except 11 and 18 March",
            [Violated; 3],
            Rejected,
        ),
        (
            "March 2027: fully booked except 9 March",
            [Violated; 3],
            Rejected,
        ),
        ("Tue 9 March", [Uncertain; 3], RequiresConfirmation),
        (
            "March 2027: fully booked except 11 March",
            [Satisfied, Violated, Violated],
            Eligible,
        ),
        ("unknown", [Uncertain; 3], RequiresConfirmation),
        (
            "Mon-Fri in March, book 6 weeks ahead",
            [Uncertain; 3],
            RequiresConfirmation,
        ),
    ] {
        let mut row = venue("dates");
        row["availability"] = json!(source);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            let observed = scenario.listings[0]
                .dates
                .iter()
                .map(|day| day.availability.outcome)
                .collect::<Vec<_>>();
            assert_eq!(observed, expected, "{source}");
            assert_eq!(scenario.eligibility, overall, "{source}");
            if source == "unknown" {
                assert_eq!(
                    scenario.dates[0].confirmation,
                    Some(ConfirmationCause::ListingEvidence)
                );
            }
        });
    }
    let mut row = venue("notice");
    row["availability"] = json!("Mon-Fri in March, book 6 weeks ahead");
    let report = parse_venues(&json!({"venues": [row]}).to_string(), Some(2027)).unwrap();
    let deduplication = deduplicate(&report);
    let mut request = partner_day_request();
    request.booking_reference_date = NaiveDate::from_ymd_opt(2027, 2, 4);
    let result = assess_venues(&deduplication, &request);
    assert_eq!(
        result[0].primary.listings[0]
            .dates
            .iter()
            .map(|day| day.availability.outcome)
            .collect::<Vec<_>>(),
        [Violated, Violated, Satisfied]
    );
    request.candidate_dates.reverse();
    assert_eq!(assess_venues(&deduplication, &request), result);
}

#[test]
fn quality_and_transport_keep_preferences_and_hard_failures_separate() {
    let mut passing = venue("walk");
    passing["walk_minutes"] = json!(22);
    let mut bad_quality = venue("quality");
    bad_quality["wifi"] = json!("poor");
    bad_quality["daylight"] = json!("mixed — plenary has no windows");
    bad_quality["capacity"] = Value::Null;
    let mut unresolved = venue("rail");
    unresolved["nearest_station"] = json!("Central");
    unresolved["wifi"] = Value::Null;
    unresolved["daylight"] = json!("mixed — casemates have no windows");
    let mut taxi = venue("taxi");
    taxi["notes"] = json!("20 min by taxi from station, no public transport.");
    with_rows(&[passing, bad_quality, unresolved, taxi], |result| {
        for (entry, overall, quality, wifi, transport) in [
            (&result[0], Rejected, Violated, Violated, Satisfied),
            (
                &result[1],
                RequiresConfirmation,
                Uncertain,
                Uncertain,
                Uncertain,
            ),
            (&result[2], Rejected, Satisfied, Satisfied, Violated),
            (&result[3], Eligible, Satisfied, Satisfied, Satisfied),
        ] {
            assert_eq!(entry.primary.eligibility, overall);
            assert_eq!(rule(&entry.primary, Criterion::Daylight).outcome, quality);
            assert_eq!(rule(&entry.primary, Criterion::Wifi).outcome, wifi);
            assert_eq!(
                rule(&entry.primary, Criterion::Transport).outcome,
                transport
            );
            assert_eq!(entry.primary.listings[0].requirements.len(), 7);
        }
        assert_eq!(
            rule(&result[0].primary, Criterion::Seating).outcome,
            Uncertain
        );
    });
}

#[test]
fn duplicate_configurations_never_lend_each_other_passing_fields_or_dates() {
    let mut left = venue("same");
    left["availability"] = json!("March 2027: fully booked except 11 March");
    left["wheelchair_accessible"] = json!(false);
    let mut right = venue("same");
    right["availability"] = json!("March 2027: fully booked except 16 March");
    right["capacity"] = json!(79);
    with_rows(&[left.clone(), right.clone()], |result| {
        let primary = &result[0].primary;
        assert_eq!(primary.listings.len(), 2);
        assert_eq!(primary.eligibility, Rejected);
        assert_eq!(primary.listings[0].requirements[3].outcome, Violated);
        assert_eq!(primary.listings[1].requirements[0].outcome, Violated);
        assert_eq!(result[0].entity.conflicts().len(), 3);
    });

    left["wheelchair_accessible"] = json!(true);
    right["capacity"] = json!(120);
    with_rows(&[left, right], |result| {
        assert_eq!(result[0].primary.eligibility, RequiresConfirmation);
        let dates = &result[0].primary.dates;
        assert_eq!(
            dates[0].confirmation,
            Some(ConfirmationCause::SourceDisagreement)
        );
        assert_eq!(
            dates.iter().map(|day| day.eligibility).collect::<Vec<_>>(),
            [RequiresConfirmation, RequiresConfirmation, Rejected,]
        );
    });
}

#[test]
fn links_potential_duplicates_without_transferring_adverse_evidence() {
    let mut left = venue("left");
    let mut right = venue("right");
    let mut separate = venue("west");
    left["name"] = json!("Assembly Hall");
    right["name"] = json!("Assembly Hall");
    separate["name"] = json!("Assembly Hall - West");
    with_rows(&[left.clone(), right.clone(), separate.clone()], |result| {
        assert_eq!(result[0].primary.eligibility, RequiresConfirmation);
        assert_eq!(result[1].primary.eligibility, RequiresConfirmation);
        assert_eq!(result[2].primary.eligibility, Eligible);
    });
    left["wifi"] = json!("poor");
    with_rows(&[separate, right, left], |result| {
        assert_eq!(result[0].primary.eligibility, Rejected);
        assert_eq!(result[1].primary.eligibility, RequiresConfirmation);
        assert_eq!(result[1].potential_duplicates.len(), 1);
        assert_eq!(
            result[1].primary.dates[0].confirmation,
            Some(ConfirmationCause::PotentialDuplicate)
        );
        assert_eq!(rule(&result[1].primary, Criterion::Wifi).outcome, Satisfied);
        assert_eq!(result[2].primary.eligibility, Eligible);
        let indices = result
            .iter()
            .flat_map(|entry| entry.entity.listings())
            .map(|listing| listing.sources[0].record_index)
            .collect::<BTreeSet<_>>();
        assert_eq!(indices, BTreeSet::from([0, 1, 2]));
    });
}

#[test]
fn original_export_and_synthetic_stadstuin_have_the_expected_80_and_120_verdicts() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json");
    let report = load_venues(path, Some(2027)).unwrap();
    let deduplication = deduplicate(&report);
    let results = assess_venues(&deduplication, &partner_day_request());
    assert_eq!(results.len(), 30);
    assert_eq!(deduplication.normalised_record_count, 30);
    assert_eq!(deduplication.potential_duplicates.len(), 3);
    let garden = results
        .iter()
        .find(|entry| entry.entity.listings()[0].id.as_str() == "v019")
        .unwrap();
    assert_garden(garden);
    let mut synthetic = venue("synthetic");
    synthetic["capacity"] = json!(85);
    synthetic["price"] = json!({"type":"per_person","amount":79,"currency":"EUR","vat":"incl"});
    synthetic["daylight"] = json!("yes, greenhouse");
    with_rows(&[synthetic], |result| assert_garden(&result[0]));
    print_dataset_summary(&results);
}

fn assert_garden(entry: &VenueAssessment<'_>) {
    let expansion = entry.expansion.as_ref().unwrap();
    assert_eq!(entry.primary.eligibility, Eligible);
    assert_eq!(
        entry.primary.listings[0].budget.cost,
        CostEvidence::Complete(MinorUnits::new(632_000))
    );
    assert_eq!(
        expansion.listings[0].budget.cost,
        CostEvidence::Complete(MinorUnits::new(948_000))
    );
    assert_eq!(rule(expansion, Criterion::Seating).outcome, Violated);
    assert_eq!(expansion.eligibility, Rejected);
}

fn print_dataset_summary(results: &[VenueAssessment<'_>]) {
    let mut totals = [[0; 3]; 2];
    let mut rejection = [BTreeMap::new(), BTreeMap::new()];
    let mut confirmation = [BTreeMap::new(), BTreeMap::new()];
    for venue in results {
        let listing = venue.entity.listings()[0];
        let mut cells = Vec::new();
        for (index, scenario) in [&venue.primary, venue.expansion.as_ref().unwrap()]
            .into_iter()
            .enumerate()
        {
            let reasons = summary_reasons(scenario, !venue.potential_duplicates.is_empty());
            let bucket = match scenario.eligibility {
                Eligible => 0,
                RequiresConfirmation => 1,
                Rejected => 2,
            };
            totals[index][bucket] += 1;
            let bucket = match scenario.eligibility {
                Eligible => None,
                RequiresConfirmation => Some(&mut confirmation[index]),
                Rejected => Some(&mut rejection[index]),
            };
            if let Some(bucket) = bucket {
                for key in &reasons {
                    *bucket.entry(key.clone()).or_insert(0) += 1;
                }
            }
            cells.push(format!(
                "{:?} • {}",
                scenario.eligibility,
                reasons.join(", ")
            ));
        }
        println!(
            "| {} | {} | {} | {} |",
            listing.id.as_str(),
            listing.name,
            cells[0],
            cells[1]
        );
    }
    println!("Totals (eligible, conditional, rejected): {totals:?}");
    println!("Rejection distribution: {rejection:?}");
    println!("Confirmation distribution: {confirmation:?}");
}

fn summary_reasons(scenario: &ScenarioAssessment<'_>, potential: bool) -> Vec<String> {
    let outcome = match scenario.eligibility {
        Eligibility::Eligible => return vec![],
        Eligibility::RequiresConfirmation => Uncertain,
        Eligibility::Rejected => Violated,
    };
    let mut reasons = BTreeSet::new();
    for listing in &scenario.listings {
        for check in listing
            .requirements
            .iter()
            .filter(|rule| rule.outcome == outcome)
        {
            reasons.insert(format!("{:?}", check.criterion));
        }
        if !listing
            .dates
            .iter()
            .any(|day| day.availability.outcome == Satisfied)
            && listing
                .dates
                .iter()
                .any(|day| day.availability.outcome == outcome)
            && (outcome == Uncertain
                || listing
                    .dates
                    .iter()
                    .all(|day| day.availability.outcome == Violated))
        {
            reasons.insert("Availability".to_owned());
        }
    }
    if potential && outcome == Uncertain {
        reasons.insert("Identity".to_owned());
    }
    reasons.into_iter().collect()
}

#[test]
fn r3_mandatory_charges_change_the_budget_without_guessing_costs() {
    for (note, expected, cost) in [
        (
            "An additional AV rental fee is required and costs €300.",
            Rejected,
            CostEvidence::Minimum(MinorUnits::new(1_520_000)),
        ),
        (
            "An additional AV rental fee is required.",
            RequiresConfirmation,
            CostEvidence::Minimum(MinorUnits::new(1_490_000)),
        ),
        (
            "An additional AV rental fee is required and costs €100 including VAT.",
            Eligible,
            CostEvidence::Complete(MinorUnits::new(1_500_000)),
        ),
        (
            "A mandatory additional AV fee costs about €300.",
            RequiresConfirmation,
            CostEvidence::Minimum(MinorUnits::new(1_490_000)),
        ),
        (
            "There are no mandatory fees.",
            Eligible,
            CostEvidence::Complete(MinorUnits::new(1_490_000)),
        ),
        (
            "An optional AV rental fee costs €300. Catering is mandatory and costs extra.",
            RequiresConfirmation,
            CostEvidence::Minimum(MinorUnits::new(1_490_000)),
        ),
    ] {
        let mut row = venue("r3-fee");
        row["price"]["amount"] = json!(14900);
        row["notes"] = json!(note);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            assert_eq!(scenario.eligibility, expected, "{note}");
            assert_eq!(scenario.listings[0].budget.cost, cost, "{note}");
        });
    }
}

#[test]
fn r3_explicit_darkness_fails_even_with_other_suitable_evidence() {
    let mut row = venue("r3-dark");
    row["daylight"] = json!("dark");
    with_rows(&[row], |result| {
        assert_eq!(result[0].primary.eligibility, Rejected);
        assert_eq!(
            rule(&result[0].primary, Criterion::Daylight).outcome,
            Violated
        );
    });
}

#[test]
fn optional_items_do_not_hide_mandatory_charges_in_the_same_sentence() {
    for (note, expected, cost) in [
        (
            "The screen is optional but a required additional AV fee costs €300 including VAT.",
            Rejected,
            CostEvidence::Complete(MinorUnits::new(1_520_000)),
        ),
        (
            "An optional screen costs €50 and a required additional AV fee costs €300.",
            Rejected,
            CostEvidence::Minimum(MinorUnits::new(1_520_000)),
        ),
        (
            "A required additional AV fee costs €100 including VAT and an optional screen costs €50.",
            Eligible,
            CostEvidence::Complete(MinorUnits::new(1_500_000)),
        ),
        (
            "An additional AV fee is required and costs €300 including VAT and an optional screen costs €50.",
            Rejected,
            CostEvidence::Complete(MinorUnits::new(1_520_000)),
        ),
        (
            "The screen is optional but the additional AV fee is required.",
            RequiresConfirmation,
            CostEvidence::Minimum(MinorUnits::new(1_490_000)),
        ),
        (
            "The AV fee is not required and costs €300 when requested.",
            Eligible,
            CostEvidence::Complete(MinorUnits::new(1_490_000)),
        ),
    ] {
        let mut row = venue("r4-fee");
        row["price"]["amount"] = json!(14900);
        row["notes"] = json!(note);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            assert_eq!(scenario.eligibility, expected, "{note}");
            assert_eq!(scenario.listings[0].budget.cost, cost, "{note}");
        });
    }
}

#[test]
fn fee_vat_exclusions_and_conflicts_keep_the_all_in_cost_unconfirmed() {
    for note in [
        "An additional mandatory AV fee costs €100 not including VAT.",
        "An additional mandatory AV fee costs €100 including VAT, but VAT is excluded.",
        "An additional mandatory AV fee costs €100. VAT treatment is unknown.",
        "An additional mandatory AV fee costs €100 including VAT. VAT is excluded.",
    ] {
        let mut row = venue("fee-vat");
        row["price"]["amount"] = json!(14900);
        row["notes"] = json!(note);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            assert_eq!(scenario.eligibility, RequiresConfirmation, "{note}");
            let budget = &scenario.listings[0].budget;
            assert_eq!(
                budget.cost,
                CostEvidence::Minimum(MinorUnits::new(1_500_000))
            );
            assert!(budget.gaps.contains(&BudgetGap::MandatoryFees));
        });
    }
}

#[test]
fn a_negated_stop_does_not_establish_public_transport() {
    for (notes, outcome, verdict) in [
        (
            "There is no bus stop near the venue.",
            Uncertain,
            RequiresConfirmation,
        ),
        (
            "The venue is not reachable by tram.",
            Uncertain,
            RequiresConfirmation,
        ),
        (
            "No bus stop nearby, but attendees can travel by train.",
            Satisfied,
            Eligible,
        ),
        ("There is a bus stop near the venue.", Satisfied, Eligible),
        (
            "There is no dedicated parking and guests can come by train.",
            Satisfied,
            Eligible,
        ),
        (
            "Guests cannot arrive by bus and by train.",
            Uncertain,
            RequiresConfirmation,
        ),
        (
            "A tram stop is not available and visitors can travel by train.",
            Satisfied,
            Eligible,
        ),
        ("Taxi only, no public transport.", Violated, Rejected),
    ] {
        let mut row = venue("transport-claim");
        row["nearest_station"] = Value::Null;
        row["walk_minutes"] = Value::Null;
        row["notes"] = json!(notes);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            assert_eq!(
                rule(scenario, Criterion::Transport).outcome,
                outcome,
                "{notes}"
            );
            assert_eq!(scenario.eligibility, verdict, "{notes}");
        });
    }
}

#[test]
fn explicit_required_room_restrictions_survive_normalisation() {
    for (field, raw, criterion, verdict) in [
        (
            "daylight",
            "The plenary is windowless",
            Criterion::Daylight,
            Rejected,
        ),
        (
            "daylight",
            "Mixed daylight elsewhere; the plenary is windowless",
            Criterion::Daylight,
            Rejected,
        ),
        (
            "daylight",
            "Mixed daylight elsewhere; the plenary is not windowless",
            Criterion::Daylight,
            RequiresConfirmation,
        ),
        (
            "daylight",
            "The plenary is dark",
            Criterion::Daylight,
            Rejected,
        ),
        (
            "wifi",
            "4G backup only in the annex; poor WiFi in plenary",
            Criterion::Wifi,
            Rejected,
        ),
        (
            "wheelchair_accessible",
            "All breakout rooms are not wheelchair accessible",
            Criterion::Accessibility,
            Rejected,
        ),
        (
            "wheelchair_accessible",
            "Some breakout rooms are not wheelchair accessible",
            Criterion::Accessibility,
            RequiresConfirmation,
        ),
        (
            "wheelchair_accessible",
            "An optional breakout room is inaccessible",
            Criterion::Accessibility,
            RequiresConfirmation,
        ),
    ] {
        let mut row = venue("required-room");
        row[field] = json!(raw);
        with_rows(&[row], |result| {
            let scenario = &result[0].primary;
            assert_eq!(scenario.eligibility, verdict, "{raw}");
            assert_eq!(
                rule(scenario, criterion).outcome,
                if verdict == Rejected {
                    Violated
                } else {
                    Uncertain
                },
                "{raw}"
            );
        });
    }
}

#[test]
fn mixed_capacity_layouts_preserve_valid_seats_and_report_unresolved_layouts() {
    for (capacity, verdict, diagnostic_field) in [
        (
            json!({"plenary_theatre":120,"plenary_boardroom":56}),
            Eligible,
            "capacity.plenary_boardroom",
        ),
        (
            json!({"plenary_theatre":85,"plenary_cabaret":null}),
            Eligible,
            "capacity.plenary_cabaret",
        ),
        (
            json!({"plenary_theatre":60,"plenary_cabaret":null}),
            RequiresConfirmation,
            "capacity.plenary_cabaret",
        ),
        (
            json!({"plenary_boardroom":56}),
            RequiresConfirmation,
            "capacity",
        ),
    ] {
        let mut row = venue("mixed-layouts");
        row["capacity"] = capacity;
        let report = parse_venues(&json!({"venues": [row]}).to_string(), Some(2027)).unwrap();
        let groups = deduplicate(&report);
        let assessment = assess_venues(&groups, &partner_day_request());
        assert_eq!(assessment[0].primary.eligibility, verdict);
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.source.field.as_deref() == Some(diagnostic_field) })
        );
    }
}

#[test]
fn potential_duplicate_memberships_follow_global_pair_order_in_confirmed_groups() {
    let rows: Vec<_> = ["a", "b", "a", "c"]
        .into_iter()
        .map(|id| {
            let mut listing = venue(id);
            listing["name"] = json!("Shared hall");
            listing
        })
        .collect();
    let report = parse_venues(&json!({"venues": rows}).to_string(), Some(2027)).unwrap();
    let groups = deduplicate(&report);
    assert_eq!(groups.entities.len(), 3);
    assert_eq!(groups.potential_duplicates.len(), 5);
    let assessments = assess_venues(&groups, &partner_day_request());
    for entry in assessments {
        let expected: Vec<_> =
            groups
                .potential_duplicates
                .iter()
                .filter(|pair| {
                    entry.entity.listings().iter().any(|&venue| {
                        std::ptr::eq(venue, pair.left) || std::ptr::eq(venue, pair.right)
                    })
                })
                .collect();
        assert_eq!(entry.potential_duplicates, expected);
        assert_eq!(entry.primary.eligibility, RequiresConfirmation);
    }
}

#[test]
fn overflow_when_adding_a_mandatory_fee_is_a_budget_rejection() {
    let mut row = venue("fee-overflow");
    row["price"]["amount"] = json!("184467440737095516.15");
    row["notes"] = json!("An additional mandatory AV fee costs €1 including VAT.");
    with_rows(&[row], |result| {
        let primary = &result[0].primary;
        let budget = &primary.listings[0].budget;
        assert_eq!(primary.eligibility, Rejected);
        assert_eq!(budget.requirement.outcome, Violated);
        assert_eq!(budget.cost, CostEvidence::Unknown);
        assert!(budget.gaps.contains(&BudgetGap::Overflow));
    });
}
