# AplanIQ Venue Shortlist

A local Rust CLI that compares venue listings for an 80-person partner day near Utrecht, with a separate 120-person expansion assessment. It explains which venues meet the working requirements, which need confirmation and which fail.

## Run locally

Use rustup with the [pinned Rust 1.100.0 toolchain](rust-toolchain.toml). From the repository root, run:

```sh
cargo run --locked -- --venues venues.json
```

- `--venues <PATH>` takes a local JSON file with a top-level `venues` array. The file path is required. Quote a path containing spaces.
- `--help` prints usage and `--version` prints the application version. Run either option by itself.
- After `cargo build --release --locked`, the equivalent binary invocation is `target/release/venue-shortlist --venues venues.json`.

## Read the report

The request uses a **EUR 15,000 all-in planning cap**, at least three simultaneous breakout rooms, wheelchair access, and 11, 16 and 18 March 2027. Thursday is preferred. The report offers preliminary recommendations from an unverified export. Confirm the event hours, final quote, room configuration and accessible facilities with the venue before booking. A conditional result lists evidence to verify. Unknown charges remain unresolved.

The recorded run includes these lines.

```text
  Source records: 30
  80 attendees: 3 eligible, 19 require confirmation, 8 rejected
  120 attendees: 1 eligible, 16 require confirmation, 13 rejected

Ranked eligible shortlist
  1. De Stadstuin [v019]
     Recommended date: Thursday 11 March 2027
     Other supported dates: Tuesday 16 March 2027, Thursday 18 March 2027
     Price: preliminary package EUR 6,320.00.
```

See the [complete CLI transcript](docs/cli-output.txt) and the [data audit](docs/data-analysis.md). The report goes to stdout. Recoverable source-row diagnostics and fatal errors go to stderr. Exit code `0` includes a valid empty shortlist and datasets with recoverable row errors. Code `2` means invalid arguments. Code `1` means an input or output failure. Fatal input errors prevent a partial report.

## Verify changes

```sh
cargo test --locked
sh scripts/quality-gate.sh
```

For a major review, run `sh scripts/quality-gate.sh --major` with Rust 1.85.0 also installed. The package uses Edition 2024. See the [engineering standards](docs/engineering/rust-standards.md), [architecture](docs/architecture.md), [assessment notes](NOTES.md) and [development log](docs/assessment-log.md) for assumptions and actual agent usage.
