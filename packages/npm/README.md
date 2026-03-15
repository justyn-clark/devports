# @justynclark/devports

`devports` is a fast local-dev command center for people who regularly run multiple apps, APIs, dashboards, and background services.

It helps you:
- track the services you actually care about
- see what is live on your machine
- start configured apps in the background
- open only the apps that are really running
- print LAN-friendly URLs for testing on phones, tablets, and other devices

## Why it is useful

If you bounce between Vite, Remix, Next.js, APIs, admin panels, and local tools, your machine turns into a pile of mystery ports fast.

`devports` gives you one place to:
- map service names to repos, ports, and start commands
- inspect live listeners alongside your configured services
- avoid guessing which localhost tab belongs to which project
- share the right hostname-based URL on your local network

## Install

```bash
npm i -g @justynclark/devports
devports --version
```

This npm package installs the native `devports` binary for your platform.

## Quickstart

Initialize config:

```bash
devports config init
```

Add a service:

```bash
devports config add web \
  --repo ~/projects/web \
  --port 3000 \
  --start "vite --host 0.0.0.0 --port 3000"
```

Start it, inspect it, and open it:

```bash
devports start web
devports list
devports open web
devports urls
```

## Core commands

```bash
devports scan
devports list
devports tui
devports start <service>
devports open <service>
devports urls
devports doctor
devports config init
devports config add <name> --repo <path> --port <port> [--start <command>]
```

## What npm install does

- maps npm version `X.Y.Z` to GitHub release tag `vX.Y.Z`
- downloads the matching native release archive from GitHub Releases
- verifies SHA256 checksums before extraction
- stores the binary inside the package `vendor/` directory
- exposes `devports` on your shell PATH through your npm global bin directory

## Supported platforms

Current published release assets support:

- macOS: amd64, arm64
- Linux: amd64, arm64

Windows packaging is temporarily unavailable while the release pipeline is migrated away from the current failing cargo-zigbuild Windows cross-compile path.

## More docs

- GitHub repo: https://github.com/justyn-clark/devports
- Installation guide: https://github.com/justyn-clark/devports/blob/main/docs/installation.md
- Release assets: https://github.com/justyn-clark/devports/releases

If the binary is missing after install:

```bash
npm rebuild @justynclark/devports
```
