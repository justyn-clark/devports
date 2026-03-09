# devports v0.1.0 — Initial Release

devports is a lightweight CLI for managing local development ports.

It helps developers track configured services, inspect running listeners, terminate stuck processes by port, launch configured apps, open running local apps in the browser, and print LAN-friendly URLs for other devices on the network.

## Key features

- Track configured services and expected ports
- Detect running listeners via lsof
- Kill stuck processes by port
- Launch services from configured repositories in the background
- Show LAN-accessible URLs for configured services
- Open running local services directly in the browser
- Optional terminal UI

## Example usage

```bash
devports config init

devports config add web \
  --repo ~/projects/web \
  --port 3000 \
  --start "vite --host 0.0.0.0 --port 3000"

devports start web
devports list
devports urls
devports open web
devports kill --port 3000
```

## Installation

```bash
cargo install --git https://github.com/justyn-clark/devports
```

More documentation is available in the README.
