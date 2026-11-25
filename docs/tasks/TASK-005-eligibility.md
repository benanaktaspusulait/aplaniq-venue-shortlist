# TASK-005 — Eligibility and Business Rules

## Status

Complete. The original export passed the two-scenario regression.

## Objective

Evaluate the approved hard requirements and explain each candidate's verdict.

## Business justification

A venue that misses a required room, budget, date or access condition cannot be recommended as confirmed.

## Dependencies

[TASK-003](TASK-003-normalisation.md) for typed evidence and [TASK-004](TASK-004-deduplication.md) for identity and conflict handling. Apply TASK-001 policy.

## Requirements and scope

Evaluate agreed headcount, setup, simultaneous breakouts, dates and hours, accessibility, budget and any hard rail or amenity requirements. Produce `Eligible`, `RequiresConfirmation` or `Rejected` with criterion-level evidence. Use compatible currency and price basis, checked arithmetic and the approved treatment of VAT, catering and minimum spend.

## Explicitly out of scope

Preference scoring, CLI formatting, unsupported price conversion, booking and internet verification.

## Expected files and modules

`src/evaluate.rs`, `src/lib.rs` and focused evaluator tests. An additional small price helper is justified only if the evaluator needs it.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Apply PD-001–007, AC-03–10 and AC-15. Distinguish preliminary eligibility from booking verification. Run the same hard checks separately at 80 and 120. Evaluate all hard criteria together for each allowed date. A hard failure rules out that date and a date-independent failure rules out all dates. Make the reference date explicit for lead time. Keep an incomplete estimate unresolved when it cannot prove a pass. A proven lower bound can reject if already above a firm cap.

## Implementation plan

Implement pure checks for whole source listings, typed budget evidence and per-date outcomes. Aggregate dates within each attendance scenario and compare decisions across confirmed source configurations. Carry potential identity pairs to follow-up without transferring evidence. Verify the original export and an equivalent synthetic Stadstuin case.

## Acceptance criteria

- Given all hard criteria pass, return `Eligible` and supporting reasons. Given one hard failure and otherwise passing or unknown evidence, return `Rejected`. Given no failures and an unresolved hard criterion, return `RequiresConfirmation`.
- Given a firm inclusive cap and known comparable totals at and above the cap, apply the documented boundary exactly. Unknown required charges or currency never silently establish a pass. A documented positive access flag is preliminary evidence, and explicit unacceptable lighting or Wi-Fi fails.
- Given Tuesday evidence and a conflicting Thursday offer, a candidate qualifies only if one allowed date satisfies all related hard requirements together.

## Required tests and edge cases

80 and 120, plenary layouts, minimum breakout boundary, stairs-only breakout, unknown access, v002 date exception, v006 lead time, v018 closure, VAT/catering/minimum spend, quote at and above cap, incomplete or non-EUR price, and a conflict on a hard field.

## Validation commands

`cargo fmt --all -- --check`, `cargo test --lib`, `cargo clippy --all-targets -- -D warnings`.

## Definition of done

The approved policy has a tested decision table and actionable per-criterion evidence. Run the checks, inspect the diff and commit locally.

## Observed outcome

The original export produced 3 eligible, 19 conditional and 8 rejected entities for 80 attendees. At 120 it produced 1 eligible, 16 conditional and 13 rejected entities. De Stadstuin passes at 80 with a stated €6,320 package and fails seating at 120, where the same rate gives €9,480. An equivalent synthetic listing repeats both verdicts. Run `sh scripts/quality-gate.sh` and `cargo test --locked --test eligibility original_export_and_synthetic_stadstuin -- --nocapture` to reproduce the gate and distributions.

## Risks and unresolved assumptions

Date granularity, verified route coverage and all-in prices are incomplete in the export. Honour the evidence threshold even when the confirmed shortlist is empty.

## Estimated effort

27 minutes.

## Suggested Git commit messages

`feat: evaluate hard venue requirements with reasons`
