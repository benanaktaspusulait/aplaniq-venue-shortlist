use std::{collections::BTreeSet, fmt};

use serde_core::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::value::RawValue;

use super::InputError;

// Read member values as original JSON slices. Decoded keys remain visible in
// source order, and numeric tokens never pass through a floating-point visitor.
struct Members<'a>(Vec<(String, &'a RawValue)>);

impl<'de> Deserialize<'de> for Members<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Object;

        impl<'de> Visitor<'de> for Object {
            type Value = Members<'de>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a JSON object")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut members = Vec::new();
                while let Some(entry) = map.next_entry()? {
                    members.push(entry);
                }
                Ok(Members(members))
            }
        }

        deserializer.deserialize_map(Object)
    }
}

pub(super) fn venue_rows(input: &str) -> Result<Vec<&RawValue>, InputError> {
    let object: Members<'_> = serde_json::from_str(input).map_err(InputError::Json)?;
    let mut keys = BTreeSet::new();
    let mut venues = None;
    for (key, value) in &object.0 {
        if !keys.insert(key) {
            return Err(InputError::DuplicateEnvelopeKey(key.clone()));
        }
        if key == "venues" {
            venues = Some(value.get());
        }
    }
    serde_json::from_str(venues.ok_or(InputError::Envelope)?).map_err(|_| InputError::Envelope)
}

pub(super) fn repeated_fields(row: &RawValue) -> Result<Vec<String>, InputError> {
    let mut repeated = Vec::new();
    // The ordinary JSON parse already validated syntax and bounded nesting.
    inspect(row, "", &mut repeated).map_err(InputError::Json)?;
    Ok(repeated)
}

fn inspect(raw: &RawValue, parent: &str, repeated: &mut Vec<String>) -> serde_json::Result<()> {
    match raw.get().trim_start().as_bytes().first() {
        Some(b'{') => {
            let object: Members<'_> = serde_json::from_str(raw.get())?;
            let mut seen = BTreeSet::new();
            for (key, value) in &object.0 {
                let path =
                    if key.chars().all(|c| c.is_alphanumeric() || c == '_') && !key.is_empty() {
                        if parent.is_empty() {
                            key.clone()
                        } else {
                            format!("{parent}.{key}")
                        }
                    } else {
                        format!("{parent}[{key:?}]")
                    };
                if !seen.insert(key) {
                    repeated.push(path.clone());
                }
                inspect(value, &path, repeated)?;
            }
        }
        Some(b'[') => {
            for (index, value) in serde_json::from_str::<Vec<&RawValue>>(raw.get())?
                .into_iter()
                .enumerate()
            {
                inspect(value, &format!("{parent}[{index}]"), repeated)?;
            }
        }
        _ => {}
    }
    Ok(())
}
