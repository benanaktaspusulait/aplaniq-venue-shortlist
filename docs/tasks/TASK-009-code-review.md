# TASK-009 — Final Independent Code Review (R4)

## Status

The original R4 review was returned in the conversation. The local corrective work and review evidence are complete on the post-assessment branch. The original assessment process deviation remains.

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

## Review and resolution

Codex inspected the original PDF, source data, tests and the committed diff from `d27ce42` to the original snapshot `35570b4`. A second pass inspected the corrective diffs and fresh test output. The same Codex performed implementation and review.

| Finding | Evidence and resolution |
| --- | --- |
| R4-001 · HIGH · P0 | A sentence containing an optional screen and a required €300 AV charge incorrectly left a €14,900 package eligible. The new eligibility test failed on the original code. Commit `ec6384d` scopes supported mixed clauses. Tests check the €15,200 rejection, the €15,000 ceiling, missing amounts and explicitly optional charges. Commit `02e18f3` verifies the rejection in CLI output. |
| R4-002 · HIGH · P0 | The PDF requires Claude Code. Codex wrote the code at the candidate's direction. The historical deviation is documented in NOTES and the assessment log. No Claude Code or GSD evidence was manufactured. |
| R4-003 · MEDIUM · P1 | NOTES now discusses the concrete R2 diff `1bac90c`. A rendered A4 page was checked. |
| R4-004 · MEDIUM · P1 | Clean-archive major gate and reproducibility completed in [TASK-008](TASK-008-validation.md#post-assessment-validation). The test checks prices and expansion per venue block. |
| R4-005 · HIGH · P1 | The optional container again passed its smoke test. The offline Trivy rescan still found two high findings in the pinned runtime base with no fixed version listed in that report. No base or dependency change was made. |

The fresh original-data run keeps v004 outside the shortlist, reports v029's unresolved USD price and preserves the original recommendation order. The technical local handoff has fresh evidence. Full compliance with the original process cannot be claimed.
