# jcn-ports - Port Manager

`ports` is a macOS-first Rust CLI + TUI for managing local dev service ports from an explicit registry.

## Install

```bash
cargo build
```

Binary path:

```bash
./target/debug/ports
```

## Config

Default config path: `~/.jcn/ports.yml` (override with `--config`).

```yaml
services:
  justbeatz-web:
    repo: /Users/justin/Documents/Justyn Clark Network/REPOS/justbeatz-web
    port: 3000
    start: bun run dev
  justbeatz-api:
    repo: /Users/justin/Documents/Justyn Clark Network/REPOS/justbeatz-api
    port: 3001
    start: bun run dev
  clientbrief-web:
    repo: /Users/justin/Documents/Justyn Clark Network/REPOS/clientbrief
    port: 3003
    start: bun run dev
```

## Commands

```bash
ports scan
ports list [--json]
ports tui
ports kill --port 3000 [--hard] [--timeout-ms 1500]
ports start <service>
ports doctor [--json]
ports config path
```

## Behavior

- `scan`: prints JSON listeners resolved from `lsof`/`ps` and process cwd/repo metadata.
- `list`: table join of live listeners with configured services.
- `kill`: targets PGID first (`TERM` then `KILL` after timeout), with PID tree fallback.
- `start`: strict port preflight; exits non-zero if target port is already listening.
- `doctor`: flags missing repos, missing start commands, duplicate configured ports, and unmapped listeners.
- `tui`: hotkeys `k`, `K`, `s`, `r`, `e`, `/`, `q`.

## Test

```bash
cargo test
```
