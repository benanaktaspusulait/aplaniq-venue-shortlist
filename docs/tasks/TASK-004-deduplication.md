# TASK-004 — Venue Deduplication

## Status

To do.

## Objective

Identify repeated venue listings conservatively and expose conflicts and unresolved identity.

## Business justification

A shortlist can count the same venue twice or hide booking constraints if overlapping records are merged without evidence.

## Dependencies

[TASK-003](TASK-003-normalisation.md) for normalised records and source references.

## Requirements and scope

Apply the adopted identity and conflict policy to the small export. Preserve traceability to every listing. Keep possible matches distinct from matches strong enough to consolidate. Produce stable group IDs or stable suspected-match warnings.

## Explicitly out of scope

Fuzzy matching infrastructure, geocoding, external searches and filling facts from the most favourable listing.

## Expected files and modules

`src/dedup.rs`, `src/lib.rs` and focused unit tests.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Use evidence such as known name variants, location and station aliases with small deterministic comparisons. Check that room or building qualifiers can denote distinct inventory. Preserve source-specific evidence and expose disagreements for hard fields. An omitted field must not erase adverse evidence. Similar names alone cannot prove the same event space. Document any dataset-specific identity override and its basis.

## Acceptance criteria

- Given the three audited pairs, each is flagged or handled under the recorded identity policy and both IDs remain visible.
- Given a confirmed match with conflicting price or capacity, conflicting evidence reaches evaluation unless the approved rule explicitly resolves it.
- Given distinct venues with similar names, unrelated records remain independent. Reordering records preserves grouping and diagnostic order.

## Required tests and edge cases

Spoorhuis v004/v005, Domzicht v009/v010, Moreelse v023/v024, synthetic similar-name distinct rooms, conflicting evidence, unknown on one side, plus reversed input order.

## Validation commands

`cargo fmt --all -- --check`, `cargo test --lib`.

## Definition of done

Identity decisions and field conflicts are auditable in tests and downstream data. Review and commit locally.

## Risks and unresolved assumptions

No supplier address proves identity for the three pairs. See PD-006 and AC-10. The Moreelse event floor might be separate inventory. Conservative warnings may be the only reliable treatment.

## Estimated effort

10 minutes.

## Suggested Git commit messages

`feat: flag duplicate listings and preserve conflicting evidence`
