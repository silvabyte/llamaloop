#!/bin/bash

# Ollama Model Migration Script
# Migrates models from LM Studio to Ollama

echo "========================================="
echo "LM Studio to Ollama Migration Script"
echo "========================================="
echo ""

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "❌ Ollama is not running. Please start Ollama first."
    echo "   Run: ollama serve"
    exit 1
fi

echo "✅ Ollama is running"
echo ""

# Array of models to pull
# Based on common LM Studio models, these are the Ollama equivalents
declare -a models=(
    "qwen2.5-coder:32b-instruct"     # Coder model with instruct support
    "qwen2.5:32b"                     # General purpose with tool support
    "llama3.1:8b"                     # Supports tool calling
    "mistral:7b"                      # Lightweight with function calling
    "qwen2.5:14b"                     # Mid-size with tool support
    "nomic-embed-text"                # For embeddings
    "gemma2:9b"                       # Google's efficient model
)

echo "📦 Models to be pulled:"
for model in "${models[@]}"; do
    echo "   - $model"
done
echo ""

# Pull each model
for model in "${models[@]}"; do
    echo "----------------------------------------"
    echo "🔄 Pulling model: $model"
    echo "   This may take a while depending on model size..."
    
    if ollama pull "$model"; then
        echo "✅ Successfully pulled: $model"
    else
        echo "❌ Failed to pull: $model"
        echo "   You can try pulling it manually later with: ollama pull $model"
    fi
    echo ""
done

echo "========================================="
echo "📋 Checking installed models..."
echo "========================================="
ollama list

echo ""
echo "========================================="
echo "🌐 Network Configuration"
echo "========================================="
echo ""
echo "To enable network access to Ollama:"
echo ""
echo "1. Set environment variable before starting Ollama:"
echo "   export OLLAMA_HOST=0.0.0.0"
echo "   ollama serve"
echo ""
echo "2. Or create a systemd service (Linux) or launchd plist (macOS)"
echo ""
echo "3. For macOS, you can create ~/Library/LaunchAgents/com.ollama.server.plist"
echo "   with OLLAMA_HOST set to 0.0.0.0"
echo ""
echo "4. Test from another device:"
echo "   curl http://YOUR_IP:11434/api/tags"
echo ""
echo "========================================="
echo "✅ Migration script complete!"
echo "========================================="