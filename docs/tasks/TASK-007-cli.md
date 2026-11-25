# TASK-007 — CLI and Output Formatting

## Status

To do.

## Objective

Expose the full pipeline through one documented local command.

## Business justification

An assessor must be able to run the agreed event scenario and understand the results without reading Rust source.

## Dependencies

[TASK-006](TASK-006-ranking.md) for the typed report and ordered shortlist.

## Requirements and scope

Parse the agreed minimum CLI inputs, read the dataset, invoke the library and render the report. Document a working default invocation for the supplied export. Use stdout for the report and stderr for actionable fatal errors. Treat an empty valid report as successful and define the non-zero usage and input error behaviour.

## Explicitly out of scope

Interactive prompts, a UI or API, additional export formats and reimplementation of eligibility inside the binary.

## Expected files and modules

`src/main.rs`, `src/lib.rs` if orchestration needs a change, `README.md`, targeted CLI tests and Cargo manifests only if the chosen parser requires a dependency.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Keep path and argument handling in the binary. Use the standard library for a simple path argument or consider `clap` if several typed flags need validation and help. Render name or stable ID, hard-criteria rationale and ranking factors, relevant caveats and empty-result reasons. State the key assumptions for the default event request.

## Acceptance criteria

- Given the original export and documented command, the binary exits successfully and prints a readable ranked shortlist or an explained empty result.
- Given an invalid input path or malformed document, the CLI exits non-zero with actionable stderr and no partial report on stdout.
- Given a typed report with an unresolved candidate, render it in follow-up and label its unresolved evidence. Explain expansion limitations and distinguish preliminary suitability from booking checks. See AC-12–13.

## Required tests and edge cases

At least one focused command test for argument or file error handling and one formatter case for an empty or unresolved report. TASK-008 covers the complete success path.

## Validation commands

`cargo test`, `cargo run --quiet -- --help` if help is supported, and run exactly the default command recorded in `README.md`.

## Definition of done

The documented command works, report and error behaviour are tested, and main delegates business decisions to the library. Review and commit locally.

## Risks and unresolved assumptions

The smallest useful event input interface depends on TASK-001. A baseline default or a documented local request input can meet the one-command requirement.

## Estimated effort

13 minutes.

## Suggested Git commit messages

`feat: expose venue shortlist through the local CLI`
