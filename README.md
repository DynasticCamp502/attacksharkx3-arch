# attacksharkx3-arch

Arch Linux packaging repository for the **Attack Shark X3** gaming mouse driver and configuration GUI.

- **Package Maintainer:** [DynasticCamp502](https://github.com/DynasticCamp502)
- **Original Author & Upstream Project:** [Enthony Araujo](https://github.com/enthonyaraujo) / [attacksharkx3-linux](https://github.com/enthonyaraujo/attacksharkx3-linux)
- **License:** MIT

---

## Overview

This repository contains the PKGBUILD and packaging files to build and install `attack-shark-x3` on Arch Linux.

- **Desktop Integration:** Installs `.desktop` menu launcher and icons.
- **Rootless Hardware Access:** Installs udev rule (`99-attack-shark-x3.rules`) to allow user-space access to mouse HID endpoints.
- **Build Fixes:** Includes patch for Rust 2024 / egui compatibility with the upstream release.

## Installation

### 1. Build and install with makepkg

```bash
git clone https://github.com/DynasticCamp502/attacksharkx3-arch.git
cd attacksharkx3-arch
makepkg -si
```

### 2. Activate udev rules

To allow the application to access the mouse without root/sudo, reload udev rules or reconnect your mouse dongle:

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

## Credits & License

- Original implementation: Copyright (c) 2026 Enthony under the MIT License ([enthonyaraujo/attacksharkx3-linux](https://github.com/enthonyaraujo/attacksharkx3-linux)).
- Arch Linux packaging: Copyright (c) 2026 DynasticCamp502 under the MIT License.

See [LICENSE](LICENSE) for full details.
