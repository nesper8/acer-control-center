#!/bin/bash

set -e

if [ "$EUID" -ne 0 ]; then
    echo "This script requires root rights. Execute it with sudo."
    exit 1
fi

echo "==> Uninstalling Acer Control Center driver..."

#
# Unload modules (reverse dependency order)
#
echo "==> Unloading kernel modules..."

for module in tuxedo_keyboard tuxedo_io acer_wmi acer_acpi; do
    if lsmod | grep -q "^${module}"; then
        echo "Removing ${module}..."
        rmmod "${module}" || true
    fi
done

#
# Remove DKMS package
#
if command -v dkms >/dev/null 2>&1; then
    echo "==> Removing DKMS package..."

    dkms status | while IFS= read -r line; do
        if echo "$line" | grep -qi "tuxedo"; then
            PACKAGE=$(echo "$line" | cut -d',' -f1 | cut -d'/' -f1)
            VERSION=$(echo "$line" | cut -d',' -f1 | cut -d'/' -f2)

            echo "Removing ${PACKAGE}/${VERSION}..."
            dkms remove "${PACKAGE}/${VERSION}" --all || true
        fi
    done
fi

#
# Remove modprobe configuration
#
echo "==> Removing modprobe configuration..."

rm -f /etc/modprobe.d/tuxedo_keyboard.conf

#
# Remove automatic module loading
#
echo "==> Cleaning /etc/modules..."

if [ -f /etc/modules ]; then
    sed -i '/^tuxedo_keyboard$/d' /etc/modules
fi

#
# Remove leftover DKMS source
#
echo "==> Removing DKMS source..."

rm -rf /usr/src/tuxedo-keyboard-*

#
# Remove leftover modules
#
echo "==> Removing installed kernel modules..."

find /lib/modules -type f \( \
    -name "tuxedo_keyboard.ko*" -o \
    -name "tuxedo_io.ko*" -o \
    -name "acer_wmi.ko*" -o \
    -name "acer_acpi.ko*" \
\) -exec rm -f {} \;

#
# Rebuild module dependency database
#
echo "==> Running depmod..."

depmod -a

rm -f /etc/udev/rules.d/99-acercontrol.rules

udevadm control --reload-rules
udevadm trigger

echo
echo "==============================================="
echo " Acer Control Center driver has been removed."
echo "==============================================="
echo
echo "A reboot is recommended."
