```text
██████╗ ███████╗██╗   ██╗██████╗  ██████╗ ██████╗ ████████╗███████╗
██╔══██╗██╔════╝██║   ██║██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔════╝
██║  ██║█████╗  ██║   ██║██████╔╝██║   ██║██████╔╝   ██║   ███████╗
██║  ██║██╔══╝  ╚██╗ ██╔╝██╔═══╝ ██║   ██║██╔══██╗   ██║   ╚════██║
██████╔╝███████╗ ╚████╔╝ ██║     ╚██████╔╝██║  ██║   ██║   ███████║
╚═════╝ ╚══════╝  ╚═══╝  ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝
```

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

<img width="1197" height="607" alt="devports-tui" src="https://github.com/user-attachments/assets/61c6a127-6ba0-4634-bb10-8af6a9a6bd24" />

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

## TUI controls

```text
Arrows, Home/End, PageUp/PageDown  navigate listeners
Enter or o                         open the selected service URL
k / K                              graceful terminate / force kill
s                                  start the selected configured service
r                                  rescan listeners
e                                  open the active config file
/                                  enter filter mode
?                                  show the full keyboard help
q                                  quit
```
