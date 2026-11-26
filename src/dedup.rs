//! Identity decisions and unresolved source disagreements within a venue export.

mod conflict;
mod identity;

use std::collections::BTreeSet;

use crate::{
    domain::{SourceRef, Venue},
    ingest::Ingestion,
};

pub use conflict::ConflictField;
use identity::{Identity, Match};

/// An unresolved difference between two source listings of one entity.
#[derive(Debug, PartialEq, Eq)]
pub struct Conflict<'a> {
    /// Field whose known claims disagree.
    pub field: ConflictField,
    /// Earlier listing in entity order.
    pub left: &'a Venue,
    /// Later listing in entity order.
    pub right: &'a Venue,
}

/// Two separately retained listings that need identity verification.
#[derive(Debug, PartialEq, Eq)]
pub struct PotentialDuplicate<'a> {
    /// Earlier member of the pair in listing order.
    pub left: &'a Venue,
    /// Later member of the pair in listing order.
    pub right: &'a Venue,
}

/// An entity backed by at least one original, unmodified normalised listing.
#[derive(Debug, PartialEq, Eq)]
pub struct VenueEntity<'a> {
    listings: Vec<&'a Venue>,
    conflicts: Vec<Conflict<'a>>,
}

impl<'a> VenueEntity<'a> {
    /// Whole listing configurations, in source index order within the entity.
    #[must_use]
    pub fn listings(&self) -> &[&'a Venue] {
        &self.listings
    }

    /// Disagreements ordered by later listing, earlier listing, then field.
    #[must_use]
    pub fn conflicts(&self) -> &[Conflict<'a>] {
        &self.conflicts
    }
}

/// Accounting and identity diagnostics for one ingested export.
///
/// Potential identity pairs stay attached to the grouping used for evaluation.
///
/// ```compile_fail
/// use venue_shortlist::{dedup::deduplicate, ingest::parse_venues};
///
/// let report = parse_venues(r#"{"venues":[]}"#, None).unwrap();
/// let mut groups = deduplicate(&report);
/// groups.potential_duplicates.clear();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Deduplication<'a> {
    source_record_count: usize,
    normalised_record_count: usize,
    rejected_records: &'a [SourceRef],
    entities: Vec<VenueEntity<'a>>,
    potential_duplicates: Vec<PotentialDuplicate<'a>>,
    confirmed_group_count: usize,
    potential_record_count: usize,
    unresolved_conflict_count: usize,
}

impl<'a> Deduplication<'a> {
    /// Original number of source rows, including rejected rows.
    #[must_use]
    pub fn source_record_count(&self) -> usize {
        self.source_record_count
    }

    /// Number of normalised source listings.
    #[must_use]
    pub fn normalised_record_count(&self) -> usize {
        self.normalised_record_count
    }

    /// Rows rejected before identity comparison.
    #[must_use]
    pub fn rejected_records(&self) -> &[SourceRef] {
        self.rejected_records
    }

    /// Distinct entities ordered by first listing ID and original row index.
    #[must_use]
    pub fn entities(&self) -> &[VenueEntity<'a>] {
        &self.entities
    }

    /// Unresolved identity pairs ordered by later listing, then earlier listing.
    #[must_use]
    pub fn potential_duplicates(&self) -> &[PotentialDuplicate<'a>] {
        &self.potential_duplicates
    }

    /// Number of entities with multiple confirmed listings.
    #[must_use]
    pub fn confirmed_group_count(&self) -> usize {
        self.confirmed_group_count
    }

    /// Number of distinct normalised rows involved in a potential pair.
    #[must_use]
    pub fn potential_record_count(&self) -> usize {
        self.potential_record_count
    }

    /// Number of unresolved field disagreements across confirmed listing pairs.
    #[must_use]
    pub fn unresolved_conflict_count(&self) -> usize {
        self.unresolved_conflict_count
    }
}

/// Groups strong identity matches in one export and keeps every listing intact.
///
/// A repeated export ID, the full normalised name and a known matching city must
/// agree. An address must match, or be omitted throughout the listings with
/// that ID, name and city. Comparisons using
/// weaker signals only produce potential-pair diagnostics. Original source row
/// indices break ties between listings that share an ID.
#[must_use]
pub fn deduplicate(report: &Ingestion) -> Deduplication<'_> {
    let mut listings: Vec<_> = report.venues().iter().collect();
    listings.sort_by_key(|venue| (&venue.id, venue.sources.first().map(|row| row.record_index)));

    let identities: Vec<_> = listings.iter().map(|venue| Identity::new(venue)).collect();
    let mut groups: Vec<(&Identity<'_>, VenueEntity<'_>)> = Vec::new();
    let mut potential_duplicates = Vec::new();
    let mut potential = BTreeSet::new();

    for (right, (listing, identity)) in listings.iter().zip(&identities).enumerate() {
        let allow_missing_address = identity.can_confirm_without_address(&identities);
        for (left, (earlier, earlier_identity)) in
            listings.iter().zip(&identities).take(right).enumerate()
        {
            if identity.compare(earlier_identity, allow_missing_address) == Match::Potential {
                potential_duplicates.push(PotentialDuplicate {
                    left: earlier,
                    right: listing,
                });
                potential.extend([left, right]);
            }
        }

        let group = groups
            .iter_mut()
            .find(|(first, _)| identity.compare(first, allow_missing_address) == Match::Confirmed);
        if let Some((_, entity)) = group {
            for earlier in &entity.listings {
                conflict::record(earlier, listing, &mut entity.conflicts);
            }
            entity.listings.push(listing);
        } else {
            groups.push((
                identity,
                VenueEntity {
                    listings: vec![listing],
                    conflicts: Vec::new(),
                },
            ));
        }
    }

    let entities: Vec<_> = groups.into_iter().map(|(_, entity)| entity).collect();
    Deduplication {
        source_record_count: report.source_record_count(),
        normalised_record_count: listings.len(),
        rejected_records: report.rejected_records(),
        confirmed_group_count: entities
            .iter()
            .filter(|entity| entity.listings.len() > 1)
            .count(),
        potential_record_count: potential.len(),
        unresolved_conflict_count: entities.iter().map(|entity| entity.conflicts.len()).sum(),
        entities,
        potential_duplicates,
    }
}
