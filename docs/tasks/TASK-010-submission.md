# TASK-010 — Documentation and Submission

## Status

To do.

## Objective

Prepare the assessor-facing repository and record final verification and time spent.

## Business justification

The assessor must be able to run the feature, audit the agent workflow and understand the decisions within a short handoff.

## Dependencies

[TASK-009](TASK-009-code-review.md) for reviewed code, review evidence and any final fixes.

## Requirements and scope

Confirm the one-command README and final checks. Keep `NOTES.md` at two pages maximum with Spec and assumptions, Product decisions and Review. Include at least three decisions, the observed review and strongest diff, manual fixes if any, time spent and what is next at the time limit. Include genuine planning artifacts and honest commit history. Disclose the deviation from the PDF’s Claude Code requirement. The eventual external deliverable is a private Git repository link shared with Joa.

## Explicitly out of scope

Deployment, public repository, invented agent history and any push or external sharing in the current local-only planning scope.

## Expected files and modules

`README.md`, `NOTES.md`, `docs/assessment-log.md` and planning artifacts actually produced by the selected agent workflow.

## Implementation guidance

Verify content from the tracked files and run the documented command in a clean local checkout or against an equivalent clean tree. Check privacy and staged content before a separately authorised submission. Stop at the three-hour total and record incomplete work, even if checks or handoff steps remain.

## Acceptance criteria

- README has one working command and `cargo test` passes. The final log contains actual check results and approximate elapsed time.
- NOTES has the required three sections, fits within two pages, and records the selected decisions and actual review. The tracked tree contains the genuine agent planning artifacts used for implementation and a disclosure of the actual agent.
- A local handoff is ready once these checks pass. The external submission milestone is satisfied only once the private link and Joa's access are verified under the user's authorisation.

## Required tests and edge cases

Final clean-tree command, final quality gates and verification of the tracked file list, no secrets in staged changes, the two-page notes limit and intact original data. If the time cap arrives first, record what remains.

## Validation commands

`sh scripts/quality-gate.sh --major`, the exact shortlist command in README, `git status --short`, `git diff --cached --check` and `git ls-files`.

## Definition of done

The verified local submission is committed and the delivery state is recorded accurately. Keep external delivery pending until its acceptance criterion is met.

## Risks and unresolved assumptions

Private access and sharing need user authorisation. The time cap can leave work unfinished. Prioritise a runnable, tested shortlist and honest notes.

## Estimated effort

8 minutes for local preparation. External access and delivery time are excluded.

## Suggested Git commit messages

`docs: finalise assessment notes and verification`
