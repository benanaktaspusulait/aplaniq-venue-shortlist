use crate::domain::SourceRef;

/// Classification of a recoverable source issue or assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// The record lacks a field needed to identify it.
    MissingRequired,
    /// The source supplied no usable value.
    Unknown,
    /// A supplied value violates the field’s format or bounds.
    Invalid,
    /// The representation is outside the supported input formats.
    Unsupported,
    /// The available source text cannot support a single interpretation.
    Ambiguous,
    /// A caller-supplied year was applied to yearless availability.
    AssumedYear,
    /// A repeated JSON object key caused the containing row to be rejected.
    DuplicateKey,
}

/// Diagnostic and provenance for an individual record or field.
#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Original record and optional field path.
    pub source: SourceRef,
    /// Machine-readable issue or assumption category.
    pub kind: DiagnosticKind,
    /// Short explanation of the issue or inference.
    pub message: &'static str,
    /// Source value when present and relevant to the diagnostic.
    pub raw: Option<String>,
}
