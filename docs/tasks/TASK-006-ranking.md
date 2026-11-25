# TASK-006 — Ranking and Recommendation Engine

## Status

To do.

## Objective

Order eligible venues deterministically and record why they appear in the shortlist.

## Business justification

Planners need to compare the merits of confirmed options and understand the reasons for their order.

## Dependencies

[TASK-005](TASK-005-eligibility.md) for verdicts and evidence. Use the priorities recorded by TASK-001.

## Requirements and scope

Apply the approved preferences, such as Thursday, rail access, daylight, Wi-Fi or comparable cost. Enforce the agreed shortlist limit after ranking. Return readable ranking factors in the typed report, with stable tie-breaking and diagnostic order.

## Explicitly out of scope

Re-evaluating hard criteria, personalised or learned scores, CLI presentation and external price or transport lookups.

## Expected files and modules

`src/rank.rs`, `src/lib.rs`, the report type where needed and ranking tests.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Apply the approved ordering in PD-007 and verify AC-11–12. The proposal uses ordered factors and a separate confirmation list. Use a stable source or group ID as the final key. Carry relevant evidence through to the report.

## Acceptance criteria

- Given an otherwise tied eligible Tuesday and Thursday option, Thursday comes first if that preference is adopted. A rejected venue with stronger preferences stays out of the confirmed list.
- Given tied eligible venues, reruns and input permutations produce the same order by the documented tie-break.
- Given a shortlisted entry, the report exposes its ranking factors and hard-criteria rationale. Any optional confirmation results remain distinguishable.

## Required tests and edge cases

Primary preference, precedence between competing preferences, ties, permutation, any output limit, empty eligible set, unknown preference evidence and a rejected high-scoring venue.

## Validation commands

`cargo fmt --all -- --check`, `cargo test --lib`.

## Definition of done

Documented ranking factors, ordering and traceable explanations have passing tests. Review and commit locally.

## Risks and unresolved assumptions

Preference priorities, distance proxy, effect of missing preference evidence and shortlist length need human agreement.

## Estimated effort

9 minutes.

## Suggested Git commit messages

`feat: rank eligible venues and explain ordering`
