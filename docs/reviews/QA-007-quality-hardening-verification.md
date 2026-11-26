# QA-007 quality-hardening verification

Codex performed the post-assessment correction stream from `2ed1284`. The timed assessment snapshot `35570b4` remains in history. The user requested a master-only workflow during hardening. Master advanced by fast-forward and subsequent changes were committed directly to master. This work is separate from the original three-hour assessment.

## Resolutions

| Finding | Result and evidence |
| --- | --- |
| Q01 and QA-007 VAT scope | `3a20821` and `558c1f7` preserve the known fee floor and require confirmation of excluded or contradictory VAT. Red/green tests include a separate adjacent VAT sentence and an exact-cap budget. AC-04–05. |
| Q02 and QA-007 transit scope | `8d4e5e8`, `558c1f7` and `c467062` preserve negative transit evidence, independent affirmative clauses and conflicting station-walk claims. Explicit taxi-only evidence remains a hard failure when no alternative is supported. AC-09. |
| Q03 required rooms | `d00e904` and `558c1f7` preserve scoped room access and recognise documented windowless or dark plenary and poor plenary Wi-Fi. Optional and uncertain rooms stay unresolved. AC-06–07 and AC-11. |
| Q04 mixed capacity | `b2d2517` retains independently usable seated layouts and records field-specific diagnostics. An unresolved layout can still make an otherwise insufficient capacity uncertain. AC-03. |
| Q05 assessment context | `7889922` gives ranking one deduplicated input and one request. A changed cap is evaluated before ranking and produces a consistent decision. This changes the unpublished library's rank_venues signature. The CLI is unchanged. |
| Q06 typed tie-breaking | `487ad12` replaces formatted ordering with field comparison. Repeated-ID permutations keep the same order regardless of input position. Numeric comparison now replaces incidental ordering within an equivalent identity. Approved ranking precedence is preserved. |
| Q07 duplicate lookup | `7664888` indexes potential pairs by source record. The confirmed-group fixture compares pair membership and order with the original direct scan method. AC-10. |
| Q08 input limits | `8ada304` accepts at most 8 MiB and 1,000 rows, bounds bytes read and rejects non-regular named paths. CLI regressions check both sides and the named error path. AC-01–02. |
| Q09 meaningful invariants | `f2aed73` covers addition overflow, conflicting equally specific dates under both rule orders and a wider integer oracle for multiplication. AC-04 and AC-08. |
| Q10 API and report | `e6bf3d4` removes two unused public types, makes rendering modes explicit and derives formatted preference and currency from the supplied context. The default report matches the baseline. |
| Q11 runtime OpenSSL | `355bd4b` replaces the OpenSSL-bearing runtime with a digest-pinned base-nossl image. The required libgcc binary, Debian package metadata and copyright come from the pinned builder. Image contents, binary linking, runtime controls and vulnerability evidence were checked. |

An independent Codex context examined the corrective diff and identified the adjacent VAT, compound room evidence and independent transit cases. Those cases were reproduced and corrected. The reviewer rechecked both follow-up commits and found no remaining blocker in the examined cases. The original scenario still returns v019, v028 and v021 at 80 attendees, and only v021 remains eligible at 120.

## Clean verification

The application and test revision was `c467062`. In an isolated archive, `sh scripts/quality-gate.sh --major` passed rustfmt, warning-denied Clippy, locked compiler checks, debug and release tests, release build, warning-denied Rustdoc and locked Rust 1.85.0 check and tests. Each of debug, release and MSRV passed **32 unit and 43 integration tests**. There were zero failures, ignores, doctests or final compiler, Clippy and Rustdoc warnings. QA-001 ran 29 unit and 34 integration tests per profile.

`cargo run --locked -- --venues venues.json` and the release binary produced byte-identical stdout and application stderr to each other and to QA-001. All 30 rows were accounted for, with 3 eligible at 80 and 1 eligible at 120. The dataset SHA-256 remains `7a943c847d7e18dd4ee00ff4b93a4335b04749e9b3c1af18db20c037fc4e3a52`. The dependency and feature trees and Cargo metadata were checked. There were no duplicates and no changes to the locked graph. Trivy 0.61.0 with its cached advisory data reported no Cargo vulnerabilities or secrets in the isolated source scan.

Rust 1.85.0 instrumented coverage was **93.46% line, 93.43% function and 85.58% region**, compared with **94.38%, 96.55% and 85.82%** at QA-001. Both runs excluded separate test files and dependencies and included inline tests. The new structural ordering and defensive paths add partially exercised branches. The suite exceeds the requested 90% aggregate line target. The available Apple LLVM tools cannot merge the pinned compiler's raw profile format. Branch counters and automated mutation testing were unavailable. The local `cargo-audit`, `cargo-deny` and coverage subcommands were absent. No tooling installation or hosted CI result is claimed.

### Docker and security

`docker build --check .` and `sh scripts/docker-smoke-test.sh` passed. Linux arm64 was verified through the full smoke suite at 27,397,364 bytes. The Linux amd64 image was built and run under Docker Desktop emulation, at 16,657,742 bytes. Its hardened run against the export matched native stdout and stderr exactly. Both use UID/GID 65532. The arm64 dynamic loader resolved glibc and libgcc_s. The builder reported the copied file as Debian libgcc-s1 14.2.0-19.

Trivy offline scans of both image archives detected no OpenSSL runtime package, no prior CVE-2026-107494 or CVE-2026-124709, no HIGH or CRITICAL findings and zero detected secrets. Each archive had 9 MEDIUM, 10 LOW and 2 UNKNOWN records in libc6 and libgcc-s1. The evidence is scoped to the scanned digests and cached database. The containers were never published. On-demand builds and the native CLI remain local.

### Measured scaling

Alternating the same baseline and hardened release executables on the same host, with streams redirected and five runs per input, gave these medians. Timings include process startup and report rendering.

| Synthetic input | Rows | Baseline | Hardened |
| --- | ---: | ---: | ---: |
| Distinct | 60 | 7.0 ms | 7.6 ms |
| Distinct | 180 | 9.9 ms | 9.8 ms |
| Distinct | 360 | 16.1 ms | 16.3 ms |
| Dense potential matches | 60 | 9.0 ms | 9.1 ms |
| Dense potential matches | 180 | 36.6 ms | 34.7 ms |
| Dense potential matches | 360 | 134.2 ms | 118.9 ms |

The 360-row dense case improved by about 11%. Pair generation and reporting still have quadratic worst-case output size. The row limit bounds that workload.

## Delivery boundary

The configured GitHub remote's authenticated repository metadata reported public visibility. The authorised push was conditioned on private visibility. No push was made and no final remote commit hash is claimed. The remote master still pointed to `35570b4` when checked. The unrelated staged IDE ignore remains outside all commits.

The [standards matrix](../engineering/standards-compliance-matrix.md) records both the local results and the remaining historical, tooling and remote publication exceptions.
