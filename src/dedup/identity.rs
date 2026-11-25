use crate::domain::{Evidence, Venue, VenueId};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Match {
    Confirmed,
    Potential,
    Distinct,
}

pub(super) struct Identity<'a> {
    id: &'a VenueId,
    name: String,
    city: Option<String>,
    address: Option<String>,
    address_missing: bool,
    station: Option<String>,
    variant: String,
}

impl<'a> Identity<'a> {
    pub(super) fn new(venue: &'a Venue) -> Self {
        let name = name_key(&venue.name);
        let city = location_key(venue.city.as_deref());
        let address = match &venue.address {
            Evidence::Known(address) => location_key(Some(address)),
            _ => None,
        };
        let station = match &venue.station_access.station {
            Evidence::Known(station) => location_key(Some(station)),
            _ => None,
        };
        let variant = name_variant(&name, city.as_deref());
        Self {
            id: &venue.id,
            name,
            city,
            address,
            address_missing: venue.address == Evidence::Missing,
            station,
            variant,
        }
    }

    pub(super) fn can_confirm_without_address(&self, export: &[Self]) -> bool {
        self.address_missing
            && !export.iter().any(|other| {
                self.id == other.id
                    && self.name == other.name
                    && self.city == other.city
                    && !other.address_missing
            })
    }

    pub(super) fn compare(&self, other: &Self, allow_missing_address: bool) -> Match {
        if different_known(self.city.as_deref(), other.city.as_deref())
            || different_known(self.address.as_deref(), other.address.as_deref())
        {
            return Match::Distinct;
        }

        let same_id = self.id == other.id;
        let same_name = !self.name.is_empty() && self.name == other.name;
        let same_city = self.city.is_some() && self.city == other.city;
        let matching_address = self.address.is_some() && self.address == other.address;
        let both_addresses_missing =
            allow_missing_address && self.address_missing && other.address_missing;
        if same_id && same_name && same_city && (matching_address || both_addresses_missing) {
            return Match::Confirmed;
        }

        let station_matches = self.station.is_some() && self.station == other.station;
        let compatible_variant =
            !self.variant.is_empty() && self.variant == other.variant && station_matches;
        if same_id || (same_city && (same_name || compatible_variant)) {
            return Match::Potential;
        }

        Match::Distinct
    }
}

fn different_known(left: Option<&str>, right: Option<&str>) -> bool {
    matches!((left, right), (Some(left), Some(right)) if left != right)
}

fn location_key(text: Option<&str>) -> Option<String> {
    text.map(|text| collapse_space(&text.to_lowercase()))
        .filter(|text| !text.is_empty())
}

fn collapse_space(text: &str) -> String {
    let mut collapsed = String::new();
    for word in text.split_whitespace() {
        if !collapsed.is_empty() {
            collapsed.push(' ');
        }
        collapsed.push_str(word);
    }
    collapsed
}

fn name_key(text: &str) -> String {
    // Canonicalise typographic dashes while retaining the boundary and all qualifiers.
    let mut key = String::new();
    for (index, part) in text.split(['-', '‐', '‑', '–', '—']).enumerate() {
        if index != 0 {
            key.push_str(" - ");
        }
        key.push_str(&collapse_space(&part.to_lowercase()));
    }
    key
}

fn name_variant(name: &str, city: Option<&str>) -> String {
    let mut stem = name.strip_prefix("het ").unwrap_or(name);
    if let Some(city) = city {
        let station_suffix = format!(" ({city} cs)");
        let city_suffix = format!(" {city}");
        stem = stem
            .strip_suffix(&station_suffix)
            .or_else(|| stem.strip_suffix(&city_suffix))
            .unwrap_or(stem);
    }
    stem.strip_suffix(" - event floor")
        .unwrap_or(stem)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Identity, Match};
    use crate::ingest::parse_venues;

    #[test]
    fn confirms_repeated_id_with_case_whitespace_and_typographic_dash_variations() {
        let first = json!({"id": "a", "name": "Étage - Noord", "city": "Utrecht"});
        for variant in [
            "Étage - Noord",
            "étage - NOORD",
            "  Étage   -   Noord ",
            "Étage—Noord",
        ] {
            let report = parse_venues(
                &json!({"venues": [first, {"id": "a", "name": variant, "city": "utrecht"}]})
                    .to_string(),
                None,
            )
            .unwrap();
            let [left, right] = report.venues.as_slice() else {
                panic!("expected two normalised listings");
            };
            assert_eq!(
                Identity::new(left).compare(&Identity::new(right), true),
                Match::Confirmed,
                "{variant}"
            );
        }
    }

    #[test]
    fn full_name_retains_diacritics_punctuation_and_space_qualifiers() {
        let report = parse_venues(
            &json!({"venues": [
                {"id": "a", "name": "Étage - Noord", "city": "Utrecht"},
                {"id": "a", "name": "Etage - Noord", "city": "Utrecht"},
                {"id": "a", "name": "Étage Noord", "city": "Utrecht"},
                {"id": "a", "name": "Étage - Zuid", "city": "Utrecht"}
            ]})
            .to_string(),
            None,
        )
        .unwrap();
        let (first, alternatives) = report.venues.split_first().unwrap();

        for other in alternatives {
            assert_eq!(
                Identity::new(first).compare(&Identity::new(other), true),
                Match::Potential
            );
        }
    }
}
