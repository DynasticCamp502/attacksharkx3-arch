#!/usr/bin/env bash
set -euo pipefail

echo "==> Building Attack Shark X3 AppDir for AppImage"
cargo build --release

APPDIR="target/AppDir"
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"

cp target/release/attack-shark-x3 "${APPDIR}/usr/bin/"
cp packaging/desktop/attack-shark-x3.desktop "${APPDIR}/"
cp packaging/desktop/attack-shark-x3.desktop "${APPDIR}/usr/share/applications/"
cp assets/icons/icon.png "${APPDIR}/attack-shark-x3.png"
cp assets/icons/icon.png "${APPDIR}/usr/share/icons/hicolor/256x256/apps/attack-shark-x3.png"

cat <<'EOF' > "${APPDIR}/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/attack-shark-x3" "$@"
EOF
chmod +x "${APPDIR}/AppRun"

echo "==> AppDir created at ${APPDIR}. Run appimagetool to create .AppImage."
