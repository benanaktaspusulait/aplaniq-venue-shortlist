#!/bin/sh
set -eu

cd "$(dirname "$0")/.."
root=$(pwd -P)
image=aplaniq-venue-shortlist:local
# Keep bind-mounted fixtures in the shared project path on Docker Desktop.
temp=$(mktemp -d "$root/.docker-smoke.XXXXXX")
container=
cleanup() {
    if [ -n "$container" ]; then
        docker rm -f "$container" >/dev/null
    fi
    rm -rf "$temp"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM
temp=$(cd "$temp" && pwd -P)

run_locked() {
    docker run --rm --network none --read-only --cap-drop ALL \
        --security-opt no-new-privileges=true "$@"
}

expect_failure() {
    expected=$1
    pattern=$2
    shift 2
    if run_locked "$@" >"$temp/failure.out" 2>"$temp/failure.err"; then
        printf '%s\n' "Expected exit $expected, got success" >&2
        exit 1
    else
        actual=$?
    fi
    if [ "$actual" != "$expected" ] || [ -s "$temp/failure.out" ] ||
        ! grep -F "$pattern" "$temp/failure.err"; then
        printf '%s\n' "Unexpected failure result (exit $actual, expected $expected)" >&2
        cat "$temp/failure.err" >&2
        exit 1
    fi
}

docker build -t "$image" .
test "$(docker image inspect --format '{{.Config.User}}' "$image")" = '65532:65532'
cargo build --release --locked

native=target/release/venue-shortlist
"$native" --venues venues.json >"$temp/native.out" 2>"$temp/native.err"
container=$(docker create --network none --read-only --cap-drop ALL \
    --security-opt no-new-privileges=true \
    --mount "type=bind,source=$root/venues.json,target=/data/venues.json,readonly" \
    "$image" --venues /data/venues.json)
security=$(docker inspect --format '{{.HostConfig.NetworkMode}} {{.HostConfig.ReadonlyRootfs}} {{join .HostConfig.CapDrop ","}} {{join .HostConfig.SecurityOpt ","}}' "$container")
test "$security" = 'none true ALL no-new-privileges=true'
mount=$(docker inspect --format '{{range .Mounts}}{{.Destination}} {{.RW}}{{end}}' "$container")
test "$mount" = '/data/venues.json false'
docker start -a "$container" >"$temp/container.out" 2>"$temp/container.err"
test "$(docker inspect --format '{{.State.ExitCode}}' "$container")" = 0
cmp "$temp/native.out" "$temp/container.out"
cmp "$temp/native.err" "$temp/container.err"
printf '%s\n' 'Native and container reports and diagnostics match byte for byte.'

for option in --help --version; do
    "$native" "$option" >"$temp/native-option"
    run_locked "$image" "$option" >"$temp/container-option"
    cmp "$temp/native-option" "$temp/container-option"
done

expect_failure 2 'unrecognised arguments' "$image" --unknown
expect_failure 1 'cannot read /data/missing.json' "$image" --venues /data/missing.json
printf '{\n  "venues": [}\n' >"$temp/malformed.json"
expect_failure 1 'cannot parse /data/malformed.json: invalid venue JSON: expected value at line 2 column 14' \
    --mount "type=bind,source=$temp/malformed.json,target=/data/malformed.json,readonly" \
    "$image" --venues /data/malformed.json

# Start from a fresh container so even bind-mount placeholder paths are absent.
docker rm "$container" >/dev/null
container=
container=$(docker create "$image")
docker export "$container" >"$temp/rootfs.tar"
tar -tf "$temp/rootfs.tar" >"$temp/rootfs.list"
grep -Fx 'usr/local/bin/venue-shortlist' "$temp/rootfs.list"
if grep -E '(^|/)(venues[.]json|Cargo[.](toml|lock)|[.]git|[.]planning|[.]gsd|NOTES[.]md|assessment-log[.]md)(/|$)|^build(/|$)|(^|/)(bin|sbin)/(rustc|cargo|gcc|cc|bash|sh)$' "$temp/rootfs.list"; then
    printf '%s\n' 'Unexpected build tooling or project files in the runtime image' >&2
    exit 1
fi

docker image inspect --format 'Image {{.Os}}/{{.Architecture}}, {{.Size}} bytes, user {{.Config.User}}' "$image"
printf '%s\n' 'Docker smoke test passed.'
