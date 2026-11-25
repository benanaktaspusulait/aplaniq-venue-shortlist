# Rust standards audit

**25 November 2026.** Codex audited the repository through TASK-003 and applied engineering configuration, Rustdoc and behaviour-preserving lint corrections. TASK-004 is the next business implementation task and was not started. The original assignment, candidate-approved policies, source dataset and lockfile were preserved. Codex did not perform a separate independent R2 review or use Claude Code for this audit. Historical Codex business implementation remains disclosed.

## Baseline and final validation

Baseline `845af5f` had no exact toolchain pin, package lint tables, shared gate script or hosted workflow. The existing fmt, default Clippy with warnings denied, all-feature check and tests, release build and tests, and warning-denied rustdoc passed on Rust 1.100.0. Debug and release each ran 23 unit tests and one integration test. The first strict lint run exposed missing public Rustdoc, which dominated the output, plus pure accessor annotations, readability warnings and two long input-parser branches. The errors were resolved without blanket lint exclusions. Initial refactoring needed a return cleanup and a test import correction.

| Check | Required | Status | Actual evidence and limitation |
| --- | --- | --- | --- |
| Format | Mandatory | PASS | Pinned `cargo fmt --all -- --check`. |
| Compiler and Clippy | Mandatory | PASS | Pinned all-target, all-feature locked check and Clippy with `-D warnings`. |
| Normal tests | Mandatory | PASS | 23 unit + 1 integration + 0 doc tests, 0 failed, 0 ignored. Empty input and empty venue array are asserted separately. |
| Release build and tests | Major | PASS | `cargo build --release --locked` and release tests, 23 unit + 1 integration + 0 doc tests, 0 failed, 0 ignored. |
| Rustdoc | Major | PASS | `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features`. |
| MSRV 1.85.0 | Conditional, differs from pin | PASS | Locked all-target check and tests on installed 1.85.0, 23 unit + 1 integration + 0 doc tests, 0 failed, 0 ignored. |
| Feature combinations | Conditional | NOT APPLICABLE | The package has no declared features. Dependency features are in the reviewed graph. |
| Supply-chain advisory and licence policy | Conditional tool availability | BLOCKED | `cargo-audit` and `cargo-deny` absent. Neither automated scan ran. |
| Coverage | Conditional tool availability | BLOCKED | `cargo-llvm-cov` absent. No line or branch percentage measured. |
| Hosted CI | Conditional hosted execution | BLOCKED | Workflow YAML, shell syntax, checkout pin and local major gate verified. No remote configured or hosted run observed. |
| Docker | Conditional, TASK-007B | NOT APPLICABLE | No image or Dockerfile exists. Client 29.9.2 is present. The daemon connection failed, so container verification cannot yet run. |

## Compliance matrix

| Requirement | Applicability | Status | Evidence | Correction or remaining action | Priority |
| --- | --- | --- | --- | --- | --- |
| 0 · Audit inputs, baseline and scope | Mandatory | PASS | Original PDF, approved requirements, architecture, source, tests, manifests, task states, staged diff and history inspected. No GSD or Claude artifacts found. | Continue with TASK-004 only when requested. | High |
| 0 · Assessment implementation agent | Mandatory delivery | FAIL | Earlier business commits used Codex, as disclosed in the log. This phase used Codex for standards and review only. | Use Claude Code for subsequent business changes, keep the historical deviation visible. | High |
| 0 · Claude Code readiness | Mandatory for future business work | BLOCKED | Isolated Claude Code 2.1.332 reports logged out. No implementation session ran. | Authenticate Claude Code before starting the next business task. | High |
| 1 · Persistent governance | Mandatory | PASS | Canonical standards, both root agent files and gate documentation linked. | Record actual plans, agents and results. | High |
| 2 · Stable pin and honest MSRV | Mandatory | PASS | 1.100.0 exact pin, rustfmt and Clippy, Edition 2024, 1.85.0 check and tests passed. | Re-run MSRV when compiler features or locked dependencies change. | High |
| 3 · Package and module boundaries | Mandatory | PASS | One package, library and thin binary. Ingestion uses normalisation and shared diagnostics, domain has no I/O or JSON dependency. | Keep recommendation and future CLI boundaries explicit. | Medium |
| 4 · Idiomatic domain design | Mandatory | PASS | Validated bounds, typed identifiers and evidence enums. Plain structs and pure transforms serve the current pipeline. | Introduce behaviour or traits only for demonstrated needs. | Medium |
| 5 · Data and money invariants | Mandatory | PASS | Positive attendance and seats, distinct standing and unresolved evidence, exact minor units and checked conversion. Tests cover overflow and unknown amounts. | Apply compatible budget and currency rules in TASK-005. | High |
| 6 · Ownership and borrowing | Mandatory | PASS | Normalisation borrows parsed values; owned evidence outlives JSON. Explicit lifetimes relate borrowed input to diagnostics or return values. | Keep source evidence and avoid speculative shared ownership. | Medium |
| 7 · Unsafe prohibition | Mandatory | PASS | Verbose Cargo compiler calls passed `--forbid=unsafe_code` to library, binary and test target. | No local suppression. Dependency unsafe inventory is separate. | High |
| 8 · Error handling and context | Mandatory | PASS | Named-file parse errors retain path, cause and position. See STD-002. | Verify stderr and exit code through the CLI in TASK-007. | Low |
| 9 · Public API and Rustdoc | Mandatory | PASS | Focused exports, constructor and field contracts, documented `InputError`, warning-denied rustdoc. | Preserve import and conversion contracts. | Medium |
| 10 · Dependency minimisation and reproducibility | Mandatory | PASS | Locked graph and feature tree reviewed. No duplicate versions reported across targets. Direct crates use MIT OR Apache-2.0 and compile at MSRV. The R2 correction added a direct edge to the already locked serde_core without adding a package version. | Inspect the graph and licence/advisory status before dependency changes. | High |
| 10 · Advisory and licence policy scan | Conditional | BLOCKED | `cargo-audit` and `cargo-deny` absent. Metadata review is limited to the graph and available registry data. | Run scans when installed and authorised. | Medium |
| 11 · Proportional resource use | Mandatory | PASS | Synchronous local input, proportional allocations and no extra runtime or profile flags. Release tests match debug outcomes. | Keep the small-export scope. | Medium |
| 12 · Record accounting and provenance | Mandatory | PASS | Dataset integration accounts for all 30 source rows and records diagnostics. Byte comparison confirms unchanged original. | Retain source IDs and claims through deduplication. | High |
| 12 · Conflicting JSON properties | Mandatory evidence preservation | PASS | Repeated properties produce an envelope error or a source-linked row rejection before normalisation. See STD-001. | Keep ordinary row accounting and decimal precision covered. | High |
| 12 · Qualified amenity evidence | Mandatory evidence preservation | PASS | Daylight and Wi-Fi qualifications survive normalisation and confirmed grouping. See R2-001. | Apply room scope during eligibility. | Medium |
| 13 · Tests for implemented behaviour | Mandatory | PASS | Unit and integration suites cover parsing, evidence and dataset processing. Critical selection cases remain assigned to later tasks. | Add business cases as their functions land. | High |
| 13 · Full v019, deduplication and CLI tests | Conditional on implementation | PASS through TASK-006 | Original export and synthetic fixtures cover grouping, both eligibility scenarios and ranking. | Verify the final CLI and complete end-to-end scenarios in TASK-007/008. | High |
| 14 · Test quality and traceability | Mandatory | PASS | Behaviour-focused local tests and the mapping below. No ignored tests or golden snapshots used. | Extend mapping with actual selection and CLI tests. | Medium |
| 14 · Actual coverage | Conditional | BLOCKED | Tool absent. No line or branch measurement. | Measure when available and useful. | Low |
| 15 · Compiler and Clippy enforcement | Mandatory | PASS | Package lint tables and test-only Clippy settings verified on the pin. Expected-failure probe denied production unwrap and test dbg, while test unwrap was accepted. | Keep warnings as errors and document any future exception. | High |
| 16 · Public documentation and contracts | Mandatory | PASS | Warning-denied Rustdoc and local Markdown links verified. README identifies scaffold behaviour. | Document the working CLI command and stderr contract in TASK-007. | Medium |
| 17 · Docker distribution | Conditional | NOT APPLICABLE | Optional task is unstarted, server connection unavailable. | Validate pinned builder, linkage and runtime at TASK-007B only. | Low |
| 18 · Reproducible quality gate | Mandatory | PASS | Shared fail-fast script passed in normal and major mode. Invalid arguments exit 2. | Keep gate command list and CI aligned. | High |
| 19 · CI definition | Conditional, in scope | PASS | Read-only workflow reuses the major script. External checkout pinned to the verified upstream commit. Local YAML and shell syntax checked. | Verify hosted execution separately after authorised setup. | Medium |
| 19 · Hosted CI execution | Conditional | BLOCKED | No configured remote or hosted run. | Record the first real hosted result when available. | Medium |
| 20 · Review checkpoints | Mandatory | PASS | R1–R4 retained in backlog. This audit is Codex self-review of its standards changes and audit of prior work, with no independent completion claim. | Run R2 after TASK-004. | High |
| 21 · Atomic commits | Mandatory | PASS | Exact-path Conventional Commits and staged/diff checks. Existing indexed `.gitignore` preserved. | No history rewrite or push. | High |
| 22–23 · Audit and final evidence | Mandatory | PASS | Baseline, configuration corrections, actual outputs and deferred findings recorded here and in the log. | Keep statuses evidence-based. | High |

<a id="unresolved-findings"></a>

## Findings

| ID and severity | Location and evidence | Impact | Correction and required regression | Status |
| --- | --- | --- | --- | --- |
| STD-001 · High · P0 | [JSON parse boundary](../../src/ingest.rs#L96). In a two-row fixture, repeated access `false → true`, Wi-Fi `poor → fiber`, capacity `60 → 160` and a €18,000 → €9,000 whole price or nested amount each returned the last claim, two normalised rows and zero diagnostics. Reversing access returned the opposite claim. | A malformed listing can become an apparently eligible candidate. Deduplication receives only the winning field. The original 30 listings contain no repeated keys. | Detect repeated keys before object-map insertion, including nested fields. Reject the ambiguous row with its index, reliable ID where available and repeated key path. Retain other rows and count the rejected row once. Reject a repeated root `venues` key as an envelope error. Regress both value orders, whole and nested prices, Wi-Fi, capacity, root duplication and a valid neighbouring row. Preserve exact decimal and overflow tests. | Resolved by the R2 correction and regression tests. |
| STD-002 · Low · P1 | [File parse wrapper](../../src/ingest.rs#L82). Loading `broken.json` containing `{\n  "venues": [}` returned `expected value at line 2 column 14`. The cause was available through `Error::source()`. The path was absent from the error. | A caller can see the parse position but cannot identify which file failed from the error alone. | Add path context at the named-file boundary and retain the original cause. Regress a named malformed file with path, cause, line and column, and verify CLI stderr and a nonzero exit in TASK-007. | Resolved at the library boundary. CLI verification remains in TASK-007. |
| R2-001 · Medium · P0 | [Daylight and Wi-Fi normalisation](../../src/normalize.rs#L393). The [v007 source](../../venues.json#L112) names casemates in both amenity restrictions. The public API returned `Known(Partial)` for both and no associated diagnostics. Notes only mention the shuttle. A synthetic windowless plenary under the same prefix also returned `Partial` with no diagnostic. | Eligibility cannot distinguish a restriction in optional rooms from a hard failure in the required plenary. | Keep the raw qualifier with the corresponding typed claim or field diagnostic. Regress v007 with both restrictions and source locations, a windowless required plenary, and a restriction in an optional room. Keep uncertain room coverage pending and preserve the existing explicit negative cases. | Resolved by the R2 correction and regression tests. |

Fixed standards gaps include the exact compiler pin, mandatory lint policy and public Rustdoc, shared reproducible gate and minimal CI wiring. They were verified through actual compiler and script output. No new runtime dependency or business policy was introduced.

## Requirements-to-tests traceability

| Criteria | Existing evidence | Remaining task |
| --- | --- | --- |
| AC-01 and AC-02 | `original_export_accounts_for_every_source_row_and_preserves_evidence`, `document_errors_distinguish_read_syntax_and_envelope` and `invalid_rows_are_accounted_for_and_later_rows_still_load` verify accounting and error classification. | Positive eligibility fixture and CLI stderr/status in TASK-005/007/008. |
| AC-03 to AC-05 | [Eligibility integration tests](../../tests/eligibility.rs) verify both attendance thresholds, exact cap and one-cent overflow of the cap, unknown costs and FX, minimum spend and arithmetic overflow. | Verify displayed budget evidence in TASK-007/008. |
| AC-06 to AC-09 | Eligibility tests cover required-room access, scope, date exclusions and lead time, transport uncertainty and the walking heuristic. | Verify rendered booking checks in TASK-007/008. |
| AC-10 | [Deduplication integration tests](../../tests/deduplication.rs) verify all three suspected pairs, distinct spaces, complete record accounting, typed conflicts and retention of adverse evidence. | R2 verified grouping. Test downstream handling of linked records and complete end-to-end coverage in TASK-005, TASK-006 and TASK-008. |
| AC-11 to AC-14 | [Ranking tests](../../tests/ranking.rs) verify approved precedence, ties, date support, typed explanations, separate outcomes and an empty shortlist. | Verify the working command, rendered explanations and delivery in TASK-007–010. |
| AC-15 | Original export and synthetic eligibility tests verify the 80/120 verdicts and costs. The ranking integration verifies the observed shortlist order. | Verify CLI display end to end in TASK-008. |
