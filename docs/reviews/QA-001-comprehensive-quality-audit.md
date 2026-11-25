# QA-001 comprehensive quality audit

Post-assessment audit of `2ed12848e175b5a6952c7ae80c732c9dc8f02621`. The assessment stop is `35570b4d3b29e72d1961d494315ea1d51f579ec5`. Codex performed the audit and the authorised follow-up. The original timed assessment and its agent deviation are documented in the assessment log.

## Baseline evidence

The major gate passed in an isolated archive. Debug, release and Rust 1.85.0 each passed 29 unit and 34 integration tests. The original export accounted for 30 rows. At 80 attendees there were 3 eligible, 19 conditional and 8 rejected. At 120 there were 1, 16 and 13 respectively. The eligible order was v019, v028, v021. Native and release output and diagnostics matched. Invalid JSON and UTF-8 exited 1, unsupported arguments 2 and a valid empty export 0. The CLI returned €14,900 and Eligible for the R4 fee fixture at `35570b4`, and €15,200 and Rejected at `2ed1284`.

The inventory counted 4,130 physical production lines (3,395 excluding comments and blanks), 141 named production functions and methods, 56 public type definitions and 21 public functions and methods. Test code accounted for 2,488 physical lines. The three largest production files were report (612), domain (477) and normalize (450). The longest functions were evaluate/rules availability (76), normalize/availability parse (75) and evaluate scenario (75). The library borrows whole source configurations through deduplication, eligibility and ranking. The binary owns I/O and rendering. The package, safe Rust constraints and exact monetary types are sound. Free-text scope, API context and tie ordering are the main weaknesses.

| Module | Production lines | Functions | Public types / functions | Responsibility |
| --- | ---: | ---: | ---: | --- |
| lib | 9 | 0 | 0 / 0 | Public module surface |
| domain | 477 | 13 | 31 / 13 | Evidence and invariants |
| diagnostic | 33 | 0 | 2 / 0 | Source diagnostics |
| ingest | 172 | 5 | 2 / 2 | Input errors and row accounting |
| ingest/keys | 91 | 6 | 0 / 0 | Duplicate members |
| normalize | 450 | 24 | 0 / 0 | Source field parsing |
| normalize/availability | 286 | 15 | 0 / 0 | Date grammar |
| normalize/charges | 129 | 5 | 0 / 0 | Mandatory fees |
| normalize/price | 157 | 8 | 0 / 0 | Exact prices |
| dedup | 142 | 3 | 4 / 3 | Entity grouping |
| dedup/identity | 128 | 8 | 0 / 0 | Identity comparisons |
| dedup/conflict | 139 | 4 | 1 / 0 | Conflicting source fields |
| evaluate | 329 | 6 | 8 / 2 | Coherent listing and scenario verdicts |
| evaluate/budget | 207 | 5 | 3 / 0 | Budget evidence |
| evaluate/rules | 371 | 7 | 0 / 0 | Hard requirements |
| rank | 301 | 8 | 5 / 1 | Deterministic order |
| main | 97 | 3 | 0 / 0 | CLI entry and status |
| report | 612 | 21 | 0 / 0 | Report and diagnostic formatting |

Rust 1.85.0 instrumented coverage was 94.38% line, 96.55% function and 85.82% region across first-party source including inline unit tests. Branch counters were unavailable. The pinned compiler profile could not be merged by the installed Apple LLVM tools. No mutation campaign ran. Offline Trivy found no Cargo advisory or secret finding. The prior Linux arm64 image rescan retained two HIGH findings in the runtime OpenSSL package, with no fixed version identified by that scan. No amd64 or hosted run was verified in QA-001.

Baseline synthetic release medians with 60, 180 and 360 distinct entries were 6.8, 9.6 and 19.5 ms. Dense potential-match medians were 9.7, 36.5 and 151.0 ms. Three runs per data point included startup and rendering.

## Findings and corrective backlog

| ID | Severity | Source and evidence | Acceptance |
| --- | --- | --- | --- |
| Q01 | HIGH | [Fee VAT](../../src/normalize/charges.rs), explicit VAT exclusion wrongly accepted at €15,000 | Excluded or ambiguous fee VAT keeps the total provisional. Known over-cap floor still rejects. |
| Q02 | HIGH | [Transit rule](../../src/evaluate/rules.rs), a negated bus stop became a positive connection | Missing stop alone remains uncertain. Documented alternatives and explicit lack of all transport retain their outcomes. |
| Q03 | MEDIUM | [Normalisation](../../src/normalize.rs), required inaccessible rooms and windowless plenary lost explicit adverse evidence | Required-room failures reject. Partial or optional scope retains uncertainty. |
| Q04 | MEDIUM | [Capacity parsing](../../src/normalize.rs), unrecognised layout erased valid theatre seats | Keep valid layouts and per-layout diagnostics without treating missing layouts as impossible. |
| Q05 | MEDIUM | [Ranking API](../../src/rank.rs), stale assessment can be combined with another request | One request drives assessment and ranking and the public boundary protects the result. |
| Q06 | MEDIUM | [Ranking comparator](../../src/rank.rs), unstable Debug format and repeated allocations | Compare typed fields and preserve permutation independence. |
| Q07 | MEDIUM | [Evaluation](../../src/evaluate.rs), global potential-pair scan for each entity | Index by listing and preserve warning and verdict order. |
| Q08 | MEDIUM | [File input](../../src/ingest.rs), no resource bounds | Bound named-file reads, preserve context and test the byte boundary. |
| Q09 | MEDIUM | [Eligibility tests](../../tests/eligibility.rs), missing assertions for additive overflow and specificity conflicts | Add independent boundary and date-conflict assertions. |
| Q10 | LOW | [Domain](../../src/domain.rs) and [renderer](../../src/report.rs), unused public report types and ambiguous rendering flags | Remove unused surface and use explicit rendering modes under unchanged CLI output. |
| Q11 | HIGH | [Container](../../Dockerfile), CVE-2026-107494 and CVE-2026-124709 in libssl3t64 | Verify scan and mitigation on a compatible digest. Document residual exposure and runtime controls. |

QA-002 addresses Q05 and Q10. QA-003 addresses Q01–Q04 and Q06. QA-004 addresses Q09. QA-005 addresses Q08 and Q11. QA-006 addresses Q07. QA-007 verifies the complete change with a clean major gate, original export comparison, Docker smoke and a separate review. See the [standards matrix](../engineering/standards-compliance-matrix.md).
