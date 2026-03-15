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

Current expected assets from CI:
1. Confirm release has:
   - `devports-vX.Y.Z-darwin-amd64.tar.gz`
   - `devports-vX.Y.Z-darwin-arm64.tar.gz`
   - `devports-vX.Y.Z-linux-amd64.tar.gz`
   - `devports-vX.Y.Z-linux-arm64.tar.gz`
   - `checksums.txt`
2. Verify `checksums.txt` includes each asset.
3. Do not expect a Windows asset until the Windows build path is reintroduced.

## 3) Verify installer paths

1. Curl installer:

```bash
curl -fsSL https://raw.githubusercontent.com/justyn-clark/devports/main/scripts/install.sh | bash -s -- --version vX.Y.Z --dir /tmp/devportsbin
PATH=/tmp/devportsbin:$PATH devports --version
```

2. npm package:

```bash
npm i -g @justynclark/devports@X.Y.Z
devports --version
```

3. Homebrew formula:

```bash
brew update
brew install devports
devports --version
```

4. Scoop manifest:

Temporarily skipped. Scoop depends on a Windows release artifact, and Windows packaging is currently disabled pending a replacement build path.

## 4) npm publish

Current state:
- `@justynclark/devports` publishes successfully from GitHub Actions
- the repo currently uses a bootstrap `NPM_TOKEN` secret for npm publish
- target end state is npm Trusted Publishing (OIDC-only), after verifying the trusted publisher binding for this package/workflow

Checklist:
1. Set `packages/npm/package.json` version to `X.Y.Z`.
2. Ensure tag is `vX.Y.Z`.
3. Confirm `.github/workflows/npm-publish.yml` still validates package name, version, and npm auth identity.
4. If still in bootstrap mode, ensure repo secret `NPM_TOKEN` exists and is a publish-capable token that bypasses 2FA.
5. If OIDC has been configured for `@justynclark/devports`, verify the trusted publisher is bound to this repo/workflow before removing `NPM_TOKEN`.
6. Publish with provenance:

```bash
cd packages/npm
npm publish --provenance --access public
```
