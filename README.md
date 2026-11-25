# AplanIQ Venue Shortlist

A local Rust CLI planned for finding and explaining venues that meet an event's hard requirements. The assessment case is a partner day near Utrecht with uncertain attendance, an approximate €15,000 budget and wheelchair access needs.

## Project status

The Cargo package and executable are named `venue-shortlist`. The library crate is `venue_shortlist`. The original [venue dataset](venues.json) is included. The executable is a scaffold that prints a status message to stderr and exits with status 1. The assessment requires Claude Code. The candidate directed this workspace to use Codex, and the deviation is recorded in the assessment log.

Phase 2 added the implementation backlog. The candidate-approved planning assumptions are recorded in TASK-001. TASK-002 added the typed Rust domain and six unit tests. TASK-003 is ready to start when requested. Shortlist eligibility is preliminary and the dataset is unverified.

## Development

Use Git and a stable Rust toolchain with Cargo, rustfmt and Clippy. The scaffold declares Rust 1.85 as its minimum for [edition 2024](https://doc.rust-lang.org/edition-guide/rust-2024/). Phase 1 was checked on stable Rust 1.100.0.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

To run the current scaffold, use `cargo run --quiet --`. The final shortlist command and its inputs will be documented during implementation.

## Assessment material

- [Requirements and client questions](docs/requirements.md)
- [Venue data audit](docs/data-analysis.md)
- [Architecture, dependencies and test strategy](docs/architecture.md)
- [Implementation backlog and execution order](docs/tasks/README.md)
- [Assessment notes](NOTES.md)
- [Development history and validation](docs/assessment-log.md)

Make a small local commit for each meaningful change. Inspect the status and staged diff, run relevant checks and stage only related files. Keep agent planning artifacts and genuine history. External submission and repository sharing require separate authorisation.
