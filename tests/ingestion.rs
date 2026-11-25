use std::path::Path;

use chrono::{NaiveDate, Weekday};
use serde_json::Value;
use venue_shortlist::{
    domain::{
        Accessibility, CateringStatus, Currency, Evidence, MinorUnits, PriceBasis, Quality,
        VatStatus,
    },
    ingest::{DiagnosticKind, load_venues},
};

fn known<T: std::fmt::Debug>(evidence: &Evidence<T>) -> &T {
    match evidence {
        Evidence::Known(value) => value,
        other => panic!("expected known evidence, got {other:?}"),
    }
}

#[test]
fn original_export_accounts_for_every_source_row_and_preserves_evidence() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json");
    let source: Value = serde_json::from_str(include_str!("../venues.json")).unwrap();
    let rows = source["venues"].as_array().unwrap();
    let report = load_venues(path, Some(2027)).unwrap();

    let mut seen = vec![0; rows.len()];
    for source in report
        .venues
        .iter()
        .flat_map(|venue| &venue.sources)
        .chain(&report.rejected_records)
    {
        seen[source.record_index] += 1;
        assert_eq!(
            source.venue_id.as_ref().map(|id| id.as_str()),
            rows[source.record_index]["id"].as_str()
        );
    }
    assert_eq!(report.source_record_count, 30);
    assert_eq!(report.venues.len(), 30);
    assert!(report.rejected_records.is_empty());
    assert!(seen.into_iter().all(|count| count == 1));
    for diagnostic in &report.diagnostics {
        let row = &rows[diagnostic.source.record_index];
        assert_eq!(
            diagnostic.source.venue_id.as_ref().map(|id| id.as_str()),
            row["id"].as_str()
        );
        let mut raw = row;
        for key in diagnostic.source.field.as_deref().unwrap().split('.') {
            raw = raw.get(key).unwrap();
        }
        let expected = raw.as_str().map_or_else(|| raw.to_string(), str::to_owned);
        assert_eq!(diagnostic.raw.as_deref(), Some(expected.as_str()));
    }
    assert!(report.diagnostics.iter().any(|d| d.source.record_index == 7
        && d.source.field.as_deref() == Some("price.amount")
        && d.kind == DiagnosticKind::Ambiguous));
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.source.record_index == 26
                && d.source.field.as_deref() == Some("capacity")
                && d.kind == DiagnosticKind::Invalid)
    );

    let stadstuin = report
        .venues
        .iter()
        .find(|venue| venue.id.as_str() == "v019")
        .unwrap();
    assert_eq!(known(&stadstuin.capacity).seated[0].bounds().maximum(), 85);
    assert_eq!(known(&stadstuin.breakouts).count.minimum(), 4);
    let price = known(&stadstuin.price);
    assert_eq!(price.basis, Evidence::Known(PriceBasis::PerPerson));
    assert_eq!(price.currency, Evidence::Known(Currency::Eur));
    assert_eq!(price.amount, Evidence::Known(MinorUnits::new(7_900)));
    assert_eq!(price.vat, Evidence::Known(VatStatus::Included));
    assert_eq!(
        stadstuin.catering,
        Evidence::Known(CateringStatus::Included)
    );
    assert_eq!(
        stadstuin.accessibility,
        Evidence::Known(Accessibility::ReportedAccessible)
    );
    assert_eq!(stadstuin.daylight, Evidence::Known(Quality::Strong));
    assert_eq!(stadstuin.wifi, Evidence::Known(Quality::Acceptable));
    assert_eq!(known(&stadstuin.station_access.walk).minutes, 12);
    let rule = &known(&stadstuin.availability).rules[0];
    assert_eq!(
        rule.dates.start(),
        NaiveDate::from_ymd_opt(2027, 3, 1).unwrap()
    );
    assert_eq!(
        rule.weekdays.as_deref(),
        Some([Weekday::Tue, Weekday::Thu].as_slice())
    );

    eprintln!(
        "dataset: {} rows, {} normalised, {} rejected, {} diagnostics",
        report.source_record_count,
        report.venues.len(),
        report.rejected_records.len(),
        report.diagnostics.len()
    );
}
