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

#[derive(Debug)]
pub enum InputError {
    Read { path: PathBuf, source: io::Error },
    Json(serde_json::Error),
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

#[derive(Debug, PartialEq, Eq)]
pub struct Ingestion {
    pub source_record_count: usize,
    pub venues: Vec<Venue>,
    pub rejected_records: Vec<SourceRef>,
    pub diagnostics: Vec<Diagnostic>,
}

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

/// Yearless March claims use the supplied context. Without it, they remain
/// ambiguous. An explicit year in a listing takes precedence.
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
