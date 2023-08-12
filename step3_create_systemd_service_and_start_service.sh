#!/bin/bash

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root or use sudo."
  exit 1
fi

# Check if user argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <username>"
  exit 1
fi

# Check if user exists
if ! id -u "$1" > /dev/null 2>&1; then
  echo "User $1 does not exist."
  exit 1
fi

# Path to the systemd service file
SERVICE_FILE="/etc/systemd/system/lidshutsleep@.service"

# Write the service file
cat <<EOL > "$SERVICE_FILE"
[Unit]
Description=Lid Shut Sleep Service for %i
After=display-manager.service

[Service]
ExecStart=/usr/local/bin/lidshutsleep
User=%i
Restart=always

[Install]
WantedBy=multi-user.target
EOL

# Reload systemd to recognize the new service
systemctl daemon-reload

# Enable the service for the specified user
systemctl enable "lidshutsleep@$1"

# Start the service for the specified user
systemctl start "lidshutsleep@$1"

echo "Service lidshutsleep@$1 installed and started successfully."
