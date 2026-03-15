# Installation

## Supported platforms

Current release automation supports:

- macOS: amd64, arm64
- Linux: amd64, arm64

Temporarily disabled in release CI:

- Windows: amd64

Reason:
- the current cargo-zigbuild Windows cross-compile path fails on `libsynchronization` linkage via `windows-sys 0.61+`
- Windows artifacts and Scoop publication should be treated as unavailable until a native Windows build path or upstream-compatible toolchain fix is in place

Release asset names currently expected from CI:

- `devports-vX.Y.Z-darwin-amd64.tar.gz`
- `devports-vX.Y.Z-darwin-arm64.tar.gz`
- `devports-vX.Y.Z-linux-amd64.tar.gz`
- `devports-vX.Y.Z-linux-arm64.tar.gz`
- `checksums.txt`

## Option 1: curl installer

Install latest:

```bash
curl -fsSL https://raw.githubusercontent.com/justyn-clark/devports/main/scripts/install.sh | bash
```

Install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/justyn-clark/devports/main/scripts/install.sh | bash -s -- --version v0.1.5
```

Install behavior:

- Downloads release metadata from GitHub API
- Downloads the platform archive and `checksums.txt`
- No compilation required. Pre-built binaries are downloaded and checksum verified.
- Verifies SHA256 before extraction and install
- Installs to `~/.local/bin/devports` by default (no sudo)

Optional flags:

- `--system`: install to `/usr/local/bin/devports` (uses sudo if needed)
- `--dir PATH`: custom install directory
- `--release-json PATH`: use local release metadata (testing only)
- `--dry-run`: print resolved release/install details only

## Option 2: npm global install

```bash
npm i -g @justynclark/devports
devports --version
```

npm install behavior:

- Maps npm version `X.Y.Z` to release tag `vX.Y.Z`
- Fetches tagged release metadata from GitHub API
- Downloads platform archive and `checksums.txt`
- No compilation required. Pre-built binaries are downloaded and checksum verified.
- Verifies SHA256 before extracting `devports` into the package `vendor/` directory
- Exposes `devports` on your shell PATH via your npm global bin directory

If the binary is missing after install:

```bash
npm rebuild @justynclark/devports
```

## Option 3: Homebrew

```bash
brew tap justyn-clark/homebrew-tap
brew install devports
```

## Option 4: Scoop

Temporarily unavailable.

Scoop depends on a Windows release artifact, and Windows release packaging is currently disabled until the toolchain issue is fixed.

## PATH notes

- curl default path: `~/.local/bin/devports`
- curl system path (`--system`): `/usr/local/bin/devports`
- npm global path: `$(npm bin -g)` (or platform-equivalent npm global bin directory)

## Uninstall

curl default install:

```bash
rm -f ~/.local/bin/devports
```

curl system install:

```bash
sudo rm -f /usr/local/bin/devports
```

npm global install:

```bash
npm uninstall -g @justynclark/devports
```
