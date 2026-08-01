#!/bin/bash
set -euo pipefail

if [ "${EUID}" -ne 0 ]; then
    echo "This script must be run as root."
    echo "Usage: sudo ./install.sh"
    exit 1
fi

install_packages() {
    if [ -f /etc/arch-release ]; then
        echo "Detected Arch Linux"
        pacman -Sy --noconfirm
        pacman -S --needed --noconfirm \
            git \
            gcc \
            make \
            dkms \
            base-devel \
            meson \
            ninja \
            rust \
            cargo \
            pkgconf
    elif [ -f /etc/debian_version ]; then
        echo "Detected Debian/Ubuntu"
        apt update
        apt install -y \
            git \
            gcc \
            make \
            dkms \
            build-essential \
            meson \
            ninja-build \
            cargo \
            rustc \
            pkg-config
    elif [ -f /etc/redhat-release ]; then
        echo "Detected Fedora/RHEL"
        dnf install -y \
            git \
            gcc \
            gcc-c++ \
            make \
            dkms \
            kernel-devel \
            kernel-headers \
            meson \
            ninja-build \
            cargo \
            rust \
            pkgconf-pkg-config
    elif command -v zypper >/dev/null 2>&1; then
        echo "Detected openSUSE"
        zypper install -y \
            git \
            gcc \
            gcc-c++ \
            make \
            dkms \
            kernel-devel \
            kernel-headers \
            meson \
            ninja \
            cargo \
            rust \
            pkg-config
    else
        echo "Unsupported Linux distribution."
        exit 1
    fi
}

install_packages

echo "Cleaning previous driver build..."
make -C driver clean || true

echo "Installing DKMS module..."
make -C driver dkmsinstall

echo "Building libacercontrol..."
if [ -d lib/build ]; then
    meson setup lib/build lib --reconfigure
else
    meson setup lib/build lib
fi
meson compile -C lib/build

echo "Installing libacercontrol..."
meson install -C lib/build

echo "Building daemon..."
cargo build --release --manifest-path daemon/Cargo.toml

echo "Installing daemon binary..."
install -Dm755 \
    daemon/target/release/acer-control-daemon \
    /usr/bin/acer-control-daemon

echo "Installing systemd service..."
install -Dm644 \
    packaging/systemd/acer-control-daemon.service \
    /usr/lib/systemd/system/acer-control-daemon.service

echo "Installing D-Bus policy..."
install -Dm644 \
    packaging/dbus/io.github.stanlysilas.AcerControl.conf \
    /usr/share/dbus-1/system.d/io.github.stanlysilas.AcerControl.conf

echo "Reloading D-Bus config..."
busctl call \
    --system \
    org.freedesktop.DBus \
    /org/freedesktop/DBus \
    org.freedesktop.DBus \
    ReloadConfig >/dev/null || true

echo "Reloading systemd..."
systemctl daemon-reload

echo "Enabling daemon..."
systemctl enable acer-control-daemon.service

echo "Starting daemon..."
systemctl restart acer-control-daemon.service

echo "Installation complete."
