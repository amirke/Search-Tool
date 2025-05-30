# Project History: Tauri-based Search Tool with Ripgrep

## Overview
This project is a Tauri-based desktop search tool (frontend: Svelte, backend: Rust) that leverages ripgrep for fast text search. The app is designed to be distributed as a Windows executable, runnable on other computers without requiring ripgrep to be installed system-wide.

---

## Development & Debugging Timeline

### 1. Initial Build & Distribution
- Built the Tauri app for Windows.
- Attempted to run the executable on other PCs.

#### Problem: SmartScreen Warning
- **Description:** Windows SmartScreen flagged the unsigned executable as untrusted.
- **Solution:**
  - Explored code signing options: commercial certificate, self-signed certificate, or Windows SDK.
  - Used a self-signed certificate (via PowerShell script) for development/testing. Updated Tauri config to use the certificate and rebuilt the app.
  - **Lesson:** For public distribution, a trusted certificate is required to avoid SmartScreen warnings.

### 2. Ripgrep Not Found on Other PCs
- **Description:** The app failed with "Failed to run ripgrep: program not found" on other computers.
- **Solution:**
  - Bundled the ripgrep binary with the app by downloading it and placing it in `src-tauri/bin/`.
  - Updated `tauri.conf.json` to include ripgrep in `externalBin`.
  - Ensured the correct platform-specific filename was used (e.g., `rg.exe-x86_64-pc-windows-msvc.exe`).
  - **Lesson:** Tauri expects platform-specific naming for external binaries; config and file names must match.

### 3. Build Errors & Double Extensions
- **Description:** Build errors due to Tauri appending platform suffixes, resulting in filenames like `rg.exe-x86_64-pc-windows-msvc.exe-x86_64-pc-windows-msvc.exe` or `rg.exe.exe`.
- **Solution:**
  - Used only the generic name in config and ensured the correct file existed in the bin directory.
  - Iterated on config and file naming to align with Tauri's expectations.
  - **Lesson:** Carefully follow Tauri's externalBin naming conventions to avoid double extensions.

### 4. Rust Backend Execution Issues
- **Description:** Rust backend failed to find or execute ripgrep.
- **Solution:**
  - Updated Rust code to try both generic and platform-specific binary names.
  - Added detailed error logging to a file (`search_tool_debug.log`) for debugging on user machines.
  - **Lesson:** Always log errors to a file for post-mortem debugging, especially for distributed apps.

### 5. Resource Directory Issues
- **Description:** Difficulty locating the bundled ripgrep binary in the installed app.
- **Solution:**
  - Used Tauri's `resource_dir` API in Rust, passing `tauri::AppHandle` to backend functions.
  - Imported necessary traits and handled type errors.
  - **Lesson:** Use Tauri's APIs for resource location; ensure correct types and imports in Rust.

### 6. Final Build & Testing
- **Description:** Final build succeeded; app and ripgrep were signed and bundled.
- **Solution:**
  - Ensured both `rg.exe` and `rg.exe-x86_64-pc-windows-msvc.exe` were present in the bin directory.
  - Advised to check the debug log if issues persisted on other PCs.
  - **Lesson:** Always test the final build on a clean machine; provide debug logging for users.

---

## Problem/Solution Reference

### Problem: SmartScreen/Permission Error
- **Cause:** Unsigned executable.
- **Solution:** Use a code signing certificate (self-signed for dev, trusted for prod).

### Problem: Ripgrep Not Found
- **Cause:** Binary not bundled or misnamed.
- **Solution:** Bundle ripgrep with correct name in `src-tauri/bin/` and reference in `tauri.conf.json`.

### Problem: Build Fails with Double Extensions
- **Cause:** Incorrect config or file naming.
- **Solution:** Use generic name in config, ensure correct file in bin directory.

### Problem: App Fails Silently on Other PCs
- **Cause:** Backend errors, missing binaries, or path issues.
- **Solution:** Add file-based debug logging in Rust backend.

### Problem: Resource Directory/Type Errors in Rust
- **Cause:** Incorrect use of Tauri APIs or missing imports.
- **Solution:** Use correct API and import necessary traits.

---

## Lessons Learned
- Always bundle all required binaries and test on a clean machine.
- Use debug logging for distributed apps.
- Follow Tauri's conventions for external binaries.
- Use code signing for Windows executables.
- Document all config and code changes for future reference. 