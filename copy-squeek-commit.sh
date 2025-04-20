#!/bin/bash

SOURCE_PATH="./target/release/squeek-commit"
DEST_PATH="/usr/local/bin/squeek-commit"

if [ -f "$DEST_PATH" ]; then
    echo "⚠️ Bestand bestaat al, verwijderen: $DEST_PATH"
    sudo rm "$DEST_PATH"
fi

echo "📦 Kopiëren van $SOURCE_PATH naar $DEST_PATH..."
sudo cp "$SOURCE_PATH" "$DEST_PATH"

sudo chmod +x "$DEST_PATH"

echo "✅ squeek-commit is succesvol gekopieerd naar /usr/local/bin/"
