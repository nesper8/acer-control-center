# Acer Control Center

Modern Linux control center for Acer gaming laptops based on the Clevo platform.

## Features

- Modern GTK4 / Libadwaita interface
- Dashboard
- Keyboard RGB control
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

or
```bash
wget https://raw.githubusercontent.com/stanlysilas/acer-control-center/master/install.sh
chmod +x install.sh
sudo ./install.sh
```

After installation reboot the laptop.

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
