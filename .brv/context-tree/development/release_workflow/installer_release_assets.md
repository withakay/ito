---
title: Installer Release Assets
summary: Installer scripts prefer cargo-dist release assets for current releases and fall back to legacy version-pinned archives when needed, verifying SHA-256 checksums before extracting and copying the binary.
tags: []
related: [development/release_workflow/context.md, development/release_workflow/release_workflow.md, development/release_workflow/build_and_coverage_guardrails.md, development/release_workflow/manifesto_instruction_implementation_notes.md, development/release_workflow/release_plz_guardrails.md]
keywords: []
createdAt: '2026-05-29T08:43:40.712Z'
updatedAt: '2026-05-29T08:43:40.712Z'
---
## Reason
Document installer asset naming, archive selection, checksum verification, and fallback behavior for Unix and Windows installers

## Raw Concept
**Task:**
Document installer release asset selection and installation flow for Unix and Windows scripts

**Changes:**
- Preferred cargo-dist release asset naming for current releases
- Kept legacy fallback for older version-pinned installs
- Verified archive integrity with SHA-256 before extraction
- Copied the extracted ito binary into the user install directory

**Files:**
- scripts/install.sh
- scripts/install.ps1

**Flow:**
detect platform -> determine target triple -> resolve tag/version -> download primary archive and checksum -> fallback to legacy archive if needed -> verify SHA-256 -> extract -> locate binary -> copy to install dir

**Timestamp:** 2026-05-29T08:43:12.331Z

## Narrative
### Structure
The Unix shell installer resolves the latest release tag through the GitHub Releases API, downloads a target-specific archive, verifies its checksum, extracts it, and copies ito into the install directory. The PowerShell installer mirrors this flow on Windows, using Invoke-RestMethod and Invoke-WebRequest, expanding the ZIP archive, and optionally adding the install directory to the user PATH.

### Dependencies
Both installers depend on GitHub release artifacts and checksum files. The shell script requires tar, sed, awk, head, and curl; the PowerShell script uses standard PowerShell cmdlets and the Windows file hash utility.

### Highlights
Primary assets are cargo-dist style archives, while legacy fallback preserves older release compatibility. The Windows installer also supports an AddToPath switch for updating the user PATH.

### Rules
Unix installer rules:
1. Use latest release tag unless ITO_VERSION is set.
2. Prefer ito-cli-${TARGET}.tar.xz and its .sha256 checksum.
3. If primary download fails, fall back to ito-${TAG}-${TARGET}.tar.gz and its checksum.
4. Verify checksum before extraction.
5. Extract archive and copy the ito binary to the install directory.

Windows installer rules:
1. Use the provided Version or the latest release tag.
2. Normalize tags to include a leading v.
3. Prefer ito-cli-$target.zip and its .sha256 checksum.
4. If primary download fails, fall back to ito-$tag-$target.zip and its checksum.
5. Verify checksum before extraction.
6. Expand the ZIP archive, locate ito.exe, and copy it to the install directory.
7. If AddToPath is set, append the install directory to the user PATH if it is not already present.

### Examples
Example Unix asset: ito-cli-x86_64-apple-darwin.tar.xz
Example Windows asset: ito-cli-x86_64-pc-windows-msvc.zip
Example legacy fallback: ito-v1.2.3-x86_64-unknown-linux-gnu.tar.gz

## Facts
- **unix_release_asset**: Unix installer prefers ito-cli-<target>.tar.xz with a matching .tar.xz.sha256 checksum. [project]
- **windows_release_asset**: Windows installer prefers ito-cli-<target>.zip with a matching .zip.sha256 checksum. [project]
- **legacy_fallback_asset**: Both installers fall back to legacy version-pinned archives named ito-v<tag>-<target>.tar.gz/.zip with matching .sha256 files. [project]
- **unix_platform_support**: The Unix installer supports macOS and Linux only. [project]
- **windows_target**: The Windows installer targets AMD64 and maps it to x86_64-pc-windows-msvc. [project]
- **checksum_verification**: Checksum verification is performed before extraction and installation on both platforms. [project]
