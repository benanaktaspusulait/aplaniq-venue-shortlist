use std::num::NonZeroU32;

use chrono::{NaiveDate, Weekday};

use crate::domain::{
    Accessibility, CateringStatus, Currency, DateStatus, Evidence, MinorUnits, PriceBasis, Quality,
    SeatingLayout, VatStatus, WalkTime,
};

use super::{Diagnostic, DiagnosticKind, Ingestion, InputError, load_venues, parse_venues};

const YEAR: Option<i32> = Some(2027);

fn parse_row(fields: &str) -> Ingestion {
    let input = format!(r#"{{"venues":[{{"id":"case","name":"Venue",{fields}}}]}}"#);
    parse_venues(&input, YEAR).unwrap()
}

fn known<T>(evidence: &Evidence<T>) -> &T {
    let Evidence::Known(value) = evidence else {
        panic!("expected known evidence");
    };
    value
}

fn diagnostic<'a>(
    report: &'a Ingestion,
    index: usize,
    field: &str,
    kind: DiagnosticKind,
) -> &'a Diagnostic {
    report
        .diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.source.record_index == index
                && diagnostic.source.field.as_deref() == Some(field)
                && diagnostic.kind == kind
        })
        .unwrap()
}

#[test]
fn document_errors_distinguish_read_syntax_and_envelope() {
    assert!(matches!(
        load_venues("venues.json/nonexistent.json", YEAR),
        Err(InputError::Read { .. })
    ));
    assert!(matches!(parse_venues("", YEAR), Err(InputError::Json(_))));
    let error = parse_venues("{\n invalid", YEAR).unwrap_err();
    assert!(matches!(error, InputError::Json(_)));
    assert!(error.to_string().contains("line 2"));
    for input in ["[]", "{}", r#"{"venues":null}"#, r#"{"venues":{}}"#] {
        assert!(matches!(
            parse_venues(input, YEAR),
            Err(InputError::Envelope)
        ));
    }
    let empty = parse_venues(r#"{"venues":[]}"#, YEAR).unwrap();
    assert_eq!(empty.source_record_count, 0);
    assert_eq!(empty.venues, []);
}

#[test]
fn invalid_rows_are_accounted_for_and_later_rows_still_load() {
    let report = parse_venues(r#"{"venues":[{}, false, {"id":" ", "name":"Given"}, {"id":"missing-name"}, {"id":"ok","name":" Room "}]}"#, YEAR).unwrap();
    assert_eq!(report.source_record_count, 5);
    assert_eq!(
        report
            .rejected_records
            .iter()
            .map(|row| row.record_index)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    assert_eq!(report.venues.len(), 1);
    let venue = &report.venues[0];
    assert_eq!(venue.name, "Room");
    assert_eq!(venue.id.as_str(), "ok");
    assert_eq!(venue.sources[0].record_index, 4);
    assert_eq!(venue.sources[0].venue_id.as_ref().unwrap().as_str(), "ok");
    assert_eq!(venue.capacity, Evidence::Missing);
    assert_eq!(venue.price, Evidence::Missing);
    let missing_name = diagnostic(&report, 3, "name", DiagnosticKind::MissingRequired);
    assert_eq!(
        missing_name.source.venue_id.as_ref().unwrap().as_str(),
        "missing-name"
    );
    assert_eq!(
        diagnostic(&report, 0, "id", DiagnosticKind::MissingRequired).raw,
        None
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.source.record_index == 1 && d.source.field.is_none())
    );
}

#[test]
fn capacity_distinguishes_ranges_layouts_and_standing_only() {
    let cases = [
        ("150", SeatingLayout::Unspecified, (150, 150), None),
        (r#""120""#, SeatingLayout::Unspecified, (120, 120), None),
        (r#""80-120""#, SeatingLayout::Unspecified, (80, 120), None),
        (
            r#""90 seated / 130 standing""#,
            SeatingLayout::Seated,
            (90, 90),
            Some(130),
        ),
    ];
    for (value, layout, (lower, upper), standing) in cases {
        let report = parse_row(&format!(r#""capacity":{value}"#));
        let capacity = known(&report.venues[0].capacity);
        assert_eq!(capacity.seated[0].layout, layout);
        assert_eq!(capacity.seated[0].bounds().minimum(), lower);
        assert_eq!(capacity.seated[0].bounds().maximum(), upper);
        assert_eq!(capacity.standing.map(NonZeroU32::get), standing);
    }
    let report = parse_row(r#""capacity":"130 standing""#);
    let capacity = known(&report.venues[0].capacity);
    assert_eq!(capacity.seated, []);
    assert_eq!(capacity.standing.map(NonZeroU32::get), Some(130));

    let report = parse_row(r#""capacity":{"plenary_cabaret":90,"plenary_theatre":140}"#);
    let seats = &known(&report.venues[0].capacity).seated;
    assert_eq!(seats.len(), 2);
    assert!(
        seats
            .iter()
            .any(|s| s.layout == SeatingLayout::Theatre && s.bounds().maximum() == 140)
    );
    assert!(
        seats
            .iter()
            .any(|s| s.layout == SeatingLayout::Cabaret && s.bounds().maximum() == 90)
    );
}

#[test]
fn invalid_capacity_is_not_replaced_by_claim_in_notes() {
    for value in [
        "0",
        "-1",
        "4294967296",
        r#""120-80""#,
        r#"{"plenary_cabaret":null}"#,
    ] {
        let report = parse_row(&format!(
            r#""capacity":{value},"notes":"website says up to 150""#
        ));
        assert!(matches!(
            report.venues[0].capacity,
            Evidence::Invalid { .. }
        ));
        assert_eq!(
            report.venues[0].notes.as_deref(),
            Some("website says up to 150")
        );
        assert_eq!(
            diagnostic(&report, 0, "capacity", DiagnosticKind::Invalid)
                .source
                .venue_id
                .as_ref()
                .unwrap()
                .as_str(),
            "case"
        );
    }
}

#[test]
fn breakout_ranges_qualifications_and_zero_survive() {
    for (value, min, max, qualification) in [
        ("0", 0, 0, None),
        ("4", 4, 4, None),
        (r#""4-6""#, 4, 6, None),
        (
            r#""3 (one is the lounge)""#,
            3,
            3,
            Some("one is the lounge"),
        ),
    ] {
        let report = parse_row(&format!(r#""breakout_rooms":{value}"#));
        let rooms = known(&report.venues[0].breakouts);
        assert_eq!((rooms.count.minimum(), rooms.count.maximum()), (min, max));
        assert_eq!(rooms.qualification.as_deref(), qualification);
    }
}

#[test]
fn object_quotes_and_dutch_part_day_quotes_use_minor_units() {
    for (value, minor, basis, vat) in [
        (
            r#"{"amount":72.5,"type":"per_person","currency":"EUR","vat":"incl"}"#,
            7250,
            PriceBasis::PerPerson,
            VatStatus::Included,
        ),
        (
            r#"{"amount":9800,"type":"per_day","currency":"EUR","vat":"excl"}"#,
            980_000,
            PriceBasis::PerDay,
            VatStatus::Excluded,
        ),
        (
            r#""€4.200 per dagdeel ex. BTW""#,
            420_000,
            PriceBasis::PerPartDay,
            VatStatus::Excluded,
        ),
    ] {
        let report = parse_row(&format!(r#""price":{value}"#));
        let price = known(&report.venues[0].price);
        assert_eq!(price.amount, Evidence::Known(MinorUnits::new(minor)));
        assert_eq!(price.basis, Evidence::Known(basis));
        assert_eq!(price.vat, Evidence::Known(vat));
        assert_eq!(price.currency, Evidence::Known(Currency::Eur));
    }
}

#[test]
fn monetary_precision_and_overflow_are_checked_without_floats() {
    for (value, expected) in [
        ("90071992547409.91", Some(9_007_199_254_740_991)),
        ("184467440737095516.15", Some(u64::MAX)),
        ("184467440737095516.16", None),
        ("72.501", None),
        ("-5", None),
        (r#""not a price""#, None),
    ] {
        let report = parse_row(&format!(r#""price":{{"amount":{value},"currency":"EUR"}}"#));
        let price = known(&report.venues[0].price);
        match expected {
            Some(minor) => assert_eq!(price.amount, Evidence::Known(MinorUnits::new(minor))),
            None => assert!(matches!(price.amount, Evidence::Invalid { .. })),
        }
    }
}

#[test]
fn ambiguous_amount_and_invalid_floor_preserve_other_quote_evidence() {
    let report = parse_row(
        r#""price":{"amount":"7.900","type":"per_day","currency":"EUR","vat":"excl","minimum_spend":"broken"}"#,
    );
    let price = known(&report.venues[0].price);
    assert_eq!(price.amount, Evidence::Unknown);
    assert_eq!(price.basis, Evidence::Known(PriceBasis::PerDay));
    assert_eq!(price.vat, Evidence::Known(VatStatus::Excluded));
    assert_eq!(
        price.minimum_spend,
        Evidence::Invalid {
            raw: "broken".into()
        }
    );
    assert_eq!(
        diagnostic(&report, 0, "price.amount", DiagnosticKind::Ambiguous)
            .raw
            .as_deref(),
        Some("7.900")
    );
    assert_eq!(
        diagnostic(&report, 0, "price.minimum_spend", DiagnosticKind::Invalid)
            .raw
            .as_deref(),
        Some("broken")
    );
}

#[test]
fn minimum_spend_and_catering_are_independent_of_vat_and_quote_presence() {
    let report = parse_row(
        r#""price":{"amount":95,"currency":"EUR","type":"per_person","minimum_spend":10000,"vat":null},"catering_included":"partial""#,
    );
    let venue = &report.venues[0];
    let price = known(&venue.price);
    assert_eq!(
        price.minimum_spend,
        Evidence::Known(MinorUnits::new(1_000_000))
    );
    assert_eq!(price.vat, Evidence::Unknown);
    assert_eq!(venue.catering, Evidence::Known(CateringStatus::Partial));
    let report = parse_row(r#""price":"on request","catering_included":true"#);
    assert_eq!(report.venues[0].price, Evidence::Unknown);
    assert_eq!(
        report.venues[0].catering,
        Evidence::Known(CateringStatus::Included)
    );
    let report = parse_row(r#""price":null,"catering_included":null"#);
    assert_eq!(report.venues[0].price, Evidence::Unknown);
    assert_eq!(report.venues[0].catering, Evidence::Unknown);
}

#[test]
fn foreign_currency_is_preserved_without_assuming_conversion_or_scale() {
    let report = parse_row(r#""price":{"amount":110,"currency":"USD","type":"per_person"}"#);
    let price = known(&report.venues[0].price);
    assert_eq!(price.currency, Evidence::Known(Currency::Usd));
    assert_eq!(price.amount, Evidence::Known(MinorUnits::new(11_000)));
    assert_eq!(price.vat, Evidence::Missing);
    let report = parse_row(r#""price":{"amount":100,"currency":"JPY","type":"per_day"}"#);
    let price = known(&report.venues[0].price);
    assert_eq!(
        price.currency,
        Evidence::Known(Currency::Other("JPY".into()))
    );
    assert_eq!(price.amount, Evidence::Unknown);
    diagnostic(&report, 0, "price.amount", DiagnosticKind::Unsupported);
}

#[test]
fn accessibility_quality_and_transport_keep_adverse_and_partial_evidence() {
    for (value, expected) in [
        ("true", Evidence::Known(Accessibility::ReportedAccessible)),
        ("false", Evidence::Known(Accessibility::Inaccessible)),
        (
            r#""ground floor only""#,
            Evidence::Known(Accessibility::Partial("ground floor only".into())),
        ),
        (
            r#""yes, except casemate rooms""#,
            Evidence::Known(Accessibility::Partial("yes, except casemate rooms".into())),
        ),
        (r#""unknown""#, Evidence::Unknown),
    ] {
        let report = parse_row(&format!(
            r#""wheelchair_accessible":{value},"nearest_station":"Utrecht CS","walk_minutes":"~10 min","notes":"Breakouts stairs only. Tram stop nearby.","daylight":"no windows in plenary","wifi":"unknown""#
        ));
        let venue = &report.venues[0];
        assert_eq!(venue.accessibility, expected);
        assert_eq!(venue.daylight, Evidence::Known(Quality::Unacceptable));
        assert_eq!(venue.wifi, Evidence::Unknown);
        let station = &venue.station_access;
        assert_eq!(
            station.station,
            Evidence::Known("Utrecht Centraal".to_owned())
        );
        assert_eq!(
            station.walk,
            Evidence::Known(WalkTime {
                minutes: 10,
                approximate: true
            })
        );
        assert_eq!(
            station.public_transport_details.as_deref(),
            venue.notes.as_deref()
        );
    }
}

#[test]
fn availability_preserves_specific_exceptions_before_broad_rules() {
    let report = parse_row(r#""availability":"March: Thursdays only, except 18 March""#);
    let schedule = known(&report.venues[0].availability);
    let exception = &schedule.rules[0];
    assert_eq!(
        exception.dates.start(),
        NaiveDate::from_ymd_opt(2027, 3, 18).unwrap()
    );
    assert_eq!(exception.dates.start(), exception.dates.end());
    assert_eq!(exception.status, DateStatus::Unavailable);
    assert_eq!(
        schedule.rules[1].weekdays.as_deref(),
        Some([Weekday::Thu].as_slice())
    );
    assert_eq!(schedule.rules[1].status, DateStatus::Available);
    assert_eq!(schedule.rules[2].status, DateStatus::Unavailable);
    diagnostic(&report, 0, "availability", DiagnosticKind::AssumedYear);

    let report = parse_row(r#""availability":"March 2027: fully booked except 9 and 11 March""#);
    let schedule = known(&report.venues[0].availability);
    assert_eq!(schedule.rules.len(), 3);
    assert_eq!(schedule.rules[0].status, DateStatus::Available);
    assert_eq!(
        schedule.rules[1].dates.start(),
        NaiveDate::from_ymd_opt(2027, 3, 11).unwrap()
    );
    assert_eq!(schedule.rules[2].status, DateStatus::Unavailable);
    assert_eq!(report.diagnostics, []);
}

#[test]
fn availability_preserves_exact_dates_closure_lead_time_and_venue_preference() {
    for (label, rules, lead, preference) in [
        ("Tue 9, 16, 23 March; Thu 11, 25 March", 5, None, None),
        ("Mon-Fri in March, book 6 weeks ahead", 1, Some(42), None),
        (
            "Tue/Thu in March, Tuesdays preferred by venue",
            1,
            None,
            Some(Weekday::Tue),
        ),
        ("fully booked Q1 2027", 1, None, None),
    ] {
        let report = parse_row(&format!(r#""availability":"{label}""#));
        let schedule = known(&report.venues[0].availability);
        assert_eq!(schedule.rules.len(), rules);
        assert_eq!(schedule.minimum_notice_days, lead);
        assert_eq!(schedule.preferred_weekday, preference);
        if label.starts_with("fully booked") {
            assert_eq!(schedule.rules[0].status, DateStatus::Unavailable);
            assert_eq!(
                schedule.rules[0].dates.start(),
                NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()
            );
            assert_eq!(
                schedule.rules[0].dates.end(),
                NaiveDate::from_ymd_opt(2027, 3, 31).unwrap()
            );
        }
    }
}

#[test]
fn ambiguous_dates_never_become_open_days() {
    for (label, kind) in [
        ("most Thursdays in March", DiagnosticKind::Ambiguous),
        ("Tuesdays only", DiagnosticKind::Ambiguous),
        ("ask", DiagnosticKind::Unknown),
        ("Tue 11 March", DiagnosticKind::Invalid),
        ("Thu 32 March", DiagnosticKind::Invalid),
    ] {
        let report = parse_row(&format!(r#""availability":"{label}""#));
        assert!(!matches!(report.venues[0].availability, Evidence::Known(_)));
        diagnostic(&report, 0, "availability", kind);
    }
    let report = parse_venues(
        r#"{"venues":[{"id":"case","name":"Room","availability":"Thu 11 March"}]}"#,
        None,
    )
    .unwrap();
    assert_eq!(report.venues[0].availability, Evidence::Unknown);
    diagnostic(&report, 0, "availability", DiagnosticKind::Ambiguous);
}

#[test]
fn absent_station_name_does_not_erase_walk_or_transport_evidence() {
    let report = parse_row(r#""walk_minutes":12,"notes":"Tram stop nearby.""#);
    let station = &report.venues[0].station_access;

    assert_eq!(station.station, Evidence::Missing);
    assert_eq!(
        station.walk,
        Evidence::Known(WalkTime {
            minutes: 12,
            approximate: false
        })
    );
    assert_eq!(
        station.public_transport_details.as_deref(),
        Some("Tram stop nearby.")
    );
}

#[test]
fn rejects_repeated_json_properties_without_losing_adjacent_rows() {
    for (fields, path) in [
        (
            r#""wheelchair_accessible": false, "wheelchair_accessible": true"#,
            "wheelchair_accessible",
        ),
        (
            r#""wheelchair_accessible": true, "wheelchair_accessible": false"#,
            "wheelchair_accessible",
        ),
        (r#""wifi":"poor","w\u0069fi":"fiber""#, "wifi"),
        (r#""capacity":60,"capacity":160"#, "capacity"),
        (
            r#""price":{"amount":18000},"price":{"amount":9000}"#,
            "price",
        ),
        (r#""price":{"amount":18000,"amount":9000}"#, "price.amount"),
        (
            r#""extension":[{"note":false,"note":true}]"#,
            "extension[0].note",
        ),
    ] {
        let input = format!(
            r#"{{"venues":[{{"id":"a","name":"Hall",{fields}}},{{"id":"b","name":"Annexe"}}]}}"#
        );
        let report = parse_venues(&input, YEAR).unwrap();
        assert_eq!(report.source_record_count, 2);
        assert_eq!(report.venues.len(), 1);
        assert_eq!(report.venues[0].sources[0].record_index, 1);
        assert_eq!(report.rejected_records.len(), 1);
        assert_eq!(report.rejected_records[0].record_index, 0);
        assert_eq!(
            report.rejected_records[0]
                .venue_id
                .as_ref()
                .unwrap()
                .as_str(),
            "a"
        );
        assert_eq!(report.diagnostics[0].source.field.as_deref(), Some(path));
    }
    assert!(matches!(
        parse_venues(r#"{"venues":[],"venues":[]}"#, YEAR),
        Err(InputError::DuplicateEnvelopeKey(key)) if key == "venues"
    ));
    let report = parse_venues(
        r#"{"venues":[{"id":"first","id":"second","name":"Hall"}]}"#,
        YEAR,
    )
    .unwrap();
    assert_eq!(report.rejected_records[0].venue_id, None);
    assert_eq!(report.diagnostics[0].kind, DiagnosticKind::DuplicateKey);
}

#[test]
fn named_file_parse_error_retains_path_and_json_cause() {
    use std::{error::Error, path::Path};

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let error = load_venues(&path, YEAR).unwrap_err();
    let InputError::FileParse {
        path: actual,
        source,
    } = &error
    else {
        panic!("expected a named file parse error");
    };
    assert_eq!(actual, &path);
    let InputError::Json(json) = source.as_ref() else {
        panic!("expected a JSON error");
    };
    assert_eq!((json.line(), json.column()), (1, 2));
    assert!(error.to_string().contains(path.to_str().unwrap()));
    assert!(error.to_string().contains("line 1 column 2"));
    assert!(error.source().and_then(Error::source).is_some());
}

#[test]
fn deduplication_preserves_source_positions_when_internal_listings_are_reordered() {
    use crate::dedup::deduplicate;

    for input in [
        include_str!("../../venues.json"),
        r#"{"venues":[
            {"id":"same","name":"Assembly Hall","city":"Utrecht"},
            {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"Street 1"},
            {"id":"same","name":"Assembly Hall","city":"Utrecht"},
            {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"street  1"}
        ]}"#,
    ] {
        let original = parse_venues(input, YEAR).unwrap();
        let mut permuted = parse_venues(input, YEAR).unwrap();
        permuted.venues.reverse();
        let shift = 7 % permuted.venues.len();
        permuted.venues.rotate_left(shift);
        assert_eq!(deduplicate(&permuted), deduplicate(&original));
    }
}
