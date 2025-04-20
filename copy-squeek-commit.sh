#!/bin/bash

SOURCE_PATH="./target/release/squeek-commit"
DEST_PATH="/usr/local/bin/squeek-commit"

if [ -f "$DEST_PATH" ]; then
    echo "⚠️ File already exists, removing: $DEST_PATH"
    sudo rm "$DEST_PATH"
fi

echo "📦 Copying from $SOURCE_PATH to $DEST_PATH..."
sudo cp "$SOURCE_PATH" "$DEST_PATH"

sudo chmod +x "$DEST_PATH"

echo "✅ squeek-commit has been successfully copied to /usr/local/bin/"