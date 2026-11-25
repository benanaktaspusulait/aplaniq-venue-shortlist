mod availability;
mod price;

use std::num::NonZeroU32;

use serde_json::Value;

use crate::diagnostic::{Diagnostic, DiagnosticKind};
use crate::domain::{
    Accessibility, Breakouts, Capacity, CateringStatus, CountRange, Evidence, Quality,
    SeatedCapacity, SeatingLayout, SourceRef, StationAccess, Venue, VenueId, WalkTime,
};

use DiagnosticKind::{Ambiguous, Invalid, Unknown, Unsupported};

type Issue = (DiagnosticKind, &'static str);

pub(super) struct Row<'a, 'report> {
    index: usize,
    id: Option<&'a str>,
    diagnostics: &'report mut Vec<Diagnostic>,
}

impl<'a, 'report> Row<'a, 'report> {
    pub(super) fn new(
        index: usize,
        value: &'a Value,
        diagnostics: &'report mut Vec<Diagnostic>,
    ) -> Self {
        Self {
            index,
            id: value.get("id").and_then(Value::as_str),
            diagnostics,
        }
    }

    pub(super) fn source(&self, field: Option<&str>) -> SourceRef {
        SourceRef {
            record_index: self.index,
            venue_id: self.id.and_then(|id| VenueId::new(id.to_owned())),
            field: field.map(str::to_owned),
        }
    }

    fn flag(&mut self, field: Option<&str>, (kind, message): Issue, value: Option<&Value>) {
        self.diagnostics.push(Diagnostic {
            source: self.source(field),
            kind,
            message,
            raw: value.map(raw),
        });
    }

    fn evidence<T>(
        &mut self,
        field: &str,
        value: Option<&Value>,
        parse: impl FnOnce(&Value) -> Result<T, Issue>,
    ) -> Evidence<T> {
        let Some(value) = value else {
            return Evidence::Missing;
        };
        let result = if value.is_null() {
            Err((Unknown, "source value is null"))
        } else {
            parse(value)
        };
        match result {
            Ok(known) => Evidence::Known(known),
            Err(issue) => {
                self.flag(Some(field), issue, Some(value));
                if issue.0 == Invalid {
                    Evidence::Invalid { raw: raw(value) }
                } else {
                    Evidence::Unknown
                }
            }
        }
    }

    fn required<'value>(
        &mut self,
        field: &str,
        value: Option<&'value Value>,
    ) -> Option<&'value str> {
        if let Some(text) = value
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            return Some(text);
        }
        let issue = if value.is_none() {
            (
                DiagnosticKind::MissingRequired,
                "required text field is missing",
            )
        } else {
            (Invalid, "expected nonblank text")
        };
        self.flag(Some(field), issue, value);
        None
    }

    fn optional_text(&mut self, field: &str, value: Option<&Value>) -> Option<String> {
        match self.evidence(field, value, |value| text(value).map(str::to_owned)) {
            Evidence::Known(text) => Some(text),
            _ => None,
        }
    }
}

pub(super) fn normalize(
    value: &Value,
    assumed_year: Option<i32>,
    row: &mut Row<'_, '_>,
) -> Option<Venue> {
    let Some(fields) = value.as_object() else {
        row.flag(None, (Invalid, "expected a venue object"), Some(value));
        return None;
    };

    let id = row
        .required("id", fields.get("id"))
        .and_then(|id| VenueId::new(id.to_owned()));
    let name = row.required("name", fields.get("name"));
    let (id, name) = (id?, name?);
    let notes = row.optional_text("notes", fields.get("notes"));
    let station_access = StationAccess {
        station: row.evidence(
            "nearest_station",
            fields.get("nearest_station"),
            station_name,
        ),
        walk: row.evidence("walk_minutes", fields.get("walk_minutes"), walk_time),
        public_transport_details: notes.as_deref().and_then(transport_note),
    };
    let daylight_text = fields.get("daylight");
    let wifi_text = fields.get("wifi");
    let mut venue = Venue {
        id,
        name: name.trim().to_owned(),
        city: row.optional_text("city", fields.get("city")),
        address: row.evidence("address", fields.get("address"), |value| {
            text(value).map(str::to_owned)
        }),
        sources: vec![row.source(None)],
        capacity: row.evidence("capacity", fields.get("capacity"), capacity),
        breakouts: row.evidence("breakout_rooms", fields.get("breakout_rooms"), breakouts),
        price: price::normalize(row, fields.get("price")),
        catering: row.evidence(
            "catering_included",
            fields.get("catering_included"),
            catering,
        ),
        accessibility: row.evidence(
            "wheelchair_accessible",
            fields.get("wheelchair_accessible"),
            accessibility,
        ),
        availability: availability::normalize(row, fields.get("availability"), assumed_year),
        station_access,
        daylight: row.evidence("daylight", daylight_text, daylight),
        wifi: row.evidence("wifi", wifi_text, wifi),
        daylight_qualification: None,
        wifi_qualification: None,
        notes,
    };
    venue.daylight_qualification = qualification(&venue.daylight, daylight_text);
    venue.wifi_qualification = qualification(&venue.wifi, wifi_text);
    Some(venue)
}

fn raw(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        _ => value.to_string(),
    }
}

fn text(value: &Value) -> Result<&str, Issue> {
    let text = value.as_str().ok_or((Invalid, "expected text"))?.trim();
    if text.is_empty() {
        return Err((Unknown, "source text is blank"));
    }
    Ok(text)
}

fn whole(value: &Value) -> Result<u32, Issue> {
    value
        .as_u64()
        .and_then(|n| u32::try_from(n).ok())
        .ok_or((Invalid, "expected a nonnegative whole number within range"))
}

fn count(text: &str) -> Result<u32, Issue> {
    text.trim()
        .parse()
        .map_err(|_| (Invalid, "expected a nonnegative whole count within range"))
}

fn count_range(value: &str) -> Result<CountRange, Issue> {
    if let Some((low, high)) = value.split_once(['-', '–']) {
        bounds(count(low)?, count(high)?)
    } else {
        let n = count(value)?;
        bounds(n, n)
    }
}

fn bounds(low: u32, high: u32) -> Result<CountRange, Issue> {
    CountRange::new(low, high).ok_or((Invalid, "count range is reversed"))
}

fn seats(layout: SeatingLayout, range: CountRange) -> Result<SeatedCapacity, Issue> {
    let positive = |n| NonZeroU32::new(n).ok_or((Invalid, "seated capacity must be positive"));
    SeatedCapacity::new(
        layout,
        positive(range.minimum())?,
        positive(range.maximum())?,
    )
    .ok_or((Invalid, "seated capacity range is reversed"))
}

fn capacity(value: &Value) -> Result<Capacity, Issue> {
    let mut result = Capacity {
        seated: Vec::new(),
        standing: None,
    };

    if let Some(object) = value.as_object() {
        for (key, value) in object {
            let layout = match key.as_str() {
                "plenary_theatre" => SeatingLayout::Theatre,
                "plenary_cabaret" => SeatingLayout::Cabaret,
                _ => return Err((Unsupported, "unrecognised room configuration")),
            };
            let n = whole(value)?;
            result.seated.push(seats(layout, bounds(n, n)?)?);
        }
        if result.seated.is_empty() {
            return Err((Invalid, "room configuration is empty"));
        }
        return Ok(result);
    }
    if value.is_number() {
        let n = whole(value)?;
        result
            .seated
            .push(seats(SeatingLayout::Unspecified, bounds(n, n)?)?);
        return Ok(result);
    }

    let label = text(value)?.to_ascii_lowercase();
    if !label.ends_with(" seated") && !label.ends_with(" standing") && !label.contains('/') {
        result
            .seated
            .push(seats(SeatingLayout::Unspecified, count_range(&label)?)?);
        return Ok(result);
    }
    for part in label.split('/').map(str::trim) {
        if let Some(n) = part
            .strip_suffix(" seated")
            .filter(|_| result.seated.is_empty())
        {
            result
                .seated
                .push(seats(SeatingLayout::Seated, count_range(n)?)?);
        } else if let Some(n) = part
            .strip_suffix(" standing")
            .filter(|_| result.standing.is_none())
        {
            result.standing = Some(
                NonZeroU32::new(count(n)?)
                    .ok_or((Invalid, "standing capacity must be positive"))?,
            );
        } else {
            return Err((
                Ambiguous,
                "unsupported or repeated seating and standing labels",
            ));
        }
    }
    Ok(result)
}

fn breakouts(value: &Value) -> Result<Breakouts, Issue> {
    if value.is_number() {
        let n = whole(value)?;
        return Ok(Breakouts {
            count: bounds(n, n)?,
            qualification: None,
        });
    }
    let label = text(value)?;
    let (count, qualification) = if let Some((count, detail)) = label.split_once('(') {
        let detail = detail
            .strip_suffix(')')
            .ok_or((Invalid, "unclosed breakout qualification"))?
            .trim();
        if detail.is_empty() {
            return Err((Invalid, "empty breakout qualification"));
        }
        (count, Some(detail.to_owned()))
    } else {
        (label, None)
    };
    Ok(Breakouts {
        count: count_range(count)?,
        qualification,
    })
}

fn catering(value: &Value) -> Result<CateringStatus, Issue> {
    match value {
        Value::Bool(true) => Ok(CateringStatus::Included),
        Value::Bool(false) => Ok(CateringStatus::Excluded),
        Value::String(_) if text(value)?.eq_ignore_ascii_case("partial") => {
            Ok(CateringStatus::Partial)
        }
        Value::String(_) if text(value)?.eq_ignore_ascii_case("unknown") => {
            Err((Unknown, "catering inclusion is unknown"))
        }
        _ => Err((
            Unsupported,
            "expected catering inclusion or a partial claim",
        )),
    }
}

fn accessibility(value: &Value) -> Result<Accessibility, Issue> {
    match value {
        Value::Bool(true) => Ok(Accessibility::ReportedAccessible),
        Value::Bool(false) => Ok(Accessibility::Inaccessible),
        Value::String(_) => {
            let original = text(value)?;
            let lower = original.to_ascii_lowercase();
            match lower.as_str() {
                "yes" => Ok(Accessibility::ReportedAccessible),
                "no" => Ok(Accessibility::Inaccessible),
                "unknown" | "ask" => Err((Unknown, "wheelchair access is unconfirmed")),
                "ground floor only" => Ok(Accessibility::Partial(original.to_owned())),
                _ if lower.starts_with("yes, except ") || lower.starts_with("lift to ") => {
                    Ok(Accessibility::Partial(original.to_owned()))
                }
                _ => Err((Unsupported, "unrecognised accessibility detail")),
            }
        }
        _ => Err((Invalid, "expected an accessibility flag or text")),
    }
}

fn station_name(value: &Value) -> Result<String, Issue> {
    let name = text(value)?;
    if name.eq_ignore_ascii_case("Utrecht CS") {
        return Ok("Utrecht Centraal".to_owned());
    }

    Ok(name.to_owned())
}

fn walk_time(value: &Value) -> Result<WalkTime, Issue> {
    if value.is_number() {
        return Ok(WalkTime {
            minutes: whole(value)?,
            approximate: false,
        });
    }
    let label = text(value)?;
    let (approximate, time) = match label.strip_prefix('~') {
        Some(rest) => (true, rest.trim()),
        None => (false, label),
    };
    let minutes = time
        .strip_suffix(" min")
        .ok_or((Unsupported, "expected a walking time in minutes"))?;
    Ok(WalkTime {
        minutes: count(minutes)?,
        approximate,
    })
}

fn transport_note(notes: &str) -> Option<String> {
    let lower = notes.to_ascii_lowercase();
    [
        "train",
        "bus",
        "tram",
        "taxi",
        "shuttle",
        "public transport",
        "from the station",
        "at the station",
        "from amsterdam",
    ]
    .iter()
    .any(|term| lower.contains(term))
    .then(|| notes.to_owned())
}

fn qualification(quality: &Evidence<Quality>, value: Option<&Value>) -> Option<String> {
    if matches!(quality, Evidence::Known(Quality::Partial)) {
        return value
            .and_then(Value::as_str)
            .map(str::trim)
            .map(str::to_owned);
    }
    None
}

fn daylight(value: &Value) -> Result<Quality, Issue> {
    match text(value)?.to_ascii_lowercase().as_str() {
        "yes" => Ok(Quality::Acceptable),
        "yes, skylights" | "yes, fully glazed" | "yes, greenhouse" => Ok(Quality::Strong),
        "no" | "no windows in plenary" => Ok(Quality::Unacceptable),
        "partial" | "limited" => Ok(Quality::Partial),
        "unknown" => Err((Unknown, "daylight is unconfirmed")),
        label if label.starts_with("mixed ") => Ok(Quality::Partial),
        _ => Err((Unsupported, "unrecognised daylight description")),
    }
}

fn wifi(value: &Value) -> Result<Quality, Issue> {
    match text(value)?.to_ascii_lowercase().as_str() {
        "fiber" | "good" => Ok(Quality::Acceptable),
        "fiber 1 gbps" | "fiber 500 mbps" | "fiber, 1 gbps, dedicated event network" => {
            Ok(Quality::Strong)
        }
        "weak" | "poor" | "unreliable" | "complaints reported" => Ok(Quality::Unacceptable),
        "guest wifi" | "fair" => Ok(Quality::Partial),
        "unknown" => Err((Unknown, "Wi-Fi quality is unconfirmed")),
        label if label.starts_with("4g backup only ") => Ok(Quality::Partial),
        _ => Err((Unsupported, "unrecognised Wi-Fi description")),
    }
}
