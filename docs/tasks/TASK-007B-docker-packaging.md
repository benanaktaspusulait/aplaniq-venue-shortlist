# TASK-007B — Optional Docker Packaging

## Status

To do if time permits.

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

`Dockerfile`, `.dockerignore` and `README.md`. Estimated 10 minutes, optional and outside the mandatory backlog estimate. Suggested commit `build: package the local shortlist CLI in an optional container`.
