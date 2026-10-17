#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RULES_SRC="${SCRIPT_DIR}/../packaging/udev/99-attack-shark-x3.rules"
RULES_DEST="/etc/udev/rules.d/99-attack-shark-x3.rules"

echo "==> Installing udev rules for Attack Shark X3..."
sudo cp "${RULES_SRC}" "${RULES_DEST}"
echo "==> Reloading udev rules..."
sudo udevadm control --reload-rules
sudo udevadm trigger
echo "==> Done! You can now configure Attack Shark X3 without root privileges."
