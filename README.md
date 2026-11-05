# pushkind-broker

pushkind-broker is a lightweight Rust 2024 daemon that exposes one or more
ZeroMQ `XSUB ⇄ XPUB` proxy pairs for Pushkind services. Each pair runs in its own
thread with configurable high-water marks, making it easy to fan messages out to
workers while keeping back-pressure under control.

## Features

- Multiple broker pairs defined via simple YAML/TOML/JSON configuration files.
- Per-socket high-water mark tuning to balance throughput and memory usage.
- Structured logging through `log`/`env_logger` for operational visibility.
- Graceful shutdown that zeros linger to avoid stalled sockets.
- Optional systemd unit (`pushkind-broker.service`) for production deployment.

## Project Structure

- `src/main.rs` — initializes logging, loads settings, and spawns threads.
- `src/config.rs` — loads `proxy.{yaml,toml,json}`, validates content, and
  defines the `Settings`/`Pair` types.
- `src/proxy.rs` — configures ZeroMQ sockets and calls `zmq::proxy`.
- `proxy.yaml` — sample configuration describing two broker pairs.
- `pushkind-broker.service` — hardened systemd unit file template.
- `Makefile` — `make check` helper that formats, lints, and tests.

## Configuration

At startup the broker searches for `proxy.yaml`, `proxy.toml`, or `proxy.json` in
the working directory (in that order). Each file should define at least one
pair:

```yaml
pairs:
  - name: emailer
    frontend: "tcp://127.0.0.1:5557"
    backend:  "tcp://127.0.0.1:5558"
    xsub_rcvhwm: 100000   # optional; defaults to 100_000
    xpub_sndhwm: 100000   # optional; defaults to 100_000
```

- `frontend` is the `XSUB` bind address that publishers connect to.
- `backend` is the `XPUB` bind address that subscribers connect to.
- `xsub_rcvhwm` / `xpub_sndhwm` set ZeroMQ high-water marks and default to
  `100_000` when omitted.
- Use the `name` field to label log lines for easier troubleshooting.

A configuration with no pairs is rejected to prevent accidental no-op runs.

## Running Locally

```bash
cargo run --release
```

Set `RUST_LOG=info` (or another level) to adjust log verbosity. The broker
inherits its working directory; ensure the desired `proxy.*` file is present or
set the `PWD` appropriately before launching under a supervisor.

## Development Workflow

```bash
make check             # fmt + clippy + tests
cargo build --all-targets --verbose
cargo test  --all-targets --verbose
```

The crate uses `tempfile` for configuration tests; add new cases alongside the
modules they exercise and keep panics out of production paths.

## Dev Container

Open the repository in a development container using VS Code with the
Dev Containers extension or GitHub Codespaces. The container includes the Rust
toolchain and the ZeroMQ development libraries needed to build and test the
broker.

## Deploying with systemd

The repository includes `pushkind-broker.service` as a starting point. Customize
the `WorkingDirectory`, `ExecStart`, and security hardening directives to match
your environment before installing the unit.

## License

Licensed under the MIT License – see `LICENSE` for details.
