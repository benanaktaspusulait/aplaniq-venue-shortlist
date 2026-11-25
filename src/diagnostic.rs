use crate::domain::SourceRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    MissingRequired,
    Unknown,
    Invalid,
    Unsupported,
    Ambiguous,
    AssumedYear,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub source: SourceRef,
    pub kind: DiagnosticKind,
    pub message: &'static str,
    pub raw: Option<String>,
}
