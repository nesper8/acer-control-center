# Acer Control Center

Modern Linux control center for Acer gaming laptops based on the Clevo platform.

## Features

- Modern GTK4 / Libadwaita interface
- Dashboard
- Keyboard RGB control
- Automatic restoration of the last keyboard RGB color and brightness after reboot
- Live keyboard preview
- System D-Bus daemon
- DKMS kernel driver
- Live fan monitoring
- Automatic installation

## Supported Hardware

Currently tested on

- Acer ALG AL15G-53

Additional Clevo-based Acer laptops may work but are currently untested.

## Installation

Clone the repository

```bash
git clone https://github.com/stanlysilas/acer-control-center.git
cd acer-control-center
sudo ./install.sh
```

On Fedora, the installer places `libacercontrol` in `/usr/lib64` and configures
the GUI to link against that system library directory.

or
```bash
wget https://raw.githubusercontent.com/stanlysilas/acer-control-center/master/install.sh
chmod +x install.sh
sudo ./install.sh
```

After installation reboot the laptop.

Keyboard RGB color and brightness are saved automatically when changed and
restored by the daemon on startup. The saved settings are kept at
`/var/lib/acer-control-daemon/keyboard-rgb`.

If the application reports `Failed to open /dev/acer_io`, load the driver with:

```bash
sudo modprobe acer_io
```

If that reports that the module cannot be found, rerun the installer so DKMS can
build the driver for your current kernel.

If `modprobe acer_io` reports `Key was rejected by service`, Secure Boot is
rejecting the unsigned or untrusted kernel module. Check its status with:

```bash
sudo mokutil --sb-state
```

You can disable Secure Boot in your UEFI settings, or enroll the DKMS signing
certificate if one is available:

```bash
sudo mokutil --import /var/lib/dkms/mok.pub
```

Set an enrollment password, reboot, and confirm enrollment in the MOK Manager
screen. After booting, try `sudo modprobe acer_io` again. If the certificate
does not exist, configure DKMS module signing for your system or disable Secure
Boot; do not import a private key.

## Uninstall

```bash
sudo ./uninstall.sh
```

## Project Structure

driver/
daemon/
gui/
lib/
packaging/

## Current Status

# Implemented
- Dashboard
- Keyboard RGB
- D-Bus daemon
- Driver communication
- Live fan monitoring

# Planned
- Performance profiles
- Battery management
- Manual fan control
- Firmware updates

## License

GPL-2.0
