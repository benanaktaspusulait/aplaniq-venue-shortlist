# Master quality review and corrections

This review used master at `214d6dc` after the assessment and the previous hardening pass. Codex inspected the CLI and report, ingestion and JSON diagnostics, field normalisers, the domain types, identity and eligibility, ranking, tests, build scripts, CI configuration and Docker packaging. The [canonical Rust standards](../engineering/rust-standards.md), [approved decisions](../requirements.md#working-product-decisions) and [prior verification](QA-007-quality-hardening-verification.md) define the comparison.

## Verified findings and plan

| ID | Priority | Evidence and standard | Correction and acceptance | Status |
| --- | --- | --- | --- | --- |
| MR-01 | High | [Mandatory fee parser](../../src/normalize/charges.rs) accepts mandatory and required but misses compulsory. An otherwise valid €14,900 quote plus a compulsory €300 fee was eligible with a complete €14,900 package. Violates PD-002 and the all-in budget invariant. | Recognise compulsory wording and explicit negation. Assert €15,200 rejected, optional fees excluded, and mandatory unpriced quotes unresolved. | Open |
| MR-02 | High | [Transport evaluation](../../src/evaluate/rules.rs) matched bus stop in a statement that the nearby stop is closed. A row with no station or walk was eligible. Violates PD-005 and preservation of negative evidence. | Treat explicit unavailable service as insufficient positive evidence while keeping independent documented routes. Test both and keep the explicit taxi-only rejection. | Open |
| MR-03 | Medium | [Ingestion](../../src/ingest.rs) and [deduplication](../../src/dedup.rs) expose mutable collections and independently published totals. A caller can remove rows without updating provenance counts or erase potential identity pairs before ranking. Violates the private invariant-field standard. | Expose shared slices and count accessors. Preserve source-index ordering and potential-pair eligibility tests. Add compile-fail examples for external mutation. | Open |
| MR-04 | Medium | [Terminal output](../../src/report.rs) strips ordinary controls but does not account for Unicode directional formatting controls in listing text. Such controls can alter how surrounding report text is displayed. | Filter directional formatting at the terminal boundary, preserving multilingual letters and leaving raw source evidence intact. Test both behaviours. | Open |

## Scope and verification

Fix the confirmed findings in order with regressions and the canonical gate. Re-run the original export and compare both application streams with the prior verified capture. Run the major gate and container smoke suite on the final source. Record actual results and verification limits below.
