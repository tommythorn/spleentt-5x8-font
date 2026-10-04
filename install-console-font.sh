#!/usr/bin/env bash
set -e

# 1. Copy the console font (ensure it matches the actual filename inside the repo)
sudo cp ./SpleenttMedium.psf /usr/share/consolefonts/
sudo gzip -f /usr/share/consolefonts/SpleenttMedium.psf

# 2. Inject configuration safely
sudo tee /etc/default/console-setup > /dev/null << 'EOF'
ACTIVE_CONSOLES="/dev/tty[1-6]"
CHARMAP="UTF-8"
CODESET="guess"
FONTFACE=""
FONTSIZE=""
FONT="SpleenttMedium.psf.gz"
EOF

# 3. Apply changes and compile into early boot
sudo setupcon
sudo update-initramfs -u
echo "SpleenttMedium successfully set for early boot framebuffer!"
