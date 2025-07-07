#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Directory containing your PNG images (relative to script location)
PNG_DIR="$SCRIPT_DIR/../assets/visual_effects"

# Output directory for KTX2 files (relative to script location)
KTX2_DIR="$SCRIPT_DIR/../assets/visual_effects_ktx2"

# Create the output directory if it doesn't exist
mkdir -p "$KTX2_DIR"

# Loop through all PNG files in the PNG directory
for png in "$PNG_DIR"/*.png; do
    # Use basename to get the file name without the directory
    base_name=$(basename "$png" .png)
    
    # Define the output file path
    ktx2="$KTX2_DIR/${base_name}.ktx2"
    
    # Convert PNG to KTX2 using ktxsc
    # toktx --target_type RGBA --2d --t2 --encode astc --astc_blk_d 4x4 --clevel 5 --qlevel 255 "$ktx2" "$png"
    # Working:
    toktx --t2 --2d --encode uastc --zcmp 10 "$ktx2" "$png"
    
    echo "Converted $png to $ktx2"
done

echo "All PNG images have been converted to KTX2 format."