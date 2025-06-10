#!/bin/bash

# Function to replace in a file
replace_in_file() {
    local file=$1
    sed -i '' 's/ChainCraftNode/ChaincraftNode/g' "$file"
    sed -i '' 's/ChainCraftError/ChaincraftError/g' "$file"
}

# Replace in all test files
for file in tests/test_*.rs examples/*.rs; do
    echo "Processing $file..."
    replace_in_file "$file"
done

echo "Done!" 