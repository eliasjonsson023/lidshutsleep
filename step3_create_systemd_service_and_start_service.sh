#!/bin/bash

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root or use sudo."
  exit 1
fi

# Path to the systemd service file
SERVICE_FILE="/etc/systemd/system/lidshutsleep.service"

# Write the service file
cat <<EOL > "$SERVICE_FILE"
[Unit]
Description=Lid Shut Sleep Service
After=display-manager.service

[Service]
ExecStart=/usr/local/bin/lidshutsleep
User=root
Restart=always

[Install]
WantedBy=multi-user.target
EOL

# Reload systemd to recognize the new service
systemctl daemon-reload

# Enable the service for the specified user
systemctl enable "lidshutsleep"

# Start the service for the specified user
systemctl start "lidshutsleep"

echo "Service lidshutsleep installed and started successfully."
echo "A reboot might be needded if lidshutsleep doesn't work yet."
