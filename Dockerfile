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
    cargo build --release --locked; \
    install -m 0644 "$(gcc -print-file-name=libgcc_s.so.1)" /build/libgcc_s.so.1; \
    dpkg-query -s libgcc-s1 > /build/libgcc-s1.status; \
    install -m 0644 /usr/share/doc/libgcc-s1/copyright /build/libgcc-s1.copyright

# The CLI needs glibc and libgcc_s. Keep Debian package metadata for image scans.
FROM gcr.io/distroless/base-nossl-debian13:nonroot@sha256:4006ac92504be607b70de01d1f64f38288c69fcd8faa820f6d3aed266ea32ac5
WORKDIR /data
COPY --from=build /build/libgcc_s.so.1 /usr/lib/libgcc_s.so.1
COPY --from=build /build/libgcc-s1.status /var/lib/dpkg/status.d/libgcc-s1
COPY --from=build /build/libgcc-s1.copyright /usr/share/doc/libgcc-s1/copyright
COPY --from=build /build/target/release/venue-shortlist /usr/local/bin/venue-shortlist
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/venue-shortlist"]
