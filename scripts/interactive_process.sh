#!/bin/bash

# ================= Configuration =================
# Path to the binary executable.
# Ensure this points to your compiled binary.
TOOL_PATH="../target/release/angrybirds-cryptor-cli"

# ===============================================

# Check if the tool exists
if [ ! -f "$TOOL_PATH" ]; then
    echo "Error: Binary not found at: $TOOL_PATH"
    echo "Please run 'cargo build --release' or update the TOOL_PATH variable in the script."
    exit 1
fi

echo "========================================"
echo "   Angry Birds Cryptor Batch Tool"
echo "========================================"

# --- 1. Get Input Directory ---
while true; do
    # -p displays the prompt, -e allows editing (like backspace)
    read -e -p "Please enter the [Source] directory path: " INPUT_DIR
    
    # Remove quotes that might be added when dragging and dropping folders
    INPUT_DIR=$(echo "$INPUT_DIR" | tr -d '"' | tr -d "'")

    if [ -d "$INPUT_DIR" ]; then
        break
    else
        echo "❌ Error: Directory '$INPUT_DIR' does not exist. Please try again."
    fi
done

# --- 2. Get Output Directory ---
read -e -p "Please enter the [Output] directory path (Default: ./output): " OUTPUT_DIR
OUTPUT_DIR=$(echo "$OUTPUT_DIR" | tr -d '"' | tr -d "'")

# If output directory is empty, default to ./output
if [ -z "$OUTPUT_DIR" ]; then
    OUTPUT_DIR="./output"
fi
# Create output directory
mkdir -p "$OUTPUT_DIR"


# --- 3. Select Mode ---
echo "----------------------------------------"
echo "Select Operation Mode:"
echo " 1) Batch Decrypt (Recommended: Auto-detect Key)"
echo " 2) Batch Encrypt"
read -p "Enter choice [1 or 2]: " MODE_CHOICE

if [ "$MODE_CHOICE" == "2" ]; then
    MODE="encrypt"
    echo "--- Encryption Settings ---"
    read -p "Enter Game Name (e.g., classic, seasons, rio): " GAME_NAME
    read -p "Enter File Category (e.g., native, save): " CATEGORY
    
    if [ -z "$GAME_NAME" ] || [ -z "$CATEGORY" ]; then
        echo "❌ Error: Encryption mode requires both Game Name and Category!"
        exit 1
    fi
else
    MODE="decrypt"
fi

# ================= Start Processing =================
echo "========================================"
echo "Starting Process..."
echo "Input: $INPUT_DIR"
echo "Output: $OUTPUT_DIR"
echo "Mode: $MODE"
if [ "$MODE" == "encrypt" ]; then
    echo "Params: Game=$GAME_NAME, Category=$CATEGORY"
fi
echo "========================================"

count=0
success=0
fail=0

# Enable nullglob to prevent loops from running if no files exist
shopt -s nullglob

for file in "$INPUT_DIR"/*; do
    # Ensure it is a file and not a subdirectory
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        output_file="$OUTPUT_DIR/$filename"
        
        echo -n "Processing: $filename ... "

        if [ "$MODE" == "decrypt" ]; then
            # Decrypt mode: Use --auto
            "$TOOL_PATH" decrypt \
                --input "$file" \
                --output "$output_file" \
                --auto \
                > /dev/null 2>&1
        else
            # Encrypt mode
            "$TOOL_PATH" encrypt \
                --input "$file" \
                --output "$output_file" \
                --game "$GAME_NAME" \
                --category "$CATEGORY" \
                > /dev/null 2>&1
        fi

        # Check exit code ($?)
        if [ $? -eq 0 ]; then
            echo "✅ Success"
            ((success++))
        else
            echo "❌ Failed"
            ((fail++))
        fi
        ((count++))
    fi
done

# Restore default shell options
shopt -u nullglob

echo "========================================"
echo "Processing Complete!"
echo "Total: $count, Success: $success, Failed: $fail"
echo "Files saved to: $OUTPUT_DIR"