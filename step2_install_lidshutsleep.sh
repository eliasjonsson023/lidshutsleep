#!/bin/bash

# Check if the binary exists
if [ ! -f "./target/release/lidshutsleep" ]; then
  echo "Binary lidshutsleep not found."
  exit 1
fi

# Copy the binary to /usr/local/bin
cp "./target/release/lidshutsleep" "/usr/local/bin/lidshutsleep"
if [ $? -ne 0 ]; then
  echo "Failed to copy the binary to /usr/local/bin. Are you running with the right permissions?"
  exit 1
fi

# Make sure it's executable
chmod +x "/usr/local/bin/lidshutsleep"

echo "Binary installed successfully to /usr/local/bin/lidshutsleep."
