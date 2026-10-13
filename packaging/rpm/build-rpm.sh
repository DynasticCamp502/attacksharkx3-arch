#!/usr/bin/env bash
set -euo pipefail

echo "==> Building Attack Shark X3 RPM package"
TOPDIR="$(pwd)/target/rpmbuild"
rm -rf "${TOPDIR}"
mkdir -p "${TOPDIR}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

VERSION="1.0.0"
TAR_NAME="attack-shark-x3-${VERSION}"

# Create source archive
git archive --format=tar.gz --prefix="${TAR_NAME}/" HEAD -o "${TOPDIR}/SOURCES/${TAR_NAME}.tar.gz" || {
    tar --exclude="./target" --exclude="./.git" -czf "${TOPDIR}/SOURCES/${TAR_NAME}.tar.gz" --transform "s,^\.,${TAR_NAME}," .
}

cp packaging/rpm/attack-shark-x3.spec "${TOPDIR}/SPECS/"

rpmbuild --define "_topdir ${TOPDIR}" -ba "${TOPDIR}/SPECS/attack-shark-x3.spec"
echo "==> RPM built at: $(find "${TOPDIR}/RPMS" -type f -name '*.rpm')"
