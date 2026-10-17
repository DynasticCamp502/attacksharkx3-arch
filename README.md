# attacksharkx3-arch

Native Linux configuration driver and GUI for the **Attack Shark X3** gaming mouse.

- **Maintainer & Packager:** [DynasticCamp502](https://github.com/DynasticCamp502)
- **Original Software:** Based on [enthonyaraujo/attacksharkx3-linux](https://github.com/enthonyaraujo/attacksharkx3-linux) by [Enthony Araujo](https://github.com/enthonyaraujo)
- **License:** MIT

---

## Features

- Complete native desktop GUI for configuring DPI stages, polling rate, debounce time, sleep timeout, button mappings, and macros.
- Hardware udev rules (`99-attack-shark-x3.rules`) for non-root access to mouse HID device nodes.
- FreeDesktop `.desktop` launcher and high-resolution application icons.
- Fully audited and fixed for Rust compatibility and protocol calibration.

---

## Installation on Arch Linux

### 1. Build and install with makepkg

```bash
git clone https://github.com/DynasticCamp502/attacksharkx3-arch.git
cd attacksharkx3-arch
makepkg -si
```

### 2. Post-installation: Reload udev rules

To configure the mouse without `sudo`, activate the udev rules:

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```
*(или переподключите беспроводной USB-донгл или кабель мыши).*

---

## Security & Verification

This codebase has been thoroughly audited:
- **0 CVEs**, **0 malicious dependencies** across all 380 crates scanned via the RustSec Advisory Database.
- **Zero outbound network connections**, telemetry, or phone-home capabilities.
- **Zero unsafe blocks** (100% Safe Rust).
- Complete audit details are documented in [AUDIT_REPORT.md](AUDIT_REPORT.md).

---

## Credits & License

- Original implementation: Copyright (c) 2026 Enthony under the MIT License ([enthonyaraujo/attacksharkx3-linux](https://github.com/enthonyaraujo/attacksharkx3-linux)).
- Arch Linux packaging and maintenance: Copyright (c) 2026 DynasticCamp502 under the MIT License.

See [LICENSE](LICENSE) for full details.
