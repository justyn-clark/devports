<img width="1000" height="228" alt="ascii-art-text" src="https://github.com/user-attachments/assets/def1a5a6-fc89-4ecf-b63f-e1616bfdff8a" />

# devports

A lightweight CLI for managing local development ports.

devports tracks which services should run on which ports,
helps you kill stuck processes, and exposes LAN URLs so
your dev servers are reachable across your network.

Perfect for:

- multi-repo dev environments
- tmux workflows
- AI agents running services (Ace, etc.)
- local homelabs

## Install

```bash
cargo install --git https://github.com/justyn-clark/devports
```

## Example config

```yaml
services:
  web:
    repo: ~/projects/web
    port: 3000
    start: vite --host 0.0.0.0 --port 3000
```

## Quickstart

```bash
devports config init

devports config add web \
  --repo ~/projects/web \
  --port 3000 \
  --start "vite --host 0.0.0.0 --port 3000"

devports start web
devports list
devports urls
```

## Commands

```bash
devports scan
devports list [--json]
devports tui
devports kill --port 3000 [--hard] [--timeout-ms 1500]
devports start <service>
devports doctor [--json]
devports urls [--host <host>]
devports open <service>
devports config path
devports config init [--force]
devports config add <name> --repo <path> --port <port> [--start <command>]
```
