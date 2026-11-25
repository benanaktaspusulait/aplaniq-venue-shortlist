# TASK-009 — Final Independent Code Review (R4)

## Status

To do.

## Objective

Review the implementation independently of the implementing agent's completion claims and steer concrete fixes.

## Business justification

The assessment evaluates human steering and whether another engineer can maintain the delivered Rust code.

## Dependencies

[TASK-008](TASK-008-validation.md) for a runnable implementation, integration evidence and actual diffs.

## Requirements and scope

Inspect the Phase 1-to-implementation diff and acceptance evidence. Check correctness, Rust idioms, names, boundaries and avoidable complexity. Probe silent data bugs and inadequate tests. Log only observed findings, actual steering and the strongest diff. Keep review small and independent through human inspection or a fresh review pass.

## Explicitly out of scope

Broad rewrites, speculative refactoring, artificial findings and allowing a review report to stand in for inspecting evidence.

## Expected files and modules

`NOTES.md` review section, `docs/assessment-log.md` and only code or tests affected by reproduced findings.

## Implementation guidance

Start with hard rules, unknowns, date exceptions, price arithmetic, duplicate conflicts and reproducibility. Use a file or commit reference per actionable finding. Request focused fixes from the implementing agent and check the resulting diff and tests. Record any small manual edit and its location.

## Acceptance criteria

- A review record identifies the commits or files inspected, evidence checked and any observed problems or residual risks.
- For each fix, the record explains steering and confirms it against the actual follow-up diff and relevant checks. If a check reveals no issue, record that accurately.
- `NOTES.md` names one concrete diff the human is proud of and explains why.

## Required tests and edge cases

Re-run cases for any observed defect. Independently inspect at least one hard rejection, one unresolved-evidence case and one stable-order example.

## Validation commands

`git log --oneline`, `git diff --stat d27ce42..HEAD` and `git diff d27ce42..HEAD -- src tests Cargo.toml Cargo.lock`. Inspect follow-up diffs. Run `sh scripts/quality-gate.sh --major` after fixes.

## Definition of done

A human can point to the evidence behind the review and any fix. Record findings and genuine follow-up commits.

## Risks and unresolved assumptions

A same-agent self-report can miss its own errors. Time-box review and prioritise defects that can misclassify a venue or misstate evidence.

## Estimated effort

12 minutes.

## Suggested Git commit messages

`fix: address shortlist review findings` or `docs: record independent implementation review`. Use a more specific fix subject for a concrete defect.
