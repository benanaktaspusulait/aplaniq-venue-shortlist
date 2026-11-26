# Master quality review and corrections

This review used master at `214d6dc` after the assessment and the previous hardening pass. Codex inspected the CLI and report, ingestion and JSON diagnostics, field normalisers, the domain types, identity and eligibility, ranking, tests, build scripts, CI configuration and Docker packaging. The [canonical Rust standards](../engineering/rust-standards.md), [approved decisions](../requirements.md#working-product-decisions) and [prior verification](QA-007-quality-hardening-verification.md) define the comparison. This is post-assessment maintenance. Codex implemented and reviewed the changes.

## Verified findings and plan

| ID | Priority | Evidence and standard | Correction and acceptance | Status |
| --- | --- | --- | --- | --- |
| MR-01 | High | [Mandatory fee parser](../../src/normalize/charges.rs) accepts mandatory and required but misses compulsory. An otherwise valid €14,900 quote plus a compulsory €300 fee was eligible with a complete €14,900 package. Violates PD-002 and the all-in budget invariant. | Recognise compulsory wording and explicit negation. Assert €15,200 rejected, optional fees excluded, and mandatory unpriced quotes unresolved. | Fixed in `55f2034` |
| MR-02 | High | [Transport evaluation](../../src/evaluate/rules.rs) matched bus stop in a statement that the nearby stop is closed. A row with no station or walk was eligible. Violates PD-005 and preservation of negative evidence. | Treat explicit unavailable service as insufficient positive evidence while keeping independent documented routes. Test both and keep the explicit taxi-only rejection. | Fixed in `70a54ad` |
| MR-03 | Medium | [Ingestion](../../src/ingest.rs) and [deduplication](../../src/dedup.rs) expose mutable collections and independently published totals. A caller can remove rows without updating provenance counts or erase potential identity pairs before ranking. Violates the private invariant-field standard. | Expose shared slices and count accessors. Preserve source-index ordering and potential-pair eligibility tests. Add compile-fail examples for external mutation. | Fixed in `8194a67` |
| MR-04 | Medium | [Terminal output](../../src/report.rs) strips ordinary controls but does not account for Unicode directional formatting controls in listing text. Such controls can alter how surrounding report text is displayed. | Filter directional formatting at the terminal boundary, preserving multilingual letters and leaving raw source evidence intact. Test both behaviours. | Fixed in `1173df0` |

## Scope and verification

Fix the confirmed findings in order with regressions and the canonical gate. Re-run the original export and compare both application streams with the prior verified capture. Run the major gate and container smoke suite on the final source. Record actual results and verification limits below.

### Regression evidence

- **MR-01.** The new eligibility test failed on the base behaviour and passed with the fix. It checks exact assessed cost and eligibility for priced, unpriced, negated and non-compulsory language.
- **MR-02.** The closed-stop reproduction initially still matched the characters at the end of the word nearby. A token-boundary check removed that accidental match. The final fixture includes closed and unavailable stops, an unrelated closed business and an alternative train route.
- **MR-03.** Both compile-fail examples compiled before encapsulation. After the fix they fail for private fields as expected. The public library reads the results with methods and shared slices. The original grouping and accounting assertions are retained. The previous permutation assertions now run inside the ingestion unit suite so they can reorder the internal fixture without exposing a public mutation method.
- **MR-04.** The renderer test failed before the output filter recognised the directional controls. It now verifies all 12 controls individually, preserves multilingual letters and a joiner, and checks the borrowed return for normal text. The filter follows the [Unicode directional formatting inventory](https://www.unicode.org/reports/tr9/#Directional_Formatting_Characters).

### Other layers checked

- **JSON and provenance.** Size and row budgets, named error chains, UTF-8 handling, repeated fields and unaffected neighbouring records remain in effect.
- **Domain and ranking.** Checked minor units, structured seating, date specificity, explicit scenario separation and typed secondary ordering remain covered. The API change in MR-03 keeps parsed source accounting and duplicate uncertainty consistent across stages. The unpublished library now uses accessors in place of the previous public report fields.
- **Build and packaging.** The dependency manifest, lockfile, pinned toolchain, CI workflow, Dockerfile and original dataset are unchanged. The current host exercises the Rust gate and local image. Hosted CI remains separate evidence.

### Final checks

The inspected code revision was `1173df0`. On 26 November 2026 the major gate passed in a clean Git archive with no compiler, Clippy or Rustdoc warnings. Debug, release and the Rust 1.85.0 MSRV each ran 34 unit tests, 45 integration tests and 2 compile-fail doctests. There were zero failed or ignored tests. Rust 1.85.0 instrumentation measured 93.64% lines, 93.82% functions and 85.95% regions using the same exclusions as QA-007. Branch coverage was not measured.

The native invocation and release binary matched each other and the previous verified application output and diagnostics byte for byte. All 30 records were preserved. The original export still reports 3 eligible, 19 conditional and 8 rejected for 80 attendees, and 1 eligible, 16 conditional and 13 rejected for 120.

The arm64 Docker smoke suite passed. A separate amd64 emulated run also matched the native streams. The locked dependency graph has no duplicate versions. The offline Trivy source scan detected zero advisory or secret findings. The rebuilt arm64 image scan reported zero HIGH and CRITICAL findings, 9 MEDIUM, 10 LOW, 2 UNKNOWN and zero detected secrets. Scan results are limited to the cached advisory database.

### Copilot audit follow-up

The original Copilot report is preserved in commit `4b0353a`. It covers the older `35570b4` source. A separate read-only reviewer found no reproducible business-code defect and ran no quality gates. The historical R3/R4 independence deviation remains in the [assessment log](../assessment-log.md) and [R3 review](../engineering/r3-business-logic-review.md). Copilot separately measured 92.84% line coverage with pinned Rust 1.100.0 and 61 passing tests on that snapshot. The later MSRV coverage and native and container results above apply to the corrected source.

GitHub Actions passed the same major gate on `4b0353a`, whose code matches `1173df0`. [Hosted workflow run](https://github.com/benanaktaspusulait/aplaniq-venue-shortlist/actions/runs/45787816515)

The dedicated `cargo audit` advisory and `cargo deny check` dependency policy and licensing checks remain blocked because neither tool is installed locally.
