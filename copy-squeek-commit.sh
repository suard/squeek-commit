#!/bin/bash

mkdir -p "$HOME/bin"

cp ./target/release/squeek-commit "$HOME/bin/"

chmod +x "$HOME/bin/squeek-commit"

echo "✅ squeek-commit has been copied to ~/bin"