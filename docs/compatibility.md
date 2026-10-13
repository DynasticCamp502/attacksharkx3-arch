# Linux Desktop & Hardware Compatibility Guide

## 1. Supported Hardware
- **Attack Shark X3 Gaming Mouse** (PixArt PAW3395 sensor, BK3633 MCU / Beken OEM):
  - 2.4 GHz USB Wireless Receiver (`VID 0x1D57`, `PID 0xFA60`)
  - USB Type-C Cable (`VID 0x1D57`, `PID 0xFA61`)
  - Bluetooth mode (BLE) connection detection

## 2. Linux Distributions Tested & Supported
- **Fedora 40, 41, 42, 43, 44**
- **Ubuntu 22.04 LTS, 24.04 LTS, 26.04 LTS**
- **Debian 12 (Bookworm), 13 (Trixie)**
- **Arch Linux / Manjaro / EndeavourOS**
- **openSUSE Tumbleweed / Leap**

## 3. Display Server Environments
- **Wayland**: Fully supported natively through `eframe` Wayland backend.
- **X11 / Xorg**: Fully supported natively.
- Desktop environments: GNOME, KDE Plasma, XFCE, Sway, Hyprland, Cosmic.

## 4. Permissions & Udev
To allow configuring the mouse without root/sudo privileges:
```bash
sudo cp packaging/udev/99-attack-shark-x3.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```
