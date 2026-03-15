#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const binaryPath = path.resolve(__dirname, "..", "vendor", process.platform === "win32" ? "devports.exe" : "devports");

if (!fs.existsSync(binaryPath)) {
  console.error("devports binary missing. Reinstall or run npm rebuild @justynclark/devports");
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

if (result.error) {
  console.error(`Failed to launch devports: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status === null ? 1 : result.status);
