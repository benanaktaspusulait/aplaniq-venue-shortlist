# AplanIQ Venue Shortlist

A local Rust CLI that compares venue listings for an 80-person partner day near Utrecht, with a separate 120-person expansion assessment. It explains which venues meet the working requirements, which need confirmation and which fail.

## Run locally

Use rustup with the [pinned Rust 1.100.0 toolchain](rust-toolchain.toml). From the repository root, run:

```sh
cargo run --locked -- --venues venues.json
```

- `--venues <PATH>` takes a local JSON file with a top-level `venues` array. The file path is required. Quote a path containing spaces. Exports may contain up to 1,000 venue rows and occupy up to 8 MiB.
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

## Run in Docker

Use Docker Engine with BuildKit or Docker Desktop, with the daemon running and the project directory available for bind mounts. Docker is optional. The native commands above work independently.

```sh
docker build -t aplaniq-venue-shortlist:local .

docker run --rm \
  --network none \
  --read-only \
  --cap-drop ALL \
  --security-opt no-new-privileges=true \
  -v "$(pwd)/venues.json:/data/venues.json:ro" \
  aplaniq-venue-shortlist:local \
  --venues /data/venues.json

docker run --rm aplaniq-venue-shortlist:local --help
docker run --rm aplaniq-venue-shortlist:local --version
```

The image runs as UID and GID `65532`. Supply a JSON file readable by that user at the mount path. Both base images are pinned by digest. The runtime contains the release binary and its required libgcc runtime on a Debian 13 distroless base without OpenSSL or a shell. Image and crate downloads can require network access during the build. The run above disables networking.

Run `sh scripts/docker-smoke-test.sh` to rebuild and compare native and container output and verify expected failures. The smoke suite passed on Linux arm64. The Linux amd64 image also ran under emulation and matched native output. See the [current verification](docs/reviews/QA-007-quality-hardening-verification.md#docker-and-security) for measurements and scan results.
