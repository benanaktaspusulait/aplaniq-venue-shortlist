# TASK-008 — End-to-End Validation

## Status

To do.

## Objective

Validate core-to-CLI behaviour with controlled fixtures and the supplied export.

## Business justification

Individually correct modules can still lose evidence or use the wrong request when wired together.

## Dependencies

[TASK-007](TASK-007-cli.md) for the documented runnable entry point and component tests from TASK-002 through TASK-007.

## Requirements and scope

Add a small integration fixture that exercises the pipeline and run a smoke check against `venues.json`. Check result ordering, explanation, warning propagation, reproducibility and command failure behaviour. Record actual outputs and outstanding issues.

## Explicitly out of scope

Replacing component tests, large snapshot baselines, benchmark infrastructure and editing the original dataset.

## Expected files and modules

`tests/shortlist.rs` or a similarly scoped integration test, optional small fixtures under `tests/fixtures/`, and `docs/assessment-log.md`.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Use the positive fixture from AC-01 and add rejected and unresolved cases under the approved policy. Use the acceptance matrix to select integration coverage. Assert meaningful outcomes and reasons. Repeat the exact documented CLI invocation against the export. Check one plausible inclusion or exclusion manually against the input and selected product policy.

## Acceptance criteria

- Given a controlled fixture, confirmed IDs, order and reasons match the expected policy. The fixture exercises at least one invalid or unresolved value.
- Given the documented command and unchanged real export, two runs complete with matching result order and explanations. A valid empty output explains why it is empty.
- Given malformed input, the complete command exits non-zero with an actionable diagnostic.

## Required tests and edge cases

Small fixture with multiple verdicts including a hard failure, malformed input, real-export smoke and repeatability. Include a conflict or qualified access statement that is visible downstream.

## Validation commands

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo check` and the documented default command twice. Verify the dataset SHA-256 against the [data audit](../data-analysis.md).

## Definition of done

Integration tests and smoke checks pass or remaining failures are recorded with evidence. Completion requires the expected success checks to pass. Review and commit tests and validation notes locally.

## Risks and unresolved assumptions

A strict policy can validly produce an empty real-data result. Use a controlled fixture to demonstrate a non-empty ranking and check its explanation.

## Estimated effort

12 minutes.

## Suggested Git commit messages

`test: verify the shortlist pipeline and original dataset`
