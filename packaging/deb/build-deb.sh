#!/usr/bin/env bash
set -euo pipefail

echo "==> Building Attack Shark X3 .deb package"
VERSION="1.0.0"
ARCH="amd64"
PKG_DIR="target/deb/attack-shark-x3_${VERSION}_${ARCH}"

cargo build --release

rm -rf "${PKG_DIR}"
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/bin"
mkdir -p "${PKG_DIR}/usr/lib/udev/rules.d"
mkdir -p "${PKG_DIR}/usr/share/applications"
mkdir -p "${PKG_DIR}/usr/share/icons/hicolor/256x256/apps"

install -m 0755 target/release/attack-shark-x3 "${PKG_DIR}/usr/bin/"
install -m 0644 packaging/udev/99-attack-shark-x3.rules "${PKG_DIR}/usr/lib/udev/rules.d/"
install -m 0644 packaging/desktop/attack-shark-x3.desktop "${PKG_DIR}/usr/share/applications/"
install -m 0644 assets/icons/icon.png "${PKG_DIR}/usr/share/icons/hicolor/256x256/apps/attack-shark-x3.png"

cat <<EOF > "${PKG_DIR}/DEBIAN/control"
Package: attack-shark-x3
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libhidapi-hidraw0 (>= 0.12.0)
Maintainer: Enthony <contact@example.com>
Description: Native Linux configuration driver and GUI for Attack Shark X3 mouse
 Graphical utility for configuring DPI stages, polling rate, debounce time,
 sleep timers, button remapping, and macros on Attack Shark X3.
EOF

dpkg-deb --build "${PKG_DIR}" "target/deb/attack-shark-x3_${VERSION}_${ARCH}.deb" || {
    echo "dpkg-deb not installed, package directory staged at ${PKG_DIR}"
}
echo "==> Done staged at target/deb/"
