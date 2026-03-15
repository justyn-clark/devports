# Changelog

All notable changes to this project should be documented in this file.

## [0.1.5] - Align release version and supported surfaces

### Fixed

- Realigned Cargo, npm, and tag versioning after the failed `v0.1.5` publish mismatch
- Kept release documentation aligned with the currently supported macOS/Linux publish surfaces

## [0.1.4] - Stabilize release CI and refresh release surfaces

### Fixed

- Made service startup shell spawning portable across macOS, Linux, and Windows command launch paths
- Replaced the flaky background launch timing test with a deterministic handshake-based test for CI
- Unblocked GitHub Release builds on Ubuntu runners by removing the hard dependency on `zsh`

### Changed

- Refreshed release-facing package and install metadata so published surfaces stay aligned with the current package identity and install paths

## [0.1.3] - Improve npm package presentation

### Changed

- Upgraded npm package description, keywords, homepage, and issue metadata
- Rewrote npm README to better explain the product, workflow, and value
- Polished the npm package page so the published listing is clearer and more compelling

## [0.1.0] - Initial release

### Added

- Config-backed service registry with expected ports and optional start commands
- Listener discovery and ownership inspection
- Port-based process termination with graceful and force-kill paths
- Background launch for configured services with per-repo start logs
- Local browser open for configured running services
- LAN URL rendering for configured services
- Interactive terminal UI for browsing listeners and launching common actions
