# R3 business logic review

**NO-GO for TASK-007.** R3-001 can admit a venue whose documented compulsory charge puts it above the working cap. Fix it in the evaluator before the CLI consumes the shortlist. The remaining findings are P1 and can be resolved before submission.

## Baseline and method

Reviewed `8251f03..a30101b`, including the R2 corrections, eligibility, ranking, tests and documentation. The workspace export matches the original assessment attachment byte for byte. Codex used a temporary copy of the committed baseline and a separate raw-data calculation, including exact decimal arithmetic. It compared all 60 venue-scenarios, checking seating, breakouts, budget, access, travel, daylight, Wi-Fi, all three date outcomes and identity flags. None of the reported verdicts was used as a reference input. The same Codex session implemented the baseline. No independent reviewer was used.

| Attendees | Eligible | Requires confirmation | Rejected |
| ---: | ---: | ---: | ---: |
| 80 | 3 | 19 | 8 |
| 120 | 1 | 16 | 13 |

The baseline matched the source-derived reference for every verdict, criterion, date, identity flag and supported complete quote. No false-positive or false-negative recommendation was demonstrated in the original 30 records. The findings below were reproduced with separate, otherwise viable source records.

## Ranking and expansion

| Rank | Listing | Recalculated 80-person package | Recalculated 120-person package and outcome |
| ---: | --- | ---: | --- |
| 1 | v019 De Stadstuin | 80 × €79 = €6,320 | 120 × €79 = €9,480. Rejected at 85 seats. |
| 2 | v028 Zaal Overvecht | 80 × €38 = €3,040 | 120 × €38 = €4,560. Rejected at 100 seats. |
| 3 | v021 Fabriek 1912 | 80 × €68 = €5,440 | 120 × €68 = €8,160. Eligible at 130 seats. |

All three list VAT and catering included. These are preliminary package amounts taken from the source. All three support 11, 16 and 18 March 2027 at day level. Those dates fall on Thursday, Tuesday and Thursday. The earliest supported Thursday is 11 March. Event times and booking remain unverified.

The [implemented lexicographic key](../../src/rank.rs#L60) compares supported preferred weekday, documented transport, daylight, Wi-Fi, comparable EUR headroom, expansion and stable identity. v019 wins on the documented 12-minute Centraal walk. v028 and v021 share the same transport and amenity bands, so v028's €11,960 headroom beats v021's €9,560 before expansion is considered. This follows PD-007.

An expansion-first alternative would compare supported weekday, 120-person verdict and comparable 80-person headroom before transport and amenities. It yields v021, v028, v019. v021 is the only resizeable candidate and costs €880 less than v019 at 80. Under that policy the short Centraal walk and stronger daylight lose priority. Keep PD-007 for the primary 80-person list and prominently identify v021 as the single option that currently supports 120. Promote expansion above the other preferences only if the planner confirms a single booking must accommodate both attendance outcomes. The client has not approved either trade-off.

## Verified findings

| Finding | Evidence and business impact | Minimum correction and regression |
| --- | --- | --- |
| **R3-001 · HIGH · P0** — mandatory costs are lost, PD-002 / AC-04–05. [Budget evaluation](../../src/evaluate/budget.rs#L64) | €14,900 with VAT and catering included plus the note `An additional AV rental fee is required and costs €300.` returned a complete €14,900 package and `Eligible`. The payable lower bound is €15,200 if the amount is comparable. Separately, `There are no mandatory fees.` incorrectly produced `MandatoryFees` and `RequiresConfirmation`. The phrase checks ignore word order and negation. | Recognise explicit required charges and absence. Keep an unpriced or unsupported mandatory charge unresolved. Add a known, compatible additional charge with checked arithmetic. Regress required €300 at €14,900, required charge with unknown amount, and explicit absence of charges. Assert both budget evidence and shortlist status. |
| **R3-002 · MEDIUM · P1** — explicit darkness loses its hard-failure status, PD-007 / AC-11. [Normalisation](../../src/normalize.rs#L421) | `daylight: "dark"` returned an unsupported-field diagnostic and `RequiresConfirmation`. PD-007 explicitly treats darkness in the plenary as a hard failure. The source stays outside eligible results, but the planner sees an unnecessary follow-up. | Normalise the explicit adverse term to unacceptable. Regress darkness, already-supported windowless plenary, uncertain partial light, and an adverse restriction confined to an optional room. |
| **R3-003 · MEDIUM · P1** — equivalent date restrictions produce different verdicts, PD-004 / AC-08. [March restriction parser](../../src/normalize/availability.rs#L127) | `March 2027: Thursdays only` returned three uncertain dates because an exception is required by the month-first parser. `Thursdays only March 2027` returned Thursday pass, Tuesday exclusion, Thursday pass. A viable Thursday option can be unnecessarily withheld. Existing `Tue only March 2027` coverage passes. | Share weekday-only handling between month-first and weekday-first forms. Regress both word orders for Tuesday and Thursday, the existing March 18 exclusion, and an ambiguous monthless phrase that must stay unresolved. |
| **R3-004 · MEDIUM · P1** — inclusive access with an explicit exception for all breakouts is understated, PD-003 / AC-06. [Room restriction](../../src/evaluate/rules.rs#L157) | `wheelchair_accessible: "yes, except all breakout rooms are stairs-only"` became conditional. The generic `except` token incorrectly treated the restriction as partial. With three required breakouts and no accessible alternative, the access criterion must reject. | Distinguish an exception that covers an entire required room category from an optional or partial restriction. Regress all breakouts, an explicitly inaccessible plenary, and some optional stairs-only breakouts. |
| **R3-005 · LOW · P1** — aggregate confirmation cause absent, AC-12. [Scenario date](../../src/evaluate.rs#L64) · [Aggregation](../../src/evaluate.rs#L189) | Two confirmed listings with passing dates on different days correctly return conditional. Each listing exposes its reasons but the aggregate date exposes only a verdict. A renderer must reconstruct why the entity is conditional to distinguish a source disagreement from material identity uncertainty. | Expose the confirmation cause when combining verdicts. Regress a confirmed group with disjoint passing dates, an ordinary listing with unknown availability, and a potential pair that would otherwise pass. Consume the cause in presentation. |

## Gate, integrity and CLI readiness

- `sh scripts/quality-gate.sh --major` passed. Inspected the script. It runs fmt, warning-denied Clippy, locked all-target check, debug tests, release build and tests, warning-denied Rustdoc and the Rust 1.85.0 locked check and tests when the pin differs. Pin was Rust 1.100.0. Debug, release and MSRV each passed 29 unit and 24 integration tests, with no failures, ignored tests or doc tests. `cargo-audit`, `cargo-deny` and `cargo-llvm-cov` were unavailable.
- Existing assertions check standing versus seated boundaries, exact €15,000 and one cent over, partial costs and overflow, date exclusions, incompatible scope, duplicate grouping, shortlist partitioning and comparable headroom transitivity. Tests cover an empty input and mixed noneligible input. Separate all-conditional and all-rejected output fixtures belong in TASK-007. The table specifies the missing business regressions.
- Rechecked duplicate row and nested key rejection with preserved neighbouring rows, named-file error context and both v007 qualifiers. v004/v005, v009/v010 and v023/v024 preserve six identities and three warnings. The rejecting v004 cannot enter the shortlist. v005 stays in follow-up based on its own evidence. There is no cross-listing assembly of passing costs, rooms or dates.
- Reversed and seeded-shuffled original rows produced the same order, verdicts and ranking factors. The key compares only supported dates and agreed complete prices. Known zero headroom correctly precedes absent headroom. Reused IDs use semantic claim fields before source position. The final formatting-based comparison allocates only on identity collisions. No instability or performance defect was demonstrated for the pinned toolchain and current workload.
- The single Cargo package is acyclic across ingestion, typed evidence, eligibility and ranking. The locked graph reported no duplicate package versions. The three direct dependencies serve date validation and JSON evidence. Code review and the gates found no first-party unsafe code, unjustified lint suppression or production panic in the assessed paths. Separate requirement checks retain reasons even when one fails.
- The public API supplies the three lists, dates, costs and their completeness, factors, both attendance verdicts, evidence and diagnostics. Source counts are available from ingestion and deduplication. TASK-007 can render these without a new pricing or eligibility engine. After the P0 correction, show package amounts and day-level availability as preliminary, and add the aggregate reason from R3-005 before final submission.

Only review documentation changed. No policy or application code was changed.


## Correction status

All five findings were corrected and covered by eligibility regressions before TASK-007. Known fixed additional EUR charges now contribute to the mandatory lower bound, and unresolved scope, conflicting claims or VAT treatment remain visible. The date parser accepts equivalent month-first and weekday-first restrictions, explicit darkness and all-room access exclusions are hard failures, and the evaluator exposes a typed cause for aggregate date confirmation. The original dataset and PD-007 remain unchanged.
