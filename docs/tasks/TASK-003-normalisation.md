# TASK-003 — JSON Ingestion and Data Normalisation

## Status

To do.

## Objective

Read the supplied export and convert supported field shapes into typed evidence with traceable diagnostics.

## Business justification

The export mixes numbers, strings, objects and nulls. Coercion without provenance can incorrectly qualify a venue.

## Dependencies

[TASK-002](TASK-002-domain-model.md) for evidence and diagnostic types.

## Requirements and scope

Validate the JSON and venue array envelope. Normalise observed capacity, breakout count, price, availability, station and accessibility shapes. Preserve daylight, Wi-Fi and relevant notes as evidence. Distinguish a fatal document error from a recoverable field or record issue. Preserve the original dataset bytes.

## Explicitly out of scope

Deciding if evidence satisfies an event, choosing a duplicate winner, live venue verification, general natural-language parsing and guessing missing facts.

## Expected files and modules

`src/ingest.rs`, `src/normalize.rs`, `src/lib.rs`, targeted unit tests, and `Cargo.toml`/`Cargo.lock` for dependencies used here.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Use `serde_json` with a tolerant record boundary. Prefer explicit supported patterns and an unresolved fallback for unfamiliar text. Preserve amount precision before converting to minor units. Retain VAT, currency, estimate, minimum spend and rate basis. Preserve general access claims and room-level limitations. Parse dates and exceptions as evidence. Carry notes if they qualify structured fields.

## Acceptance criteria

- Given malformed JSON or a wrong envelope, loading reports an actionable document error. Given a single malformed record in a valid array, loading continues and reports its ID or index and field path.
- Given v004 and v017, the capacity evidence retains the separate layouts. Given v027, zero is invalid and the note is retained as unverified evidence.
- Given v002, v008 and v029, the decimal amount, grouped amount under the adopted policy and USD currency remain distinguishable. Given the whole export, all 30 records are accounted for as loaded or individually diagnosed.

## Required tests and edge cases

Malformed JSON, incorrect envelope, missing/null/type-mismatched fields, v002 and v005 range or qualified rooms, v003 and v017 text prices, v009 minimum spend, v026 explicit dates, v002 date exclusion, v006 lead time, v018 closure, unknown dates, v011 alternate transport and v017 partial access. Combine cases in table-driven tests where useful.

## Validation commands

`cargo fmt --all -- --check`, `cargo test --lib`, `git diff --exit-code HEAD -- venues.json`.

## Definition of done

The ingestion contract and representative dataset formats have passing tests and each unresolved conversion remains traceable. Review and commit locally.

## Risks and unresolved assumptions

Use the approved interpretations of grouped amounts and implied years. See AC-01–03, AC-05–09 and AC-15. Preserve v019’s whole-number capacity, per-person VAT-inclusive catering quote, positive access flag, daylight, fiber and station walk. Prioritise audited patterns. Unsupported text can be reported for confirmation.

## Estimated effort

30 minutes.

## Suggested Git commit messages

`feat: ingest venue export and normalise observed evidence`
