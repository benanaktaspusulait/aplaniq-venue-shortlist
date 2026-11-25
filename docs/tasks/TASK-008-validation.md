# TASK-008 — End-to-End Validation

## Status

The original assessment stopped at the time limit. The local validation deliverable was completed on the post-assessment branch. The original state and the later results are recorded separately below.

## Objective

Validate core-to-CLI behaviour with controlled fixtures and the supplied export.

## Business justification

Individually correct modules can still lose evidence or use the wrong request when wired together.

## Dependencies

[TASK-007](TASK-007-cli.md) for the documented runnable entry point and component tests from TASK-002 through TASK-007.

## Requirements and scope

Add a small integration fixture that exercises the pipeline and run a smoke check against `venues.json`. Check result ordering, explanation, warning propagation, reproducibility and command failure behaviour. Record actual outputs and outstanding issues.

## Explicitly out of scope

Replacing component tests, large snapshot baselines, benchmark infrastructure and editing the original dataset.

## Expected files and modules

`tests/shortlist.rs` or a similarly scoped integration test, optional small fixtures under `tests/fixtures/`, and `docs/assessment-log.md`.

## Implementation guidance

See the [requirements and acceptance matrix](../requirements.md).

Use the positive fixture from AC-01 and add rejected and unresolved cases under the candidate-approved policy. Verify the v019 regression (AC-15) and a synthetic clone of its evidence with a different ID. Use the acceptance matrix to select integration coverage. Assert meaningful outcomes and reasons. Repeat the exact documented CLI invocation against the export. Check one plausible inclusion or exclusion manually against the input and selected product policy.

## Acceptance criteria

- Given a controlled fixture, confirmed IDs, order and reasons match the expected policy. The fixture exercises at least one invalid or unresolved value.
- Given the documented command and unchanged real export, two runs complete with matching result order and explanations. A valid empty output explains why it is empty.
- Given malformed input, the complete command exits non-zero with an actionable diagnostic.

## Required tests and edge cases

Small fixture with multiple verdicts including a hard failure, malformed input, real-export smoke and repeatability. Include a conflict or qualified access statement that is visible downstream.

## Validation commands

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo check` and the documented default command twice. Verify the dataset SHA-256 against the [data audit](../data-analysis.md).

## Definition of done

Integration tests and smoke checks pass or remaining failures are recorded with evidence. Completion requires the expected success checks to pass. Review and commit tests and validation notes locally.

## Risks and unresolved assumptions

A strict policy can validly produce an empty real-data result. Use a controlled fixture to demonstrate a non-empty ranking and check its explanation.

## Estimated effort

12 minutes.

## Suggested Git commit messages

`test: verify the shortlist pipeline and original dataset`

## Time reconciliation

The reported project start was approximately 19:00 GMT on 25 November 2026. The TASK-008 check at 22:22 GMT was approximately 3 hours 22 minutes later. The first recorded environment check at 19:22:12 and first commit at 19:23:13 are later evidence points. They do not establish the start of human planning. The previous 1 hour 56 minutes 30 seconds total sums observed task intervals. Some intervals include waiting and others exclude final documentation or earlier work. Gaps between commits do not establish breaks, and there is no reliable record of breaks or of all work outside the measured intervals. The actual active total is undetermined. The three-hour limit was treated as reached. Codex stopped development and recorded this handoff.

## Validation matrix

Historical results describe the earlier tasks. TASK-008 did not rerun them or change the application, tests or CI.

| Check | TASK-008 status | Evidence | Remaining limitation |
| --- | --- | --- | --- |
| Assessment time compliance | BLOCKED | Approximate start and observed intervals reconciled above. | Remaining work time cannot be established. |
| Original assessment requirements | NOT VERIFIED | Existing native command and traceability for AC-01–15 are recorded in the [standards audit](../engineering/standards-audit.md#requirements-to-tests-traceability). | Final acceptance and submission review remain. |
| Tracked inputs and original dataset | PASS | Git tree contains the pinned toolchain, lockfile, source, tests and documented command. File and committed bytes both matched the extracted source. SHA-256 `7a943c847d7e18dd4ee00ff4b93a4335b04749e9b3c1af18db20c037fc4e3a52`. | Static check only. |
| Repository hygiene | NOT VERIFIED | Tracked path inventory contains no build cache, IDE configuration or binary build output. | No fresh secret scan or full content audit. The pre-existing staged .gitignore was preserved. |
| Clean checkout build and execution | NOT VERIFIED | Existing CLI commit `f3f49bd`. | Rebuild from a clean checkout and run the documented command. |
| Native CLI, dataset accounting, business results, determinism | NOT VERIFIED | Earlier CLI run accounted for all 30 records. At 80: 3 eligible, 19 conditional, 8 rejected, ordered v019, v028, v021. At 120: 1 eligible, 16 conditional, 13 rejected. Prior debug and release outputs matched. [Captured report](../cli-output.txt). | No fresh end-to-end run, independent recalculation or permutation check in TASK-008. |
| Unit, integration and regression tests | NOT VERIFIED | TASK-007 recorded 29 unit and 32 integration passes in each profile and zero failed, ignored or doc tests. R2 and R3 corrections are recorded. | No fresh tests or regression additions. Audit remaining acceptance-criterion gaps. |
| Debug, release, MSRV, rustfmt, Clippy and Rustdoc | NOT VERIFIED | The major gate previously passed on Rust 1.100.0 and MSRV 1.85.0 in TASK-007. | Rerun the canonical major gate from the clean checkout. |
| Dependency security | NOT VERIFIED | No new dependency changes in packaging. | No fresh dependency advisory or licence scan. |
| Container build, runtime controls, native equivalence and ARM64 | NOT VERIFIED | TASK-007B previously verified Linux arm64 hardened execution and byte-identical native/container streams in commits `29d6366` and `069c97b`. | The image and build cache were removed at the user's separate request. Rebuild and rerun the smoke test to revalidate. |
| AMD64 | NOT VERIFIED | Both pinned base manifests offer amd64. | Application image build and runtime were not tested on amd64. |
| Container vulnerability exposure | NOT VERIFIED | Retained TASK-007B Trivy report inspected. No critical or secret findings. Two high records remain in Debian 13 distroless `libssl3t64` `3.5.9-1~deb13u1`, CVE-2026-107494 and CVE-2026-124709. No fixed version was listed. The binary did not link OpenSSL. | This does not establish absence of runtime exposure. Rebuild and rescan against a refreshed pinned base when fixes are available. |
| CI workflow | PASS | Existing `.github/workflows/quality.yml` at `29defc5` uses read-only contents, an immutable checkout reference and the canonical major gate. | Static inspection only. No workflow changes or fresh upstream verification. |
| Remote CI execution | NOT VERIFIED | No hosted run was observed. | Validate during an authorised hosted run. |

At the stop, clean-checkout reproduction, fresh native and Docker validation, and targeted coverage checks remained open. R4 was still ahead.

## Post-assessment validation

- A clean archive of `02e18f3` passed `sh scripts/quality-gate.sh --major`. Debug, release and Rust 1.85.0 each passed 29 unit and 34 integration tests. There were no failures, ignored tests or doctests.
- The documented `cargo run --locked -- --venues venues.json` ran twice. Stdout and application diagnostics matched each other, the release binary and the earlier native capture. All 30 records were accounted for. At 80 attendees the order remained v019, v028, v021 with 3 eligible, 19 conditional and 8 rejected. At 120, Fabriek 1912 alone was eligible, with 16 conditional and 13 rejected. The archived export matched the original ZIP bytes.
- The controlled CLI fixture in `02e18f3` checks a synthetic v019, two ranked entries, qualified access, a hard budget rejection, both headcounts and a source diagnostic. The original-export CLI test binds prices, dates and expansion to each listing. Existing process tests check empty and malformed inputs, status and output channels.
- The Docker smoke test passed on Linux arm64. The rebuilt image measured 39,258,290 bytes. Runtime settings, native parity and error paths passed again. The image was removed after the scan. Application execution on amd64 remains unverified.
- The [assessment log](../assessment-log.md#25-november-2026-post-assessment-completion) records security scan scope, dependency metadata and remaining limitations.
