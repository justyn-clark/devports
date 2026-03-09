#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const os = require("node:os");
const https = require("node:https");
const crypto = require("node:crypto");
const { spawnSync } = require("node:child_process");

const REPO = "justyn-clark/devports";
const ROOT = path.resolve(__dirname, "..");
const VENDOR_DIR = path.join(ROOT, "vendor");

function resolvePlatform() {
  const platformMap = {
    darwin: "darwin",
    linux: "linux",
    win32: "windows"
  };

  const archMap = {
    x64: "amd64",
    arm64: "arm64"
  };

  const osName = platformMap[process.platform];
  const archName = archMap[process.arch];

  if (!osName) {
    throw new Error(`Unsupported OS for devports npm package: ${process.platform}`);
  }
  if (!archName) {
    throw new Error(`Unsupported architecture for devports npm package: ${process.arch}`);
  }
  if (osName === "windows" && archName === "arm64") {
    throw new Error("Windows arm64 is not published for devports yet");
  }

  return { osName, archName };
}

function readPackageVersion() {
  const pkgPath = path.join(ROOT, "package.json");
  const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
  if (!pkg.version) {
    throw new Error("package.json is missing version");
  }
  return pkg.version;
}

function request(url, expectJson, redirects = 0, authOrigin = "") {
  const parsed = new URL(url);
  const headers = {
    "User-Agent": "devports-npm-installer",
    Accept: expectJson ? "application/vnd.github+json" : "*/*"
  };

  const token = process.env.GITHUB_TOKEN;
  const resolvedAuthOrigin = authOrigin || (token ? parsed.origin : "");
  if (token && resolvedAuthOrigin && parsed.origin === resolvedAuthOrigin) {
    headers.Authorization = `Bearer ${token}`;
  }

  return new Promise((resolve, reject) => {
    const req = https.get(url, { headers }, (res) => {
      const chunks = [];
      res.on("data", (chunk) => chunks.push(chunk));
      res.on("end", async () => {
        const body = Buffer.concat(chunks);

        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          if (redirects >= 5) {
            reject(new Error(`Too many redirects while fetching ${url}`));
            return;
          }

          try {
            const nextUrl = new URL(res.headers.location, url).toString();
            const nextOrigin = new URL(nextUrl).origin;
            const forwardedAuthOrigin = nextOrigin === resolvedAuthOrigin ? resolvedAuthOrigin : "";
            const redirected = await request(nextUrl, expectJson, redirects + 1, forwardedAuthOrigin);
            resolve(redirected);
          } catch (err) {
            reject(err);
          }
          return;
        }

        if (res.statusCode < 200 || res.statusCode >= 300) {
          reject(new Error(`Request failed ${res.statusCode} for ${url}: ${body.toString("utf8")}`));
          return;
        }

        if (!expectJson) {
          resolve(body);
          return;
        }

        try {
          resolve(JSON.parse(body.toString("utf8")));
        } catch (err) {
          reject(new Error(`Invalid JSON from ${url}: ${err.message}`));
        }
      });
    });

    req.on("error", reject);
  });
}

function checksumForAsset(checksumsText, assetName) {
  const lines = checksumsText.split(/\r?\n/);
  for (const line of lines) {
    const match = line.match(/^([a-fA-F0-9]{64})\s+\*?(.+)$/);
    if (match && match[2] === assetName) {
      return match[1].toLowerCase();
    }
  }
  return "";
}

function findBinary(startDir, wantedName) {
  const entries = fs.readdirSync(startDir, { withFileTypes: true });
  for (const entry of entries) {
    const full = path.join(startDir, entry.name);
    if (entry.isFile() && entry.name === wantedName) {
      return full;
    }
    if (entry.isDirectory()) {
      const nested = findBinary(full, wantedName);
      if (nested) {
        return nested;
      }
    }
  }
  return "";
}

function validateArchiveEntries(archivePath, isZip) {
  const args = isZip ? ["-Z1", archivePath] : ["-tzf", archivePath];
  const listing = spawnSync(isZip ? "unzip" : "tar", args, { encoding: "utf8" });

  if (listing.status !== 0) {
    throw new Error("Failed to list archive entries");
  }

  const entries = listing.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  for (const entry of entries) {
    const stripped = entry.replace(/^\.\/+/, "");
    if (!stripped) {
      continue;
    }
    const normalized = path.posix.normalize(stripped);
    if (normalized === ".." || normalized.startsWith("../") || path.posix.isAbsolute(stripped) || normalized.includes("/../")) {
      throw new Error(`Archive contains unsafe path entry: ${entry}`);
    }
  }
}

async function main() {
  const version = readPackageVersion();
  const tag = `v${version}`;
  const { osName, archName } = resolvePlatform();
  const isWindows = osName === "windows";
  const extension = isWindows ? "zip" : "tar.gz";
  const binaryName = isWindows ? "devports.exe" : "devports";
  const assetName = `devports-${tag}-${osName}-${archName}.${extension}`;

  const release = await request(`https://api.github.com/repos/${REPO}/releases/tags/${tag}`, true);
  const asset = (release.assets || []).find((item) => item.name === assetName);
  if (!asset) {
    throw new Error(`Release ${tag} is missing asset ${assetName}`);
  }

  const checksumsAsset = (release.assets || []).find((item) => item.name === "checksums.txt");
  if (!checksumsAsset) {
    throw new Error(`Release ${tag} is missing checksums.txt`);
  }

  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "devports-npm-"));
  const archivePath = path.join(tempRoot, assetName);
  const checksumsPath = path.join(tempRoot, "checksums.txt");
  const extractDir = path.join(tempRoot, "extract");

  try {
    const archiveBody = await request(asset.browser_download_url, false);
    fs.writeFileSync(archivePath, archiveBody);

    const checksumsBody = await request(checksumsAsset.browser_download_url, false);
    fs.writeFileSync(checksumsPath, checksumsBody);

    const expected = checksumForAsset(fs.readFileSync(checksumsPath, "utf8"), assetName);
    if (!expected) {
      throw new Error(`checksums.txt does not contain an entry for ${assetName}`);
    }

    const actual = crypto.createHash("sha256").update(fs.readFileSync(archivePath)).digest("hex");
    if (expected !== actual) {
      throw new Error(`Checksum mismatch for ${assetName}`);
    }

    validateArchiveEntries(archivePath, isWindows);
    fs.mkdirSync(extractDir, { recursive: true });

    const extract = isWindows
      ? spawnSync("unzip", ["-q", archivePath, "-d", extractDir], { stdio: "inherit" })
      : spawnSync("tar", ["-xzf", archivePath, "--no-same-owner", "-C", extractDir], { stdio: "inherit" });
    if (extract.status !== 0) {
      throw new Error("Failed to extract devports archive");
    }

    const binary = findBinary(extractDir, binaryName);
    if (!binary) {
      throw new Error(`Extracted archive did not contain ${binaryName}`);
    }

    fs.mkdirSync(VENDOR_DIR, { recursive: true });
    const vendorBinary = path.join(VENDOR_DIR, binaryName);
    fs.copyFileSync(binary, vendorBinary);
    fs.chmodSync(vendorBinary, 0o755);
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
}

main().catch((err) => {
  console.error(`[devports npm] ${err.message}`);
  process.exit(1);
});
