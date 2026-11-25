# Assessment log

## 25 November 2026, Phase 1

- OpenAI Codex wrote the project scaffold and prepared the requirements, data audit, architecture and test strategy. Claude Code is the implementation agent specified by the assessment. No venue-selection logic or business tests were implemented in this phase.
- The workspace already had a local Git repository on `master` with no commits and an existing staged `/.idea/` rule in `.gitignore`. The rule remains staged and was omitted from the Phase 1 commits. Added `/target/` to the local Git excludes to keep build output out of status.
- At the first recorded environment check (19:22:12 GMT), no usable Rust installation was found. Provisioned stable Rust, Cargo, rustfmt and Clippy in a temporary directory through rustup with the minimal profile and `--no-modify-path`. No global Rust or shell settings were changed. The host was arm64 macOS 26.6.2. Git was 2.43.0, rustc and Cargo were 1.100.0, rustfmt was 1.11.0-stable and Clippy was 0.1.100. The scaffold declares Rust 1.85 for edition 2024 and the checks ran on the installed temporary stable toolchain.
- The initial pasted brief did not contain the original source files. Committed provisional requirements and an audit plan while locating them. The user supplied the ZIP at approximately 19:24 GMT. Read both PDF pages and all 30 venue records, replaced provisional material with source-backed findings, and copied `venues.json` into the repository. Verified the tracked bytes against the ZIP. An initial PDF image and JSON audit command used an incorrect temporary path and was rerun with the correct path. Poppler reported Type 3 glyph bounding-box warnings during extraction. Both rendered pages were readable and checked against the extracted text.
- Documented the processing pipeline, alternatives requiring human review, three proposed client questions and the focused test plan. Kept the scaffold dependency-free and reserved data processing for Claude Code.
- All four required Cargo checks were rerun successfully by 19:25:37 GMT. This was 3 minutes 25 seconds after the first recorded environment check. This interval covers through validation and excludes earlier work before that timestamp and final documentation and commit checks.

## Validation

| Command or check | Observed result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --all-targets -- -D warnings` | Passed. |
| `cargo test` | Passed across the library, binary and doc-test targets. Zero tests exist at bootstrap. |
| `cargo check` | Passed. |
| `cargo run --quiet --` | Confirmed the scaffold message on stderr, empty stdout and the expected exit status 1. |
| Dataset validation | Parsed the complete JSON, checked duplicate keys and verified that the workspace and Git copies match the ZIP bytes. |

This work was local. No client questions were sent and no repository was pushed or shared.

## 25 November 2026, Phase 2

- Codex reread the original assessment and reviewed all 30 records, the Phase 1 documents, current Git history and the scaffold. Codex prepared the ten task specifications, the dependency graph, assessment traceability matrix and the Claude Code handoff. All implementation tasks remain to do. See the [backlog](tasks/README.md).
- The repository had no existing GSD or Claude planning artifacts. The `claude` executable was absent from the shell PATH at this check. No Claude Code or GSD execution was recorded. Later planning should use the task IDs in the genuine implementation plan and maintain one live task status.
- The first recorded Phase 2 repository check was at 19:26:36 GMT. Backlog and scaffold validation returned by 19:29:25 GMT, an observed interval of 2 minutes 49 seconds. Together with the 3 minutes 59 seconds observed in Phase 1, the recorded intervals total 6 minutes 48 seconds through Phase 2 validation. These intervals exclude work outside their start and end checks. The estimate for the remaining backlog is 141 active minutes within the overall 180-minute cap.
- Verified the required task fields, links, graph edges, dependency order, initial statuses and effort totals. Confirmed that the tracked Rust scaffold and dataset matched the Phase 1 baseline and that `venues.json` matched the ZIP entry. Re-ran `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` and `cargo check` successfully. The scaffold still has zero tests.
- Used local documentation commits. The pre-existing staged `.gitignore` was preserved. No client contact, push or sharing took place.

## 25 November 2026, TASK-001

- Codex reviewed the original assessment PDF, the unchanged ZIP dataset, Phase 1 and 2 documents, Git history and the repository's planning paths. The bootstrap and backlog were present. No Claude Code or GSD planning artifacts were present and neither agent workflow ran.
- Codex classified the client's requirements, drafted seven pending product decisions, prepared three unsent English client questions and mapped 14 acceptance criteria to scope and implementation tasks. Updated the downstream references, architecture and notes to use the requirements document as the policy source.
- Verified the March 2027 weekday set, representative venue evidence, task traceability and Markdown links. The first A4 render of `NOTES.md` clipped the decision table. Replaced the table with short bullets and verified that the resulting one-page A4 render was legible. The four Cargo scaffold checks passed, with zero tests. Confirmed that the Rust tree and original dataset bytes were unchanged.
- The first repository check was at 19:29:55 GMT. The specification commit was made at 19:33:08 GMT, an observed interval of 3 minutes 13 seconds excluding this log commit. Record actual implementation time separately within the 180-minute assessment cap.
- TASK-001 is ready for human review. PD-001–007 await approval and TASK-002 has not started. The pre-existing staged `.gitignore` was preserved. No client message was sent and no push or sharing occurred.

## 25 November 2026, policy revision before TASK-002

- The candidate revised PD-001–007 to distinguish preliminary shortlist eligibility from booking readiness. The documented access flag and public transport evidence can support provisional eligibility. The revised policy rejects explicitly unacceptable light or Wi-Fi and adds the v019 80/120 regression. These are candidate-approved working assumptions. No client questions were sent or answered.
- Installed Claude Code 2.1.332 in a temporary directory and ran its authentication check. It reported no signed-in account. The candidate then explicitly directed Codex to implement the tasks. No Claude Code model session ran. Using Codex for implementation deviates from the assessment PDF’s Claude Code requirement. No GSD artifacts were generated.

## 25 November 2026, TASK-002

- Codex implemented the typed event request, venue and source evidence, price and capacity invariants, availability and assessment outcomes. Codex reviewed the staged diff. Ingestion, eligibility, deduplication, ranking and CLI behaviour were outside this task. TASK-003 did not start.
- Selected `chrono` 0.4.45 with default features disabled for `NaiveDate` and `Weekday`. Deferred `serde` and `serde_json` until ingestion, `clap` until CLI needs are known, and `rust_decimal` while checked integer minor units suffice.
- The formatter check caught a layout difference after the invalid-evidence payload was added. Ran rustfmt and reran all four gates successfully. `cargo test` ran six passing library tests and zero binary or doc tests. Inspected the one-page A4 render of NOTES. Confirmed the original dataset bytes match the ZIP. Checked the v019 source facts and per-person arithmetic, with the end-to-end regression still to be implemented in the later tasks.
- The observed interval from the first repository check at 19:35:47 GMT through the TASK-002 implementation commit at 19:40:06 GMT was 4 minutes 19 seconds, excluding this final handoff update. The pre-existing staged `.gitignore` was preserved. No push or client communication took place.

## 25 November 2026, R1 architecture and domain review

- Codex compared the assessment PDF, candidate-approved policies, architecture, completed task specs, Rust source and tests, dependency lock and Git history. No Claude Code or GSD planning artifacts were present. The documented Codex deviation from the PDF remains an assessment delivery risk. The local domain task was complete within scope. Chrono is used for calendar values, the public domain depends on neither JSON nor CLI, and checked minor-unit multiplication is tested. No blocking architectural decision was identified.
- **Medium** · `src/domain.rs:66` and `:176` at `8514cb1`. Catering was nested inside a known price. v017 reports `price: "on request"` and `catering_included: true` (`venues.json:269`). Ingestion would drop known catering or need to invent a price. Move catering to independent venue evidence and test the on-request case.
- **Medium** · `src/domain.rs:177` at `8514cb1`. `Option<Money>` treated an invalid or unknown minimum spend the same as an omitted floor. A valid quote with an invalid floor could later look complete. Keep the floor's evidence state and raw invalid value. The source supplies a structured floor on v009. Test malformed versus absent floor evidence.
- Baseline `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked` and `cargo check --locked` passed with six tests. Both corrections and two regression tests are in `b95607e`. All four gates passed again with eight tests. R1 passed before TASK-003.

## 25 November 2026, TASK-003 ingestion and normalisation

- Codex inspected the complete original export, added the file-loading and string-parsing API, exact monetary parsing, normalised evidence and source-linked diagnostics, and verified the untouched dataset bytes. The lockfile records `serde_json` 1.0.154 with `arbitrary_precision`. Cargo's initial broad version feature lookup did not resolve the feature; selecting the version verified in the registry did. The first compile required an explicit lifetime on the March suffix parser, which was corrected before tests passed.
- Reviewed the actual implementation diff. **Low** · `src/normalize.rs:122–135` at `44593e0`. A missing station name could discard valid walking and transport evidence. Normalising those fields independently and testing a row with a walk and tram note but no station corrected the loss in `9d318cc`.
- Final `cargo fmt --all`, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked` and `cargo check --locked` succeeded. There were 23 passing unit tests and one passing dataset integration test. The original export accounted for 30 normalised, zero rejected records and 36 diagnostics. Tests verified v019's normalised evidence. Its eligibility and ranking regression belongs to later tasks.
- Added R1–R4 checkpoints. R4 is TASK-009. Docker packaging is optional TASK-007B and the native execution path remains mandatory. Docker implementation and TASK-004 did not start.
- From the first repository check at 19:40:54 GMT to the final code commit at 19:48:08 GMT, the observed interval was 7 minutes 14 seconds. Together with the prior observed periods in the user-facing handoffs, this is 22 minutes 27 seconds through the code commit, excluding the final documentation checks and any unrecorded time.

## 25 November 2026, Rust engineering standards audit

- Codex audited the single Cargo package, its reusable library and thin binary, source and tests, current agent instructions, dependency graph and history. No existing AGENTS.md or CLAUDE.md was present. Added [one canonical standard](engineering-standards.md) with thin pointers from both agent files, and connected it to the README, review checkpoints and optional Docker task.
- Confirmed Rust and Cargo 1.100.0 on arm64 macOS and Rust 1.100.0 against the official stable release notes and channel manifest. Kept Edition 2024 and the manifest's Rust 1.85 floor. In the isolated rustup home, the locked graph passed `cargo +1.85.0 check --locked --all-targets` and `cargo +1.85.0 test --locked`. No global toolchain was upgraded or repo toolchain version pinned.
- **Low** · `src/ingest.rs:12` and `src/normalize.rs:12` at `d72a0fd`. Parsing called normalisation, which imported diagnostic types back from parsing. Moved the shared types to a private module while retaining the public import paths. In the same refactor, required text now borrows input before allocating owned fields, removing the intermediate venue name allocation at the old `src/normalize.rs:86` and `:133`. Existing row, diagnostic and original-export tests passed. Corrected in `7d8a4f0`.
- **Low** · `src/lib.rs:1` and `src/main.rs:1` at `d72a0fd`. The application crates had no compile-time unsafe-code prohibition. Enforced it in `d4bdd1a`. The audit retained validated value constructors, closed evidence enums, typed I/O and syntax errors, checked monetary arithmetic and the pure domain boundary. Deliberately kept public record fields for evidence carriers.
- Registry metadata and the locked build confirmed `chrono` 0.4.45 (Rust 1.62, default features off) and `serde_json` 1.0.154 (Rust 1.71, `std` and `arbitrary_precision`). `cargo tree --locked -d` and the all-target variant reported no duplicate versions. Neither cargo-audit nor cargo-llvm-cov was installed. No advisory scan or coverage measurement ran.
- On stable, fmt, locked Clippy, test and all-target check, locked release build and no-dependency rustdoc passed. On both tested toolchains, 23 unit tests and one integration test passed, with zero failed or ignored. The export matched the original extraction and its recorded SHA-256. The former Downloads ZIP path was unavailable during this check. The manifest, lockfile, product requirements and dataset were unchanged. TASK-004 did not start.

## 25 November 2026, strict Rust compliance audit

- Codex reviewed the assessment, existing instructions, requirements, architecture, source, tests, tooling and commit history. The baseline `845af5f` passed its local gates on Rust 1.100.0, but lacked the exact toolchain pin, package lint policy, common gate command and CI definition. No new business implementation or independent R2 review occurred.
- Pinned Rust 1.100.0 including rustfmt and Clippy, retained Edition 2024 and verified MSRV 1.85.0 in the isolated toolchain. Added Cargo lints for all first-party targets, explicit Clippy test configuration and Rustdoc. Initial strict lint errors were corrected without group suppression. The lockfile and source dataset are unchanged.
- Ran the local normal and major quality gates. Pinned debug, pinned release and MSRV each passed 23 unit tests and 1 integration test, with 0 failed, 0 ignored and 0 doc tests. The release build and warning-denied rustdoc passed. Local CI configuration checks passed. Hosted CI, automated supply-chain scans and coverage remain unverified. Optional Docker packaging was not started.
- Recorded STD-001 (repeated JSON properties can erase adverse evidence) and STD-002 (file parse errors omit their path) in the [standards audit](engineering/standards-audit.md). The original data contains no repeated keys. Business corrections must use Claude Code, and the earlier Codex implementation deviation remains disclosed. The isolated Claude Code 2.1.332 authentication check reported logged out. TASK-004 was not started.

## 25 November 2026, TASK-004 deduplication

- Codex verified TASK-003 using the pinned quality gate. There were 23 passing unit tests and one passing integration test. The complete original export had 30 normalised rows, zero rejected records and 36 ingestion diagnostics. Its bytes match the original extraction.
- The candidate explicitly directed Codex to implement and review TASK-004. No Claude Code model session or GSD run took place. This was an exception to the project agent routing at the candidate's direction and deviates from the assessment PDF.
- Deduplication retains complete source listings, marks potential pairs and identifies typed field disagreements within confirmed groups. The unchanged export produced 30 entities from 30 listings, zero confirmed groups, three potential pairs involving six listings and zero field disagreements inside confirmed groups. All source IDs and row indices are accounted for. The [data audit](data-analysis.md#potential-duplicate-venues) records each pair's evidence and uncertainty.
- Self-review found that two addressless listings could be grouped while another listing for the same ID, full name and city contained address evidence. This could prematurely hide ambiguous inventory. The implementation now requires an address match for confirmation in that situation and a regression tests both the separate addressless rows and the confirmed addressed pair. Clippy identified reference and fixture style issues during implementation. Fixed them without suppressions and checked the staged source diff, negative evidence and configuration provenance.
- The canonical gate passed on Rust 1.100.0 with 25 unit tests, seven integration tests and zero binary or doc tests. The locked, offline all-target check and test suite also passed on Rust 1.85.0 with the same counts. No test failed or was ignored in either passing suite. No dependencies changed. Coverage and automated supply-chain tools were unavailable. STD-001 and STD-002 remain open as tracked in the [standards audit](engineering/standards-audit.md#unresolved-findings).
- The observed interval from the first TASK-004 check at 20:12:30 GMT through implementation commit `768dc05` at 20:34:51 GMT was 22 minutes 21 seconds. This wall-clock interval includes waiting time and excludes final documentation. Combined with earlier recorded intervals it totals 58 minutes 48 seconds. The staged `.gitignore` was preserved and no remote push took place. Independent R2 and TASK-005 have not started.
