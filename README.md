# devports

[![Release](https://github.com/justyn-clark/devports/actions/workflows/release.yml/badge.svg)](https://github.com/justyn-clark/devports/actions/workflows/release.yml)
[![npm publish](https://github.com/justyn-clark/devports/actions/workflows/npm-publish.yml/badge.svg)](https://github.com/justyn-clark/devports/actions/workflows/npm-publish.yml)
[![GitHub release](https://img.shields.io/github/v/release/justyn-clark/devports)](https://github.com/justyn-clark/devports/releases)
[![npm version](https://img.shields.io/npm/v/%40justynclark%2Fdevports)](https://www.npmjs.com/package/@justynclark/devports)
[![Rust](https://img.shields.io/badge/rust-edition%202024-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

A CLI command center for local development services.

`devports` tracks configured local services, inspects live listeners, launches configured apps, opens only services that are actually running, and prints LAN-friendly URLs when you want to reach those apps from another device.

<img width="1197" height="607" alt="devports-tui" src="https://github.com/user-attachments/assets/61c6a127-6ba0-4634-bb10-8af6a9a6bd24" />

## Install

Recommended install paths:

- npm: `npm i -g @justynclark/devports`
- curl: `curl -fsSL https://raw.githubusercontent.com/justyn-clark/devports/main/scripts/install.sh | bash`
- Homebrew:
  - `brew tap justyn-clark/homebrew-tap`
  - `brew install devports`

Alternative developer install:

```bash
cargo install --git https://github.com/justyn-clark/devports
```

Notes:
- release automation currently ships macOS and Linux binaries
- Windows/Scoop packaging is temporarily disabled while the release pipeline is moved off the current failing Windows cross-compile path
- full install details live in [docs/installation.md](docs/installation.md)

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

## How To Use

1. Initialize the config.

```bash
devports config init
```

2. Add a service entry with a name, repo path, expected port, and start command.

```bash
devports config add web \
  --repo ~/projects/web \
  --port 3000 \
  --start "vite --host 0.0.0.0 --port 3000"
```

3. Launch the service.

```bash
devports start web
```

This returns immediately and prints the local URL, process ID, and log path. Service output is written to `.devports/start.log` inside the configured repo.

4. Open the running local app.

```bash
devports open web
```

This opens `http://127.0.0.1:<port>` only if that configured service is actually listening.

5. Inspect runtime state.

```bash
devports list
devports doctor
devports scan
```

- `list` joins live listeners with configured services
- `doctor` checks config quality and likely unmapped local dev listeners
- `scan` prints the raw listener records

6. Print LAN URLs when you want to open the app from another device.

```bash
devports urls
devports urls --host 192.168.1.50
```

If no services are configured, `devports urls` prints an explicit message instead of failing silently.

## Real Example

For a repo like `~/projects/jcn-studio-mcp`, where the project already defines a root `dev` script and Vite is configured for port `4321`, a working config entry would look like this:

```bash
devports config add jcn-studio-web \
  --repo "~/projects/jcn-studio-mcp" \
  --port 4321 \
  --start "pnpm dev"
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
o                                  open the selected local service URL
k / K                              graceful terminate / force kill
s                                  launch the selected configured service
r                                  rescan listeners
e                                  open the active config file
/                                  enter filter mode
?                                  show the full keyboard help
q                                  quit
```
