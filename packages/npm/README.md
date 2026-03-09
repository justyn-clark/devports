# @justyn-clark/devports

This package installs the native `devports` binary for your platform and exposes it as `devports`.

## Install

```bash
npm i -g @justyn-clark/devports
devports --version
```

## Behavior

- Maps npm version `X.Y.Z` to GitHub release tag `vX.Y.Z`
- Downloads `checksums.txt` and the platform archive from GitHub Releases
- Verifies SHA256 before extracting
- Stores the binary in `vendor/devports` or `vendor/devports.exe`
