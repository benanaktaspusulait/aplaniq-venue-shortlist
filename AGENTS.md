# Repository Guidelines

Read and follow the canonical [Rust engineering standards](docs/engineering/rust-standards.md) before making any change. Use the [requirements](docs/requirements.md) for business decisions and the [backlog](docs/tasks/README.md) for scope and review checkpoints. Preserve the original venue export and describe actual agent usage and results in the [assessment log](docs/assessment-log.md).

## Project Structure & Module Organization

The `venue-shortlist` Cargo package builds the `venue_shortlist` library and a thin `venue-shortlist` binary. Keep filesystem and CLI concerns at the edges. Ingestion reads the export and returns normalised evidence with source diagnostics. The domain owns invariants, including positive seated capacity, exact monetary amounts and explicit evidence states. Keep module dependencies acyclic and the library API focused. The architecture and current workflow are described in [architecture.md](docs/architecture.md).

## Build, Test, and Development Commands

Use the pinned Rust toolchain and the verified Rust 1.85.0 minimum. Run the [quality gate](docs/engineering/quality-gates.md) for every implementation task and its major mode at review checkpoints and delivery. To run the original export integration test, use `cargo test --locked --test ingestion original_export_accounts_for_every_source_row_and_preserves_evidence`.

## Testing and Reviews

Keep domain tests beside the code and cross-module tests in `tests/`. Trace new business tests to the acceptance criteria. Use the R1–R4 review evidence requirements in the standards. Report real gate output and test counts. Route business implementation and corrections through Claude Code. Codex audits, plans and enforces repository standards.

## Commit Guidelines

Use small Conventional Commits and include regression tests with their fixes. Stage the exact files for each change, review both staged and unstaged work, and preserve the user's staged changes. Keep history honest and seek explicit authorisation before pushing.
