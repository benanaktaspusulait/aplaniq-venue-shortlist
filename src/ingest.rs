//! Document loading and accounting for every source venue row.

use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde_json::Value;

pub use crate::diagnostic::{Diagnostic, DiagnosticKind};

use crate::domain::{SourceRef, Venue};
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
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(f, "cannot read {}: {source}", path.display()),
            Self::Json(source) => write!(f, "invalid venue JSON: {source}"),
            Self::Envelope => write!(f, "expected a top-level object with a venues array"),
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Json(source) => Some(source),
            Self::Envelope => None,
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
/// document errors reported by [`parse_venues`].
pub fn load_venues(
    path: impl AsRef<Path>,
    assumed_year: Option<i32>,
) -> Result<Ingestion, InputError> {
    let path = path.as_ref();
    let input = fs::read_to_string(path).map_err(|source| InputError::Read {
        path: path.to_owned(),
        source,
    })?;
    parse_venues(&input, assumed_year)
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
/// if the document lacks its top-level venues array.
pub fn parse_venues(input: &str, assumed_year: Option<i32>) -> Result<Ingestion, InputError> {
    let document: Value = serde_json::from_str(input).map_err(InputError::Json)?;
    let records = document
        .as_object()
        .and_then(|object| object.get("venues"))
        .and_then(Value::as_array)
        .ok_or(InputError::Envelope)?;

    let mut report = Ingestion {
        source_record_count: records.len(),
        venues: Vec::new(),
        rejected_records: Vec::new(),
        diagnostics: Vec::new(),
    };
    for (index, value) in records.iter().enumerate() {
        let mut row = Row::new(index, value, &mut report.diagnostics);
        match normalize(value, assumed_year, &mut row) {
            Some(venue) => report.venues.push(venue),
            None => report.rejected_records.push(row.source(None)),
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests;
