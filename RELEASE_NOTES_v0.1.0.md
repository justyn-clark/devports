# devports v0.1.0 — Initial Release

devports is a lightweight CLI for managing local development ports.

It helps developers track which services should run on which ports, kill stuck processes, and expose LAN URLs so development servers are reachable across the network.

## Key features

- Track configured services and expected ports
- Detect running listeners via lsof
- Kill stuck processes by port
- Start services from configured repositories
- Show LAN-accessible URLs
- Open services directly in the browser
- Optional terminal UI
- Works great with tmux and multi-service dev environments

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
devports kill 3000
```

## Installation

```bash
cargo install --git https://github.com/justyn-clark/devports
```

More documentation is available in the README.
