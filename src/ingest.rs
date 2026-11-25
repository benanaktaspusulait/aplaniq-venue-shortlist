//! Document loading and accounting for every source venue row.

use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

mod keys;

use serde_json::Value;

pub use crate::diagnostic::{Diagnostic, DiagnosticKind};

use crate::domain::{SourceRef, Venue, VenueId};
use crate::normalize::{Row, normalize};

/// Failure to load a complete venue input document.
#[derive(Debug)]
pub enum InputError {
    /// The input file could not be read as UTF-8.
    Read {
        /// Path passed by the caller.
        path: PathBuf,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// The input is not valid JSON.
    Json(serde_json::Error),
    /// The document does not contain a top-level array of venues under the expected key.
    Envelope,
    /// A repeated key makes the document envelope ambiguous.
    DuplicateEnvelopeKey(String),
    /// A named file was read but its contents could not be parsed.
    FileParse {
        /// Path passed by the caller.
        path: PathBuf,
        /// Underlying document error.
        source: Box<InputError>,
    },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(f, "cannot read {}: {source}", path.display()),
            Self::Json(source) => write!(f, "invalid venue JSON: {source}"),
            Self::DuplicateEnvelopeKey(key) => write!(f, "repeated envelope key {key:?}"),
            Self::FileParse { path, source } => {
                write!(f, "cannot parse {}: {source}", path.display())
            }
            Self::Envelope => write!(f, "expected a top-level object with a venues array"),
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Json(source) => Some(source),
            Self::FileParse { source, .. } => Some(source.as_ref()),
            Self::Envelope | Self::DuplicateEnvelopeKey(_) => None,
        }
    }
}

/// Normalised rows and diagnostics from a readable venue document.
#[derive(Debug, PartialEq, Eq)]
pub struct Ingestion {
    /// Number of rows in the source array.
    pub source_record_count: usize,
    /// Successfully identified and normalised records in source order.
    pub venues: Vec<Venue>,
    /// Source references for rows that could not be loaded.
    pub rejected_records: Vec<SourceRef>,
    /// Field and record issues, including nonfatal assumptions.
    pub diagnostics: Vec<Diagnostic>,
}

/// Reads and normalises the venue export at the given path.
///
/// See [`parse_venues`] for the availability year policy and row-level diagnostics.
///
/// # Errors
///
/// Returns [`InputError::Read`] for unreadable or non-UTF-8 input, or the
/// [`InputError::FileParse`] with the path and a document error from [`parse_venues`].
pub fn load_venues(
    path: impl AsRef<Path>,
    assumed_year: Option<i32>,
) -> Result<Ingestion, InputError> {
    let path = path.as_ref();
    let input = fs::read_to_string(path).map_err(|source| InputError::Read {
        path: path.to_owned(),
        source,
    })?;
    parse_venues(&input, assumed_year).map_err(|source| InputError::FileParse {
        path: path.to_owned(),
        source: Box::new(source),
    })
}

/// Parses a venue export while accounting for every input row.
///
/// Yearless March claims use the supplied context. Without it, they remain
/// ambiguous. An explicit year in a listing takes precedence. Recoverable row
/// and field issues are returned with the report.
///
/// # Errors
///
/// Returns [`InputError::Json`] for malformed JSON or [`InputError::Envelope`]
/// if the document lacks its top-level venues array. Repeated envelope keys
/// return [`InputError::DuplicateEnvelopeKey`]. Rows with repeated keys are
/// counted and rejected individually.
pub fn parse_venues(input: &str, assumed_year: Option<i32>) -> Result<Ingestion, InputError> {
    let document: Value = serde_json::from_str(input).map_err(InputError::Json)?;
    let root = document.as_object().ok_or(InputError::Envelope)?;
    let raw_records = keys::venue_rows(input)?;
    let records = root
        .get("venues")
        .and_then(Value::as_array)
        .ok_or(InputError::Envelope)?;

    let mut report = Ingestion {
        source_record_count: records.len(),
        venues: Vec::new(),
        rejected_records: Vec::new(),
        diagnostics: Vec::new(),
    };
    for (index, (value, raw)) in records.iter().zip(raw_records).enumerate() {
        let repeated = keys::repeated_fields(raw)?;
        if !repeated.is_empty() {
            reject_repeated_fields(&mut report, index, value, repeated);
            continue;
        }
        let mut row = Row::new(index, value, &mut report.diagnostics);
        match normalize(value, assumed_year, &mut row) {
            Some(venue) => report.venues.push(venue),
            None => report.rejected_records.push(row.source(None)),
        }
    }

    Ok(report)
}

fn reject_repeated_fields(
    report: &mut Ingestion,
    index: usize,
    row: &Value,
    repeated: Vec<String>,
) {
    let id = row
        .get("id")
        .and_then(Value::as_str)
        .filter(|_| !repeated.iter().any(|key| key == "id"));
    let source = |field| SourceRef {
        record_index: index,
        venue_id: id.and_then(|id| VenueId::new(id.to_owned())),
        field,
    };
    report.rejected_records.push(source(None));
    report
        .diagnostics
        .extend(repeated.into_iter().map(|key| Diagnostic {
            source: source(Some(key)),
            kind: DiagnosticKind::DuplicateKey,
            message: "repeated JSON property makes the row ambiguous",
            raw: None,
        }));
}

#[cfg(test)]
mod tests;
