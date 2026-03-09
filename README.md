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

`devports` tracks configured local services, inspects live listeners, launches configured apps, opens running local apps in the browser, and prints LAN-friendly URLs when you want to reach those apps from another device.

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

`devports start <service>` launches the configured command in the background from the service repo and writes output to `.devports/start.log`.

`devports open <service>` opens the local app at `http://127.0.0.1:<port>`, but only when that configured service is actually listening.

`devports urls` is the LAN/share surface. It prints hostname-based URLs for configured services and does not change what `open` launches locally.

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
o                                  open the selected local service URL
k / K                              graceful terminate / force kill
s                                  launch the selected configured service
r                                  rescan listeners
e                                  open the active config file
/                                  enter filter mode
?                                  show the full keyboard help
q                                  quit
```
