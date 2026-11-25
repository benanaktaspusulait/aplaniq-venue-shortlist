FROM rust:1.100.0-trixie@sha256:e84068000361bf3d4a5ba2b7647f62893528b1eb9f847d7a23b493f5ce8ea254 AS build
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml /tmp/project-toolchain.toml
COPY src/ ./src/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    set -eu; \
    expected="$(awk -F '"' '/^channel = / {print $2}' /tmp/project-toolchain.toml)"; \
    actual="$(rustc --version | awk '{print $2}')"; \
    test "$actual" = "$expected"; \
    cargo build --release --locked

FROM gcr.io/distroless/cc-debian13:nonroot@sha256:29ee70762214fe8190cea04ef1c0a84a8365879e047ce8f911ae5a5ab2355fa2
WORKDIR /data
COPY --from=build /build/target/release/venue-shortlist /usr/local/bin/venue-shortlist
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/venue-shortlist"]
