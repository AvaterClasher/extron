#!/bin/bash

# Define the URLs for the executables
LINUX_URL="https://github.com/AvaterClasher/extron/releases/download/latest/extron"

# Define the target path (typically a directory in the PATH)
TARGET_DIR=$(echo $PATH | tr ':' '\n' | grep -vE '^$' | head -n 1)

# Determine the OS and download the appropriate executable
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    EXE_PATH="$TARGET_DIR/extron"
    curl -L $LINUX_URL -o $EXE_PATH
    chmod +x $EXE_PATH
    echo "Downloaded and placed the executable at $EXE_PATH"
else
    echo "Unsupported OS."
fi