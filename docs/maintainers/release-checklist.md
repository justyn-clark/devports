# Release checklist

## 1) Prepare and tag

1. Ensure CI is green on `main`.
2. Confirm version and changelog updates.
3. Create and push the tag:

```bash
git tag -a vX.Y.Z -m "devports vX.Y.Z"
git push origin vX.Y.Z
```

## 2) Verify GitHub release assets

1. Confirm release has:
   - `devports-vX.Y.Z-darwin-amd64.tar.gz`
   - `devports-vX.Y.Z-darwin-arm64.tar.gz`
   - `devports-vX.Y.Z-linux-amd64.tar.gz`
   - `devports-vX.Y.Z-linux-arm64.tar.gz`
   - `devports-vX.Y.Z-windows-amd64.zip`
   - `checksums.txt`
2. Verify `checksums.txt` includes each asset.

## 3) Verify installer paths

1. Curl installer:

```bash
curl -fsSL https://raw.githubusercontent.com/justyn-clark/devports/main/scripts/install.sh | bash -s -- --version vX.Y.Z --dir /tmp/devportsbin
PATH=/tmp/devportsbin:$PATH devports --version
```

2. npm package:

```bash
npm i -g @justyn-clark/devports@X.Y.Z
devports --version
```

3. Homebrew formula:

```bash
brew update
brew install devports
devports --version
```

4. Scoop manifest:

```powershell
scoop update
scoop install devports
devports --version
```

## 4) npm publish (OIDC-only)

1. Set `packages/npm/package.json` version to `X.Y.Z`.
2. Ensure tag is `vX.Y.Z`.
3. Configure npm Trusted Publishing (OIDC) for `@justyn-clark/devports` with this GitHub repo/workflow as trusted publisher.
4. Publish workflow must run with Node 24 and npm 11.10.1.
5. Publish with provenance:

```bash
cd packages/npm
npm publish --provenance --access public
```
