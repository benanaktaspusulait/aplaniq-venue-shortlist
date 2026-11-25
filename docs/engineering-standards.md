# Rust engineering standards

Apply these standards to implementation, tests, reviews and optional packaging. The [requirements](requirements.md) own the candidate-approved policies and acceptance criteria. The [assessment log](assessment-log.md) records decisions, actual agent usage and verified results. Keep the original venue export intact.

## Toolchain and language

- Use stable Rust, Edition 2024, and honour the minimum supported Rust version (MSRV) in `Cargo.toml`, currently **1.85.0**. Use current stable for engineering gates. Recheck the locked graph on the MSRV when changing language features, the manifest or the lockfile. Record both tested versions. The 25 November 2026 audit used stable **1.100.0** and separately verified **1.85.0**. [Rust release notes](https://doc.rust-lang.org/stable/releases.html) · [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
- Prefer exhaustive matching, `let else`, `?`, straightforward iterators and `Option`/`Result` combinators when they clarify intent. Use type inference, derives, standard collections and const functions where they earn their place. The Edition 2024 `let` chain feature requires Rust 1.88, so it is outside the current floor. Use stable features supported by the MSRV. Change the floor explicitly with compatibility evidence and a documented reason. Keep global toolchain changes subject to human approval. [Rust 1.88 release notes](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/#let-chains)

Reproduce the current MSRV check with an available 1.85.0 toolchain:

```sh
cargo +1.85.0 check --locked --all-targets
cargo +1.85.0 test --locked
```

## Package and boundaries

- Keep one Cargo package, `venue-shortlist`, with the reusable `venue_shortlist` library crate and thin `venue-shortlist` binary crate. Modules group code inside a crate. External crates belong in Cargo dependencies.
- Let the CLI handle arguments and I/O orchestration and render a typed result. Application orchestration calls the core. Ingestion and recommendation may use the domain. The domain stays independent of filesystem access, raw JSON, CLI and formatting. Keep dependencies acyclic. Use current [module boundaries](architecture.md#module-boundaries) and split files when responsibilities justify it.
- Default to private visibility, use `pub(crate)` or narrower visibility for internal APIs, and expose `pub` deliberately. Keep a focused `lib.rs`. Use explicit imports and domain terms consistently. Keep public fields for deliberate data carriers. Protect value invariants with private fields and validated construction instead of widening mutability for convenience. Use snake_case for modules and add Rustdoc for meaningful contracts or invariants.

## Domain design and reliability

- Use structs and newtypes for meaningful values, `impl` for behaviour a type owns, enums for closed states and composition for collaboration. Introduce traits, associated types or static polymorphism when a concrete behavioural boundary warrants them. Prefer a pure function when it expresses a rule directly. Keep responsibilities focused and avoid framework-style layers and speculative dispatch.
- Preserve missing, unknown, invalid and conflicting evidence when decisions depend on it. Use `Option` for actual absence and `Result` when callers need a failure reason. Keep seated and standing capacity distinct. Represent money in integer minor units or a justified exact decimal type. Use checked arithmetic for external values and preserve unknown costs. Floating-point budget comparisons are prohibited.
- Borrow with `&str` and slices when ownership is unnecessary. Prefer immutable bindings and avoid copies, allocations, indirection, explicit lifetimes and concurrency without a concrete reason. Prefer readable code over micro-optimisation.
- The library and binary forbid unsafe code. Validate input and return actionable typed errors where callers need distinctions. Keep the source reference for problematic records. Avoid `unwrap()`, `expect()`, `panic!()`, `todo!()` and `unimplemented!()` in external-input paths. Focused tests may use assertions and fixture unwraps. Keep secrets, external network calls and AI integration out of the runtime.

## Dependencies and packaging

| Direct crate | Purpose, required features and compatibility |
| --- | --- |
| `chrono` 0.4.45 | `NaiveDate` and `Weekday` model and validate calendar evidence without implementing date arithmetic. The standard library has no equivalent calendar types. Default features are off. The registry declares Rust 1.62. [Crate metadata](https://crates.io/crates/chrono/0.4.45) |
| `serde_json` 1.0.154 | The JSON reader and heterogeneous `Value` fields avoid a custom parser. Use normal `std` support and `arbitrary_precision` to retain monetary number text for checked minor-unit conversion. The registry declares Rust 1.71. [Crate metadata](https://crates.io/crates/serde_json/1.0.154) |

- Verify actual usage, MSRV, features, maintenance and advisory status before adding or updating a crate. Prefer a maintained dependency when the standard library cannot meet the need simply. Keep test-only crates under `[dev-dependencies]`. Commit `Cargo.lock`, inspect `cargo tree --locked` and `cargo tree --locked -d`, and keep features narrow. Preserve parser validation and avoid speculative networking, async, web, database or AI dependencies.
- Run `cargo audit` when available and report the actual result. Without the tool, report that an advisory scan did not run. The November audit inspected the graph and registry metadata for both direct crates. Neither duplicate versions in the active graph nor enabled Chrono clock/timezone features were reported.
- [Optional Docker packaging](tasks/TASK-007B-docker-packaging.md) uses an explicitly versioned Rust builder compatible with the tested stable toolchain, manifest and lockfile. Verify the native command independently. Test the container's successful build, non-root user, read-only input, argument forwarding, runtime without a network, and output parity with native execution.

## Testing and gates

- Add focused behavioural unit tests near the implementation and public API and cross-module tests under `tests/`. Trace business cases to the acceptance criteria. Include regression tests with genuine fixes where practical. Use descriptive names, small fixtures and clear arrange, act and assert steps. Keep the original export test for complete source record accounting.
- Cover capacity and both attendance scenarios, breakout requirements, all observed price units, budget edges, VAT, catering and minimum spend, unsupported currencies, access, availability, daylight and Wi-Fi, duplicate and contradictory evidence, deterministic ranking, reasons and empty output as each feature lands. Include malformed JSON, missing identifiers, invalid dates and numbers, explicit zero, overflow, empty input and unsupported representations.
- For the CLI, test a valid command, missing input, invalid JSON, help, exit statuses, deterministic output and actionable errors. Add property-based tests only when they cover a meaningful invariant. Measure coverage with `cargo llvm-cov --locked` if already installed and report the measured result or the tool's absence.

Run these at the end of every implementation task:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo check --locked --all-targets
```

Also run `cargo build --release --locked` and `cargo doc --no-deps --locked` at major reviews and final delivery. Report actual passed, failed and ignored counts. Address warnings and scope any justified lint exception narrowly.

## Reviews and Git

Keep [R1–R4](tasks/README.md#engineering-review-checkpoints). Review requirements and business rules, architecture, Rust design and ownership, dependencies, type safety, input errors, security, test coverage, edge cases and maintainability. Inspect actual diffs, tests and history. Record findings with severity, file and line, concrete evidence and impact. Verify corrections with tests and log the results.

Commit each meaningful, coherent change with its tests. Review status, staged and unstaged diffs, and relevant validation before committing exact paths. Keep secrets and unrelated staged work out. Preserve actual agent artifacts and history, including unsuccessful attempts and fixes. Push only with explicit authorisation.
