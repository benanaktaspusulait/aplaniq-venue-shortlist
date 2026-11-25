# TASK-002 — Domain Model and Dependencies

## Status

To do.

## Objective

Define the minimal Rust types shared by parsing, evaluation and reporting.

## Business justification

Explicit types keep unavailable information, negative evidence, incomplete quotes and venue conflicts distinguishable through the pipeline.

## Dependencies

[TASK-001](TASK-001-requirements.md) for the request, policy and expected report.

## Requirements and scope

Define a typed event request, source references, venue evidence and outcome/report data needed by the first use cases. Cover capacity by setup, price basis and currency, date evidence, accessibility scope and recoverable diagnostics. Keep the library independent of terminal I/O and clock state.

## Explicitly out of scope

Parsing venue values, evaluating policy, scoring, CLI message wording and a generic rule engine.

## Expected files and modules

`src/lib.rs`, `src/domain.rs` and local unit tests. Add `Cargo.toml` and `Cargo.lock` changes only when a dependency has a caller.

## Implementation guidance

Start with the smallest types used by the next task. Represent unavailable, invalid and conflicting evidence explicitly. Carry source IDs and JSON field paths where needed. Separate fatal input errors from recoverable record diagnostics. Use standard error traits first. Confirm dependency and compiler compatibility when a consuming task adds a crate, and document changes to supported Rust versions.

## Acceptance criteria

- Given missing, invalid, negative and conflicting examples from the audit, callers can represent and distinguish each state in the model.
- A report consumer can retrieve the eligibility verdict and its reasons without parsing rendered prose.
- `cargo check` succeeds and the manifest has no speculative dependencies.

## Required tests and edge cases

Test invariants introduced by constructors, such as invalid ranges or negative monetary amounts, and distinguish explicit false from absent evidence. Plain data containers need no behaviour tests of their own.

## Validation commands

`cargo fmt --all -- --check`, `cargo test --lib`, `cargo check`.

## Definition of done

The model compiles, has any needed invariant tests, and supports representative raw evidence without prematurely encoding product outcomes. Review and commit the diff locally.

## Risks and unresolved assumptions

Availability detail, exact money representation and report scope depend on TASK-001. Prefer extending a type alongside its first real use.

## Estimated effort

8 minutes.

## Suggested Git commit messages

`feat: define shortlist domain evidence and report types`
