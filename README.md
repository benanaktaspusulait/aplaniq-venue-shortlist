# AplanIQ Venue Shortlist

A local Rust CLI planned for finding and explaining venues that meet an event's hard requirements. The assessment case is a partner day near Utrecht with uncertain attendance, an approximate €15,000 budget and wheelchair access needs.

## Project status

The Cargo package and executable are named `venue-shortlist`. The library crate is `venue_shortlist`. The original [venue dataset](venues.json) is included. The executable is a scaffold that prints a status message to stderr and exits with status 1. The assessment requires Claude Code. Earlier business implementation used Codex at the candidate’s direction. The deviation is recorded in the assessment log. Future business implementation uses Claude Code.

Phase 2 added the implementation backlog. The candidate-approved planning assumptions are recorded in TASK-001. TASK-002 added the typed Rust domain and six unit tests. TASK-003 added the library JSON loader and normaliser. The original export accounts for 30 normalised records and zero rejected records. TASK-004 will handle duplicate candidates when requested. Shortlist eligibility is preliminary and the dataset is unverified.

## Development

Use Git and rustup with the [pinned toolchain](rust-toolchain.toml), which includes rustfmt and Clippy. The package uses Edition 2024 and the verified MSRV is Rust 1.85.0. Read the [engineering standards](docs/engineering/rust-standards.md) and [quality gates](docs/engineering/quality-gates.md).

```sh
sh scripts/quality-gate.sh
```

For major reviews, also have Rust 1.85.0 available and run `sh scripts/quality-gate.sh --major`. The workflow definition reuses that command. Hosted execution is still unverified.

To run the current scaffold, use `cargo run --quiet --`. The final shortlist command and its inputs will be documented during implementation.

## Assessment material

- [Requirements and client questions](docs/requirements.md)
- [Venue data audit](docs/data-analysis.md)
- [Architecture, dependencies and test strategy](docs/architecture.md)
- [Implementation backlog and execution order](docs/tasks/README.md)
- [Assessment notes](NOTES.md)
- [Development history and validation](docs/assessment-log.md)

Make a small local commit for each meaningful change. Inspect the status and staged diff, run relevant checks and stage only related files. Keep agent planning artifacts and genuine history. External submission and repository sharing require separate authorisation.
