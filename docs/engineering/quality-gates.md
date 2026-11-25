# Rust quality gates

Run `sh scripts/quality-gate.sh` from the repository for each implementation change. Use `sh scripts/quality-gate.sh --major` for R1–R4 and final delivery. The script uses the pinned [toolchain](../../rust-toolchain.toml), exits at the first nonzero command and propagates its status. Install the pinned components and the declared MSRV in an authorised rustup environment before checking. The script leaves toolchain installation to the environment.

## Commands

| Required | Execution |
| --- | --- |
| Every implementation task | `cargo fmt --all -- --check` |
| Every implementation task | `cargo clippy --locked --all-targets --all-features -- -D warnings` |
| Every implementation task | `cargo check --locked --all-targets --all-features` |
| Every implementation task | `cargo test --locked --all-features`, including doc tests |
| Major review and delivery | `cargo build --release --locked` |
| Major review and delivery | `cargo test --release --locked` |
| Major review and delivery | `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features` |
| When the MSRV differs | Run locked check and tests using `cargo +<msrv>`. The major gate runs both and reads the version from the manifest. |
| Relevant feature combinations | Exercise supported combinations separately if package features are added. The current package declares none. |

## Conditional verification

- Use `cargo audit` for vulnerability advisories, `cargo deny check` for dependency policy and licensing, and `cargo llvm-cov` for measured line and branch coverage where supported, when available and permitted. Record missing tools and resulting limits. Use geiger for unsafe inventory only. Consider semver checks for a maintained public API, fuzzing for a justified parser campaign, mutation testing for a specific test gap, Miri for undefined-behaviour risk and Loom for concurrent code. Keep `cargo test` even if nextest is available.
- [CI](../../.github/workflows/quality.yml) uses read-only access, a commit-pinned checkout action and the same major gate. A local script pass does not verify hosted execution. Add Docker smoke checks after packaging when in scope.
- Record required status, observed evidence and limitations in the [standards audit](standards-audit.md) or assessment log. Use **PASS**, **FAIL**, **BLOCKED**, or **NOT APPLICABLE**. Report passed, failed and ignored tests for each profile, plus doc-test counts and actual coverage only if measured.
