#!/bin/bash

# Build and test custom Qwen-Coder models with enhanced capabilities

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
MODELFILES_DIR="$SCRIPT_DIR/../modelfiles"

echo "========================================="
echo "🚀 Custom Model Builder for Qwen-Coder"
echo "========================================="
echo ""

# Check if base model exists
echo "Checking for base model..."
if ! ollama list | grep -q "qwen3-coder:30b"; then
    echo "❌ Base model qwen3-coder:30b not found!"
    echo ""
    echo "Would you like to pull it now? (y/n)"
    read -r response
    if [[ "$response" == "y" ]]; then
        echo "Pulling qwen3-coder:30b (this is 18GB)..."
        ollama pull qwen3-coder:30b
    else
        echo "Please pull the base model first:"
        echo "  ollama pull qwen3-coder:30b"
        exit 1
    fi
fi

echo "✅ Base model found"
echo ""

# Build models
echo "Building custom models..."
echo ""

# 1. Tool-calling version
echo "1. Building qwen-coder-tools (with tool calling support)..."
if ollama create qwen-coder-tools -f "$MODELFILES_DIR/qwen-coder-tools.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-tools"
else
    echo "   ❌ Failed to create qwen-coder-tools"
fi

echo ""

# 2. Advanced version
echo "2. Building qwen-coder-advanced (with web search & enhanced features)..."
if ollama create qwen-coder-advanced -f "$MODELFILES_DIR/qwen-coder-advanced.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-advanced"
else
    echo "   ❌ Failed to create qwen-coder-advanced"
fi

echo ""

# 3. Function calling version
echo "3. Building qwen-coder-function (OpenAI-compatible function calling)..."
if ollama create qwen-coder-function -f "$MODELFILES_DIR/qwen-coder-function.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-function"
else
    echo "   ❌ Failed to create qwen-coder-function"
fi

echo ""

# 4. Max context version
echo "4. Building qwen-coder-maxcontext (6k context for large files)..."
if ollama create qwen-coder-maxcontext -f "$MODELFILES_DIR/qwen-coder-maxcontext.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-maxcontext"
else
    echo "   ❌ Failed to create qwen-coder-maxcontext"
fi

echo ""

# 5. Ultra context version (needs quantized model)
echo "5. Checking for ultra context version (requires quantized model)..."
if ollama list | grep -q "qwen3-coder:30b-q4_K_M"; then
    echo "   Building qwen-coder-ultra (12k context with quantization)..."
    if ollama create qwen-coder-ultra -f "$MODELFILES_DIR/qwen-coder-ultra.Modelfile"; then
        echo "   ✅ Successfully created qwen-coder-ultra"
    else
        echo "   ❌ Failed to create qwen-coder-ultra"
    fi
else
    echo "   ⚠️  Skipping qwen-coder-ultra (requires quantized model)"
    echo "   To enable: ollama pull qwen3-coder:30b-q4_K_M"
fi

echo ""
echo "========================================="
echo "📋 Available Models:"
echo "========================================="
ollama list | grep -E "qwen|NAME"

echo ""
echo "========================================="
echo "🧪 Testing Tool Calling"
echo "========================================="
echo ""

# Test tool calling
echo "Testing qwen-coder-tools with a tool call request..."
echo ""

TEST_PROMPT='Please search for the latest version of React and tell me about the new features. Use the web_search tool.'

echo "Prompt: $TEST_PROMPT"
echo ""
echo "Response:"
echo "---"

ollama run qwen-coder-tools "$TEST_PROMPT" --verbose 2>/dev/null | head -20

echo "---"
echo ""
echo "========================================="
echo "🎯 Usage Examples:"
echo "========================================="
echo ""
echo "1. Basic tool calling:"
echo '   ollama run qwen-coder-tools "Use the web_search tool to find Python 3.12 features"'
echo ""
echo "2. Advanced features:"
echo '   ollama run qwen-coder-advanced "/search latest TypeScript features"'
echo ""
echo "3. Code generation with tools:"
echo '   ollama run qwen-coder-tools "Create a Python script that fetches weather data"'
echo ""
echo "4. In your chat app:"
echo "   - Select 'qwen-coder-tools' or 'qwen-coder-advanced' as your model"
echo "   - The model will recognize tool calling patterns automatically"
echo ""
echo "========================================="
echo "✅ Custom models ready to use!"
echo "========================================="