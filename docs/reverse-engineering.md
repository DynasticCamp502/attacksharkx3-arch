# Reverse Engineering Report: ATTACK SHARK X3 SOFT.exe

## 1. Executable Identity & Packaging Analysis

- **Target File**: `ATTACK SHARK X3 SOFT.exe`
- **File Format**: PE32 executable for MS Windows (GUI), Intel i386 (32-bit), 10 sections.
- **Packer / Wrapper**: Inno Setup Setup Data (v6.1.0) (Unicode).
  - Overlay size: ~2.7 MB starting at byte offset `1,232,384`.
  - Compression: `zlb\x1a` stream containing the installed application payloads.
- **Extracted Application Core**:
  - `X3.exe` (631 KiB, PE32 GUI executable, build date Jan 2024 / Nov 2025).
  - `hiddriver_1.dll` (79.5 KiB) — USB HID communication wrapper for 2.4 GHz wireless receiver.
  - `hiddriver_2.dll` (79.5 KiB) — USB HID communication wrapper for wired USB connection.
  - `hidapi.dll` (82.5 KiB) — Official Signal 11 / libusb HIDAPI library.
  - `DuiLib.dll` (1.12 MiB) — DirectUI library for C++ (GUI framework).
  - `Hook.dll` (160 KiB) — Low-level keyboard/mouse hook for recording macro sequences.
  - `res/` — DirectUI XML layouts (`main.xml`, `MS/MS_1/ms_1_page.xml`, `lan.xml`) and complete PNG skins.

## 2. Architecture & Frameworks

- **GUI Framework**: DuiLib (DirectUI Lib, open-source C++ UI library using XML layout definitions and PNG skins).
- **Communication Layer**:
  - `hiddriver_1.dll` and `hiddriver_2.dll` export:
    - `Set_VIDPID(vid, pid)`
    - `Open_FeatureDevice()`
    - `Close_FeatureDevice()`
    - `SetFeature(report_buffer, length)`
    - `GetFeature(report_buffer, length)`
    - `Hid_Read()`, `Hid_Write()`
    - `Open_DevMonitor()`, `Close_DevMonitor()`
  - Uses Windows `HidD_SetFeature` to send configuration reports.
- **Local Persistence**:
  - Windows stores profiles in `%APPDATA%\Attack SharkX3Mouse\ms_1\pro.data`.
  - Windows stores macros in `%APPDATA%\Attack SharkX3Mouse\ms_1\macro.data`.
  - The hardware does NOT support read-back (`HidD_GetFeature` is not supported for configuration reports). Settings must be persisted locally by the software and re-transmitted on apply.

## 3. Discovered Hardware Targets

- **Vendor ID**: `0x1D57` (Beken / Xenta)
- **Product ID 2.4G Wireless**: `0xFA60`
- **Product ID Wired USB**: `0xFA61`
- **USB HID Interfaces**:
  - `MI_00`: Boot keyboard
  - `MI_01`: Standard mouse input
  - `MI_02`: Composite control (Consumer control, Usage Page `0x000A` Battery input reports, Usage Page `0x000B` Configuration feature reports)
  - `MI_03`: Extended keyboard input
