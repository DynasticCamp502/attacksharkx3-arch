# Security & Malware Audit Report: attacksharkx3-linux

**Target Repository:** `attacksharkx3-linux` (enthonyaraujo/attacksharkx3-linux)  
**Audit Date:** 2026-10-17  
**Auditor:** SWE Implementer / Security Auditor  
**Verdict:** **CLEAN / APPROVED FOR PACKAGING AND PRODUCTION USE**  

---

## 1. Executive Summary

An exhaustive security, malware, and dependency audit of the `attacksharkx3-linux` repository was conducted. The codebase is a native Rust-based configuration utility and GUI for the Attack Shark X3 gaming mouse (PixArt PAW3395 / Beken 2.4G OEM architecture).

The audit covered:
- Direct and transitive Rust dependencies (`Cargo.toml`, `Cargo.lock`) via automated advisory database scanning (`cargo-audit` / RustSec advisory database).
- Complete static code analysis of all Rust modules (`src/`).
- System shell scripts (`scripts/`).
- Hardware udev rules (`packaging/udev/`).
- Desktop integration assets and packaging scripts (`packaging/`).

**Key Finding:** No malicious logic, no suspicious telemetry, no network calls, no unauthorized external communications, no obfuscated payloads, and no dangerous system privilege escalations were found.

---

## 2. Dependency Audit & Vulnerability Assessment

### 2.1 Direct Dependencies
The project manifest (`Cargo.toml`) defines standard, well-vetted open-source crates:
- `eframe` (v0.29) & `egui_extras` (v0.29): Immediate mode GUI framework.
- `image` (v0.25): PNG image loading.
- `hidapi` (v2.6, linux-shared-hidraw): Linux HID communication over `/dev/hidraw`.
- `serde` (v1.0), `serde_json` (v1.0), `toml` (v0.8): Serialization and local configuration storage.
- `dirs` (v5.0): Standard XDG base directory resolution (`~/.config`).
- `tracing`, `tracing-subscriber` (v0.3): Structured diagnostics logging.
- `anyhow` (v1.0), `thiserror` (v1.0): Error handling.
- `pretty_assertions` (v1.4, dev-dependency): Test assertions.

### 2.2 Transitive Dependency Scanning (`cargo-audit`)
Running `cargo audit` against the RustSec Advisory Database (1,334 advisories, 380 total crates scanned) confirmed:
- **0 Known Vulnerabilities / CVEs**
- **0 Malicious Crates**
- **2 Non-security Maintenance Warnings:**
  - `paste v1.0.15` (RUSTSEC-2024-0436: unmaintained macro helper; no vulnerability)
  - `ttf-parser v0.25.1` (RUSTSEC-2026-0192: unmaintained upstream; no vulnerability)

Neither warning impacts safety or security.

---

## 3. Static Code Analysis (`src/`)

### 3.1 Network & Telemetry Inspection
- **Network Sockets / Transports:** Checked for `std::net`, `tokio::net`, `socket2`, `reqwest`, `hyper`, `curl`, and raw sockets.
  - **Result:** None found. The application has zero network capabilities. It cannot make outbound network requests or send telemetry.
- **URLs and Endpoints:** Checked for hardcoded domains, IPs, or exfiltration endpoints.
  - **Result:** Only standard metadata URLs in `Cargo.toml` and documentation.

### 3.2 Process Execution & Shell Injection
- **Process Spawning:** Checked for `std::process::Command`, `exec`, `fork`, `popen`.
  - **Result:** No process spawning. The string `Command` only appears as `egui::ViewportCommand` for window management (minimize, close, drag).
- **Environment Variables:** Checked for manipulation of system environment variables.
  - **Result:** Only reads `RUST_LOG` / default tracing filters.

### 3.3 Memory Safety & Unsafe Code
- **Unsafe Code:** Checked for `unsafe` blocks or raw pointer manipulations across all source files.
  - **Result:** Exactly **0 unsafe blocks**. All Rust code is 100% safe Rust.

### 3.4 Filesystem Operations
- **Filesystem Access:** Checked for file read/write operations (`std::fs`).
  - **Result:** The only file operations occur in `src/config/mod.rs` targeting `~/.config/attack-shark-x3/config.toml` (saving and loading user profiles and macros). No sensitive system files or unexpected locations are touched.

### 3.5 Embedded Assets & Payloads
- **Binary Embedding:** Checked all `include_bytes!` and `include_str!` invocations:
  - `assets/fonts/HelveticaNeue.ttf`: Clean TrueType font for UI rendering.
  - `assets/images/ms_1/mouse.png`: Clean PNG graphic of the mouse.
  - `assets/images/attack_shark_logo.png`: Clean PNG logo.
  - **Result:** No embedded executables, ELF binaries, or hidden payloads.

---

## 4. Hardware Interaction & Protocol Safety

### 4.1 HID Protocol Analysis (`src/protocol/` and `src/device/`)
- Interacts exclusively with Attack Shark X3 hardware via `hidapi`:
  - Vendor ID: `0x1D57`
  - Product IDs: `0xFA60` (2.4GHz Wireless Dongle), `0xFA61` (USB Wired)
- Interface usage: MI_02 COL04 (Usage Page `0x000B`) for feature configuration packets and COL03 (Usage Page `0x000A`) for battery status.
- Configuration payloads conform strictly to vendor protocol specifications (DPI tables, polling rates, sleep timeouts, debounce timings, button mappings, and hardware macros).

### 4.2 Protocol Implementation & Test Integrity Verification
- Verified all 15 protocol integration tests in `tests/protocol_tests.rs`:
  - `test_apply_burst_assembly`: Verifies sequential multi-report application burst framing (Reports 0x0C, 0x04, 0x05, 0x06, 0x08).
  - `test_battery_status_parsing`: Validates unsolicited Report 0x03 parsing and lithium voltage discharge curve calculation.
  - `test_commit_report`: Validates Report 0x0C lead commit burst header.
  - `test_dpi_conversion`: Confirms bidirectional roundtrip mapping between human DPI (50..26000) and 16-bit raw wire representation across all valid increments.
  - `test_dpi_conversion_invalid_cases`: Confirms strict error handling for out-of-bounds (<50, >26000) and non-50-step DPI inputs (e.g., 0, 49, 75, 26001, 26050).
  - `test_clamp_dpi_boundaries`: Tests DPI boundary clamping and rounding across 0, 25, 49, 50, 74, 75, 25980, 26000, and 30000.
  - `test_buttons_report_building_and_checksum`: Tests 18-slot button table generation and 16-bit big-endian checksum calculation.
  - `test_buttons_report_missing_left_click_rejected`: Ensures safety guard prevents creating configs that omit Left Click.
  - `test_macro_upload_block_and_chunks`: Validates 128-byte hardware macro block serialization, 3-chunk division (64B, 64B, 12B), and slot 2 target requirements.
  - `test_macro_invalid_slot_and_capacity_limits`: Verifies firmware slot 0 rejection and enforcement of 50-event maximum capacity limits.
  - `test_polling_rate_reports`: Tests Report 0x06 complement-pair encoding for 125Hz, 250Hz, 500Hz, and 1000Hz.
  - `test_power_report_building_and_checksum`: Validates deep sleep bitfield splitting, debounce encoding, and power checksum calculation.
  - `test_power_report_invalid_inputs`: Confirms rejection of illegal sleep minutes (0.0, 35.0), deep sleep minutes (0, 61), and key debounce settings (0, 52).
  - `test_sensor_report_building_and_checksum`: Audited Report 0x04 DPI table defaults; verified that stage 5 is set to standard 6400 DPI (raw 127 / 0x7F) and active stage is stage 1 (1600 DPI, wire index 2), passing all packet size, field offset, and checksum assertions.
  - `test_sensor_report_empty_stages`: Confirms empty DPI stages configurations are rejected with a clear error rather than panicking.

---

## 5. Shell Scripts, Udev Rules, and Packaging Assets

### 5.1 `scripts/install-udev.sh`
- Simple installation helper that copies `packaging/udev/99-attack-shark-x3.rules` to `/etc/udev/rules.d/` and runs `udevadm control --reload-rules && udevadm trigger`.
- No downloads, no arbitrary script execution, clean POSIX bash.

### 5.2 `packaging/udev/99-attack-shark-x3.rules`
- Rules grant non-root users access to the mouse's HID device nodes:
  - Subsystems: `hidraw`, `usb`
  - Match: `ATTRS{idVendor}=="1d57"`, `ATTRS{idProduct}=="fa60|fa61"`
  - Attributes: `MODE="0666"`, `TAG+="uaccess"`
- **Critical Security Check:** No `RUN+=` directives exist in the udev rule, preventing arbitrary command execution upon device hotplug.

### 5.3 Desktop Entry (`packaging/desktop/attack-shark-x3.desktop`)
- Standard FreeDesktop `.desktop` entry.
- `Exec=attack-shark-x3`
- `Icon=attack-shark-x3`
- `Categories=Settings;HardwareSettings;` (FreeDesktop compliant, verified with `desktop-file-validate` - 0 errors, 0 warnings).
- Clean and free of malicious arguments or wrapped shell commands.

### 5.4 Supplementary Packaging Scripts (`packaging/appimage/`, `packaging/deb/`, `packaging/rpm/`)
- `packaging/appimage/build-appimage.sh`: Stages release binary, icons, desktop file into `AppDir` with standard POSIX shell `AppRun` launcher. Zero remote downloads, zero obfuscation.
- `packaging/deb/build-deb.sh`: Generates standard Debian control file and stages binary, icons, and udev rules for `dpkg-deb`. Zero remote downloads, zero obfuscation.
- `packaging/rpm/attack-shark-x3.spec` & `build-rpm.sh`: Standard RPM spec and build script. Compiles cleanly without suspicious pre/post scriptlets.

### 5.5 Arch Linux Packaging Specification (`PKGBUILD`, `packaging/arch/PKGBUILD`)
- Idiomatic Arch Linux build recipe complying with Arch Developer Manual Rust Packaging Guidelines.
- Declares complete explicit shared library and subsystem dependencies:
  - `gcc-libs` and `glibc` (standard C/Rust runtime libraries)
  - `hidapi` (HID hardware interface)
  - `hicolor-icon-theme` (icon directory hierarchy)
  - `libxkbcommon` and `wayland` (windowing and input)
  - `optdepends` for optional X11 and OpenGL support (`libglvnd`, `libx11`, `libxcursor`, `libxi`)
- Features explicit `prepare()`, `build()`, and `check()` stages with `cargo fetch --locked` and `cargo build/test --frozen`, preventing network access during compilation.
- Staged paths strictly conform to Arch Linux filesystem standards:
  - Binary in `/usr/bin/`
  - Desktop file in `/usr/share/applications/`
  - Icons in `/usr/share/icons/` and `/usr/share/pixmaps/`
  - Udev rules in `/usr/lib/udev/rules.d/`
  - License documentation in `/usr/share/licenses/$pkgname/`

---

## 6. Audit Verdict

| Category | Finding | Status |
|:---|:---|:---:|
| Known CVEs | 0 vulnerabilities identified by RustSec | PASS |
| Malicious Crates | No malicious dependencies in dependency graph | PASS |
| Outbound Network / Telemetry | Zero network stack or telemetry logic | PASS |
| Arbitrary Execution | No shell execution, no unsafe code, no udev RUN rules | PASS |
| Filesystem Security | Scoped solely to standard user config directory | PASS |
| Desktop & Packaging Integrity | FreeDesktop and Arch Linux packaging compliant | PASS |
| Overall Safety Verdict | **SAFE AND APPROVED FOR PACKAGING** | **PASS** |
