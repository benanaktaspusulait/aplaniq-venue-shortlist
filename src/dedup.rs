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
#[derive(Debug, PartialEq, Eq)]
pub struct Deduplication<'a> {
    /// Original number of input rows, including rejected rows.
    pub source_record_count: usize,
    /// Number of normalised source listings.
    pub normalised_record_count: usize,
    /// Rows that ingestion rejected before identity comparison.
    pub rejected_records: &'a [SourceRef],
    /// Distinct entities ordered by first listing ID and original row index.
    pub entities: Vec<VenueEntity<'a>>,
    /// Unresolved identity pairs ordered by later listing, then earlier listing.
    pub potential_duplicates: Vec<PotentialDuplicate<'a>>,
    /// Entities containing multiple confirmed listings.
    pub confirmed_group_count: usize,
    /// Number of distinct normalised rows involved in a potential pair.
    pub potential_record_count: usize,
    /// Number of unresolved field disagreements across confirmed listing pairs.
    pub unresolved_conflict_count: usize,
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
    let mut listings: Vec<_> = report.venues.iter().collect();
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
        source_record_count: report.source_record_count,
        normalised_record_count: listings.len(),
        rejected_records: &report.rejected_records,
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
