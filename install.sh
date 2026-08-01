#!/bin/bash

set -e

if [ "$EUID" -ne 0 ]; then
    echo "This script must be run as root."
    echo "Usage: sudo ./install.sh"
    exit 1
fi

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
DRIVER_DIR="$REPO_DIR/driver"
LIB_DIR="$REPO_DIR/lib"

install_packages() {
    if [ -f /etc/arch-release ]; then
        echo "Detected Arch Linux"

        pacman -Sy --noconfirm

        pacman -S --needed --noconfirm \
            git \
            gcc \
            make \
            dkms \
            meson \
            ninja \
            pkgconf \
            base-devel

        mapfile -t headers < <(
            pacman -Q | awk '/^linux([0-9.-]*)? /{print $1"-headers"}'
        )

        if [ ${#headers[@]} -gt 0 ]; then
            pacman -S --needed --noconfirm "${headers[@]}"
        fi

    elif [ -f /etc/debian_version ]; then
        echo "Detected Debian/Ubuntu"

        apt update

        apt install -y \
            git \
            gcc \
            make \
            dkms \
            meson \
            ninja-build \
            pkg-config \
            build-essential \
            linux-headers-$(uname -r)

    elif [ -f /etc/redhat-release ]; then
        echo "Detected Fedora/RHEL"

        dnf install -y \
            git \
            gcc \
            make \
            dkms \
            meson \
            ninja-build \
            pkgconf-pkg-config \
            kernel-devel \
            kernel-headers

    elif command -v zypper >/dev/null 2>&1; then
        echo "Detected openSUSE"

        zypper install -y \
            git \
            gcc \
            make \
            dkms \
            meson \
            ninja \
            pkg-config \
            kernel-devel \
            kernel-headers

    else
        echo "Unsupported Linux distribution."
        exit 1
    fi
}

install_packages

echo
echo "Cleaning previous driver build..."
make -C "$DRIVER_DIR" clean

echo
echo "Installing DKMS driver..."
make -C "$DRIVER_DIR" dkmsinstall

echo
echo "Loading kernel modules..."
modprobe acer_keyboard || true
modprobe acer_io || true

echo
echo "Ensuring modules load at boot..."

touch /etc/modules

grep -qxF "acer_keyboard" /etc/modules || \
    echo "acer_keyboard" >> /etc/modules

grep -qxF "acer_io" /etc/modules || \
    echo "acer_io" >> /etc/modules

echo
echo "Installing libacercontrol..."

if [ ! -d "$LIB_DIR/build" ]; then
    meson setup "$LIB_DIR/build" "$LIB_DIR"
fi

meson compile -C "$LIB_DIR/build"
meson install -C "$LIB_DIR/build"

echo "/usr/local/lib64" >/etc/ld.so.conf.d/acercontrol.conf
ldconfig

echo
echo "Installing udev rule..."

getent group acercontrol >/dev/null || \
    groupadd --system acercontrol

if [ -n "$SUDO_USER" ]; then
    usermod -aG acercontrol "$SUDO_USER"
fi

install -Dm644 \
    "$REPO_DIR/packaging/udev/99-acercontrol.rules" \
    /etc/udev/rules.d/99-acercontrol.rules

udevadm control --reload-rules
udevadm trigger --name-match=acer_io || true

echo
echo "Installed modules:"
lsmod | grep acer || true

echo
echo "Device nodes:"
ls -l /dev/acer* 2>/dev/null || true

echo
echo "Shared library:"
ldconfig -p | grep acercontrol || true

echo
echo "=================================================="
echo " Acer Control Center installed successfully."
echo "=================================================="
echo
echo "If this is the first installation, log out and"
echo "log back in so your new group membership takes"
echo "effect."
echo
