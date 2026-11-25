# TASK-007B — Optional Docker Packaging

## Status

Complete. Built and ran locally on Linux arm64. The native deliverable remains primary.

## Objective and scope

After [TASK-007](TASK-007-cli.md), optionally package the working local CLI before [TASK-008](TASK-008-validation.md). The assessment requires one documented local command and passing native tests. An optional local container is compatible with that requirement. The native command stays the primary path and TASK-008 can proceed if this task is skipped.

## Implementation guidance

- First run `docker version` and `docker info` to verify the client and daemon. Record the result, or record that packaging was skipped to protect the three-hour budget.
- Add a multi-stage `Dockerfile` using compatible builder and runtime images, with the exact Rust builder version in `rust-toolchain.toml` and the [engineering standards](../engineering/rust-standards.md#dependencies-and-packaging). Verify the binary linkage against the runtime image. Build the release binary with `cargo build --release --locked`. Copy only the binary into the minimal runtime image.
- Execute as a non-root user. Add `.dockerignore` to exclude build output, VCS and local credentials. Use an exec-form entrypoint with CLI argument pass-through. Supply venue JSON externally through a read-only mount. Keep the image free of secrets and the runtime independent of networking.
- Document local build, run and mount commands in `README.md`. Docker Compose, Kubernetes, cloud deployment, registry and image publication are out of scope.

## Acceptance criteria and validation

Build locally with the locked manifest. Verify the runtime user is non-root. Run a smoke test with `--network none`, a read-only input mount and CLI flags. Verify help and the invalid-input exit status. Compare native and container output and exit status for the same fixture and request. Confirm the documented native invocation still passes. A build may fetch locked crates and base images. The container run must work without network access.

## Expected files and commit

`Dockerfile`, `.dockerignore`, `scripts/docker-smoke-test.sh` and `README.md`. Estimated 10 minutes, optional and outside the mandatory backlog estimate. Build and smoke test commit `29d6366`.


## Observed validation

- Docker Desktop client and server 29.9.2 on arm64 macOS, with a Linux arm64 daemon and Buildx 0.38.1 / BuildKit 0.34.0. The builder advertises Linux arm64 and amd64 among its platforms. Both pinned indexes offer those platforms. Only Linux arm64 was built and run in this task.
- The official Rust 1.100.0 Debian Trixie builder checks the compiler against the repository pin and uses `cargo build --release --locked`. A Debian 13 distroless C/C++ runtime provides the dynamic loader, glibc and `libgcc_s.so.1` verified by the loader's `--list` output. The runtime adds only the executable and its working directory to the base. The cold build context was 167.07 kB with a source-and-manifest allowlist.
- The Linux arm64 image is **39,255,338 bytes** according to `docker image inspect`. The smoke test verified UID/GID 65532, network mode none, read-only root and input mount, all capabilities dropped, and no-new-privileges on the successful run. Exported filesystem inspection found no project data, sources, Git metadata, agents' files, assessment documents, compiler or shell.
- Native and container stdout and stderr matched byte for byte for the original export. Both exited 0 and reported the same order and 80/120 counts. Help and version matched. The container returned 2 for invalid arguments and 1 for missing and malformed input. JSON stderr retained the path, line and column. `docker build --check .` reported no warnings.
- Trivy 0.61.0 reported **0 critical, 2 high, 34 medium, 17 low and 3 unknown** vulnerability records and no detected secrets. Both high findings are in the base's `libssl3t64`, with no fixed version identified by the scanner. The CLI does not link OpenSSL. The scan covers recognised packages and secret signatures.
