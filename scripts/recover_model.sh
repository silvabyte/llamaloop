#!/bin/bash

# Quick recovery script for accidentally deleted models

echo "========================================="
echo "🚑 Model Recovery Helper"
echo "========================================="
echo ""

# The model you had before
MODEL="qwen3-coder:30b"

echo "Checking if $MODEL is already installed..."
if ollama list | grep -q "$MODEL"; then
    echo "✅ Model $MODEL is already installed!"
else
    echo "❌ Model $MODEL not found. Let's recover it."
    echo ""
    echo "Pulling $MODEL (this is 18GB, will take a while)..."
    echo ""
    
    if ollama pull "$MODEL"; then
        echo ""
        echo "✅ Successfully recovered $MODEL!"
    else
        echo "❌ Failed to pull $MODEL"
        echo ""
        echo "Alternative models you can try:"
        echo "  • qwen2.5-coder:32b (newer version, similar size)"
        echo "  • qwen2.5-coder:14b (smaller, 8.9GB)"
        echo "  • qwen2.5-coder:7b (smallest, 4.7GB)"
        echo ""
        echo "Run: ollama pull qwen2.5-coder:14b"
    fi
fi

echo ""
echo "========================================="
echo "🛡️ Protection Tips:"
echo "========================================="
echo "1. In ollamamon, deletion now requires Shift+D (capital D)"
echo "2. You'll get a confirmation dialog before deletion"
echo "3. Press Y to confirm, N or ESC to cancel"
echo ""
echo "The 'd' key conflict has been fixed!"
echo "========================================="