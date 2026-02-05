#!/usr/bin/env node

/*
  Thin launcher that execs the platform-specific native `ito` binary.
  This exists only for the npm install path; the Ito CLI itself remains
  a native Rust binary.
*/

const { spawnSync } = require("node:child_process");
const path = require("node:path");

function platformPackageName() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "darwin" && arch === "x64") return "@withakay/ito-darwin-x64";
  if (platform === "darwin" && arch === "arm64") return "@withakay/ito-darwin-arm64";
  if (platform === "linux" && arch === "x64") return "@withakay/ito-linux-x64";
  if (platform === "linux" && arch === "arm64") return "@withakay/ito-linux-arm64";
  if (platform === "win32" && arch === "x64") return "@withakay/ito-win32-x64";

  return null;
}

function binaryName() {
  return process.platform === "win32" ? "ito.exe" : "ito";
}

const pkg = platformPackageName();
if (!pkg) {
  console.error(
    `ito: unsupported platform (${process.platform}/${process.arch}). ` +
      "Supported: darwin x64/arm64, linux x64/arm64, win32 x64."
  );
  process.exit(1);
}

let pkgJson;
try {
  pkgJson = require.resolve(`${pkg}/package.json`);
} catch (err) {
  console.error(
    `ito: platform package ${pkg} is not installed. ` +
      "This can happen if optional dependencies were skipped."
  );
  process.exit(1);
}

const pkgDir = path.dirname(pkgJson);
const binPath = path.join(pkgDir, "bin", binaryName());

const res = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });

if (res.error) {
  console.error(`ito: failed to run ${binPath}: ${String(res.error)}`);
  process.exit(1);
}

process.exit(res.status == null ? 1 : res.status);
