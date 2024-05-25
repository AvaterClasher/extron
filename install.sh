#!/bin/bash

# Define the URLs for the executables
WINDOWS_URL="https://github.com/AvaterClasher/extron/releases/download/latest/extron.exe"
LINUX_URL="https://github.com/AvaterClasher/extron/releases/download/latest/extron"

# Define the target path (typically a directory in the PATH)
TARGET_DIR=$(echo $PATH | tr ':' '\n' | grep -vE '^$' | head -n 1)

# Function to download and place the executable
download_executable() {
    local url=$1
    local path=$2

    if command -v curl &> /dev/null; then
        curl -L $url -o $path
    elif command -v wget &> /dev/null; then
        wget -O $path $url
    else
        echo "Neither curl nor wget is installed. Please install one of these tools to proceed."
        exit 1
    fi

    chmod +x $path
    echo "Downloaded and placed the executable at $path"
}

# Determine the OS and download the appropriate executable
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    EXE_PATH="$TARGET_DIR/extron"
    download_executable $LINUX_URL $EXE_PATH
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    EXE_PATH="$TARGET_DIR/extron.exe"
    download_executable $WINDOWS_URL $EXE_PATH
else
    echo "Unsupported OS."
    exit 1
fi
