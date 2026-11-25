//! Public deduplication checks from JSON ingestion through source accounting.

use std::{collections::BTreeSet, path::Path};

use serde_json::json;
use venue_shortlist::{
    dedup::{ConflictField as Field, VenueEntity, deduplicate},
    domain::{
        Accessibility, Currency, Evidence, MinorUnits, PriceBasis, Quality, SeatingLayout, Venue,
    },
    ingest::{load_venues, parse_venues},
};

#[test]
fn original_export_flags_only_the_audited_pairs_and_accounts_for_each_index() {
    let report = load_venues(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json"),
        Some(2027),
    )
    .unwrap();
    let result = deduplicate(&report);
    let pairs: Vec<_> = result
        .potential_duplicates
        .iter()
        .map(|pair| (pair.left.id.as_str(), pair.right.id.as_str()))
        .collect();
    assert_eq!(
        pairs,
        [("v004", "v005"), ("v009", "v010"), ("v023", "v024")]
    );
    assert_eq!(result.source_record_count, 30);
    assert_eq!(result.normalised_record_count, 30);
    assert_eq!(result.entities.len(), 30);
    assert_eq!(result.confirmed_group_count, 0);
    assert_eq!(result.potential_record_count, 6);
    assert_eq!(result.unresolved_conflict_count, 0);
    assert_eq!(result.rejected_records, []);

    let mut accounted = vec![0; result.source_record_count];
    for listing in result.entities.iter().flat_map(VenueEntity::listings) {
        let [source] = listing.sources.as_slice() else {
            panic!("expected a unique source row for each listing");
        };
        assert_eq!(source.venue_id.as_ref(), Some(&listing.id));
        accounted[source.record_index] += 1;
    }
    assert!(accounted.into_iter().all(|count| count == 1));
    let spoorhuis = &result.potential_duplicates[0];
    assert_eq!(spoorhuis.left.wifi, Evidence::Known(Quality::Unacceptable));
    assert_eq!(
        spoorhuis.left.daylight,
        Evidence::Known(Quality::Unacceptable)
    );
    assert_eq!(spoorhuis.right.wifi, Evidence::Unknown);

    let mut shuffled = load_venues(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json"),
        Some(2027),
    )
    .unwrap();
    shuffled.venues.reverse();
    shuffled.venues.rotate_left(7);
    assert_eq!(deduplicate(&shuffled), result);
}

#[test]
fn ambiguous_middle_listing_cannot_bridge_different_known_addresses() {
    let input = r#"{"venues":[
        {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"Street 1"},
        {"id":"same","name":"Assembly Hall","city":"Utrecht"},
        {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"Street 2"},
        {"id":"other","name":"Assembly Hall - West","city":"Utrecht"}
    ]}"#;
    let report = parse_venues(input, None).unwrap();
    let result = deduplicate(&report);

    assert_eq!(result.entities.len(), 4);
    assert_eq!(result.confirmed_group_count, 0);
    assert_eq!(result.potential_record_count, 3);
    let pairs: Vec<_> = result
        .potential_duplicates
        .iter()
        .map(|pair| {
            (
                pair.left.sources[0].record_index,
                pair.right.sources[0].record_index,
            )
        })
        .collect();
    assert_eq!(pairs, [(0, 1), (1, 2)]);
}

#[test]
fn addressless_repeats_stay_separate_when_the_same_identity_has_address_evidence() {
    let input = r#"{"venues":[
        {"id":"same","name":"Assembly Hall","city":"Utrecht"},
        {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"Street 1"},
        {"id":"same","name":"Assembly Hall","city":"Utrecht"},
        {"id":"same","name":"Assembly Hall","city":"Utrecht","address":"street  1"}
    ]}"#;
    let mut report = parse_venues(input, None).unwrap();
    let result = deduplicate(&report);
    assert_eq!(result.entities.len(), 3);
    assert_eq!(result.confirmed_group_count, 1);
    assert_eq!(result.potential_duplicates.len(), 5);
    assert_eq!(result.potential_record_count, 4);
    let signature = result
        .entities
        .iter()
        .map(|entity| {
            entity
                .listings()
                .iter()
                .map(|listing| listing.sources[0].record_index)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(signature, [vec![0], vec![1, 3], vec![2]]);

    report.venues.reverse();
    let reversed = deduplicate(&report);
    assert_eq!(
        reversed
            .entities
            .iter()
            .map(|entity| {
                entity
                    .listings()
                    .iter()
                    .map(|listing| listing.sources[0].record_index)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        signature
    );
}

#[test]
fn inconsistent_identifiers_and_missing_or_invalid_identity_never_confirm() {
    let input = r#"{"venues":[
        {"id":"a","name":"Aula","city":"Utrecht","address":"Street 1"},
        {"id":"b","name":"Aula","city":"Utrecht","address":"Street 1"},
        {"id":"b","name":"Aula","address":"Street 1"},
        {"id":"c","name":"North","city":"Utrecht","address":17},
        {"id":"c","name":"North","city":"Utrecht"},
        {"id":"c","city":"Utrecht"}
    ]}"#;
    let report = parse_venues(input, None).unwrap();
    let result = deduplicate(&report);

    assert_eq!(result.entities.len(), 5);
    assert_eq!(result.confirmed_group_count, 0);
    assert_eq!(result.potential_duplicates.len(), 3);
    assert_eq!(result.potential_record_count, 5);
    assert_eq!(result.rejected_records[0].record_index, 5);
    assert_eq!(
        result.source_record_count,
        result.normalised_record_count + result.rejected_records.len()
    );
    assert_eq!(
        report.venues[3].address,
        Evidence::Invalid {
            raw: "17".to_owned()
        }
    );
}

#[test]
fn confirmed_groups_retain_whole_configurations_and_locate_each_conflict() {
    let input = r#"{"venues":[
        {"id":"a","name":"Aula","city":"Utrecht","capacity":"80-90 seated",
         "breakout_rooms":"3-4","price":{"type":"per_person","amount":95,"currency":"EUR","vat":"excl","minimum_spend":10000},
         "catering_included":true,"wheelchair_accessible":false,"wifi":"weak","daylight":"no",
         "availability":"Tue/Thu March 2027 open","nearest_station":"Utrecht CS","walk_minutes":3,"notes":"Train station on site."},
        {"id":"a","name":"Aula","city":"Utrecht","capacity":"150 standing",
         "breakout_rooms":5,"price":{"type":"per_day","amount":7000,"currency":"USD","vat":"incl","minimum_spend":8000},
         "catering_included":false,"wheelchair_accessible":true,"wifi":"fiber","daylight":"yes",
         "availability":"March 2027: fully booked except 9 and 11 March","nearest_station":"Utrecht Vaartsche Rijn","walk_minutes":12,"notes":"Take the tram."},
        {"id":"a","name":"Aula","city":"Utrecht","capacity":"85 seated"},
        {"id":"b","name":"Other Hall","city":"Amersfoort"},
        {"id":"b","name":"Other Hall","city":"Amersfoort"}
    ]}"#;
    let report = parse_venues(input, Some(2027)).unwrap();
    let result = deduplicate(&report);
    assert_eq!(result.source_record_count, 5);
    assert_eq!(result.normalised_record_count, 5);
    assert_eq!(result.entities.len(), 2);
    assert_eq!(result.confirmed_group_count, 2);
    assert_eq!(result.potential_record_count, 0);
    let group = &result.entities[0];
    let [left, right, sparse] = group.listings() else {
        panic!("expected all three source listings");
    };
    assert_eq!(result.entities[1].listings().len(), 2);
    assert_eq!(result.entities[1].conflicts(), []);
    assert_eq!(
        result
            .entities
            .iter()
            .flat_map(VenueEntity::listings)
            .map(|listing| listing.sources[0].record_index)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([0, 1, 2, 3, 4])
    );

    assert_source_configurations(left, right, sparse);

    let fields: Vec<_> = group
        .conflicts()
        .iter()
        .filter(|conflict| conflict.right.sources[0].record_index == 1)
        .map(|conflict| conflict.field)
        .collect();
    assert_eq!(
        fields,
        [
            Field::Capacity,
            Field::Breakouts,
            Field::PriceAmount,
            Field::Currency,
            Field::PriceBasis,
            Field::Vat,
            Field::MinimumSpend,
            Field::Catering,
            Field::Accessibility,
            Field::Availability,
            Field::Station,
            Field::Walk,
            Field::TransportDetails,
            Field::Daylight,
            Field::Wifi,
            Field::Notes,
        ]
    );
    for conflict in group
        .conflicts()
        .iter()
        .filter(|conflict| conflict.right.sources[0].record_index == 2)
    {
        assert_eq!(conflict.field, Field::Capacity);
    }
    assert_eq!(result.unresolved_conflict_count, 18);
}

fn assert_source_configurations(left: &Venue, right: &Venue, sparse: &Venue) {
    let Evidence::Known(seated) = &left.capacity else {
        panic!("expected seated capacity")
    };
    let Evidence::Known(standing) = &right.capacity else {
        panic!("expected standing capacity")
    };
    assert_eq!(seated.seated[0].layout, SeatingLayout::Seated);
    assert_eq!(seated.seated[0].bounds().minimum(), 80);
    assert_eq!(seated.seated[0].bounds().maximum(), 90);
    assert!(seated.standing.is_none());
    assert_eq!(standing.standing.unwrap().get(), 150);
    assert_eq!(standing.seated.as_slice(), []);
    assert_eq!(
        left.accessibility,
        Evidence::Known(Accessibility::Inaccessible)
    );
    assert_eq!(left.wifi, Evidence::Known(Quality::Unacceptable));
    assert_eq!(sparse.wifi, Evidence::Missing);

    let Evidence::Known(rate) = &left.price else {
        panic!("expected rate")
    };
    let Evidence::Known(quote) = &right.price else {
        panic!("expected quote")
    };
    assert_eq!(rate.amount, Evidence::Known(MinorUnits::new(9_500)));
    assert_eq!(rate.basis, Evidence::Known(PriceBasis::PerPerson));
    assert_eq!(rate.currency, Evidence::Known(Currency::Eur));
    assert_eq!(quote.basis, Evidence::Known(PriceBasis::PerDay));
    assert_eq!(quote.currency, Evidence::Known(Currency::Usd));
    assert_eq!(sparse.price, Evidence::Missing);
}

#[test]
fn accepts_empty_and_single_listing_exports() {
    let empty = parse_venues(r#"{"venues":[]}"#, None).unwrap();
    let empty = deduplicate(&empty);
    assert_eq!(empty.source_record_count, 0);
    assert_eq!(empty.normalised_record_count, 0);
    assert_eq!(empty.entities.as_slice(), []);
    assert_eq!(empty.potential_duplicates.as_slice(), []);
    assert_eq!(empty.unresolved_conflict_count, 0);

    let one = parse_venues(
        &json!({"venues": [{"id": "one", "name": "Aula"}]}).to_string(),
        None,
    )
    .unwrap();
    let one = deduplicate(&one);
    assert_eq!(one.entities[0].listings().len(), 1);
    assert_eq!(one.entities[0].listings()[0].id.as_str(), "one");
    assert_eq!(one.confirmed_group_count, 0);
    assert_eq!(one.potential_record_count, 0);
    assert_eq!(one.unresolved_conflict_count, 0);
}
