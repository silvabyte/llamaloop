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

# Build V2 models (working tool simulation for Ollama)
echo "1. Building qwen-coder-tools-v2 (working tool simulation)..."
if ollama create qwen-coder-tools-v2 -f "$MODELFILES_DIR/qwen-coder-tools-v2.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-tools-v2"
else
    echo "   ❌ Failed to create qwen-coder-tools-v2"
fi

echo ""

echo "2. Building qwen-coder-function-v2 (working function calls)..."
if ollama create qwen-coder-function-v2 -f "$MODELFILES_DIR/qwen-coder-function-v2.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-function-v2"
else
    echo "   ❌ Failed to create qwen-coder-function-v2"
fi

echo ""

# Original models (for compatibility)
echo "3. Building qwen-coder-tools (original)..."
if ollama create qwen-coder-tools -f "$MODELFILES_DIR/qwen-coder-tools.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-tools"
else
    echo "   ❌ Failed to create qwen-coder-tools"
fi

echo ""

echo "4. Building qwen-coder-function (original)..."
if ollama create qwen-coder-function -f "$MODELFILES_DIR/qwen-coder-function.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-function"
else
    echo "   ❌ Failed to create qwen-coder-function"
fi

echo ""

# Advanced version
echo "5. Building qwen-coder-advanced (with web search & enhanced features)..."
if [ -f "$MODELFILES_DIR/qwen-coder-advanced.Modelfile" ]; then
    if ollama create qwen-coder-advanced -f "$MODELFILES_DIR/qwen-coder-advanced.Modelfile"; then
        echo "   ✅ Successfully created qwen-coder-advanced"
    else
        echo "   ❌ Failed to create qwen-coder-advanced"
    fi
else
    echo "   ⚠️  Skipping qwen-coder-advanced (modelfile not found)"
fi

echo ""

# Max context version
echo "6. Building qwen-coder-maxcontext (6k context for large files)..."
if ollama create qwen-coder-maxcontext -f "$MODELFILES_DIR/qwen-coder-maxcontext.Modelfile"; then
    echo "   ✅ Successfully created qwen-coder-maxcontext"
else
    echo "   ❌ Failed to create qwen-coder-maxcontext"
fi

echo ""

# Ultra context version (needs quantized model)
echo "7. Checking for ultra context version (requires quantized model)..."
if ollama list | grep -q "qwen3-coder:30b-a3b-q4_K_M"; then
    echo "   Building qwen-coder-ultra (12k context with quantization)..."
    if ollama create qwen-coder-ultra -f "$MODELFILES_DIR/qwen-coder-ultra.Modelfile"; then
        echo "   ✅ Successfully created qwen-coder-ultra"
    else
        echo "   ❌ Failed to create qwen-coder-ultra"
    fi
else
    echo "   ⚠️  Skipping qwen-coder-ultra (requires quantized model)"
    echo "   To enable: ollama pull qwen3-coder:30b-a3b-q4_K_M"
fi

echo ""
echo "========================================="
echo "📋 Available Models:"
echo "========================================="
ollama list | grep -E "qwen|NAME"

echo ""
echo "========================================="
echo "🧪 Testing Tool Calling (V2 Models)"
echo "========================================="
echo ""

# Test V2 tool calling
echo "Testing qwen-coder-tools-v2 with a tool call request..."
echo ""

TEST_PROMPT='Use the web_search tool to find information about Python 3.12 features'

echo "Prompt: $TEST_PROMPT"
echo ""
echo "Response from qwen-coder-tools-v2:"
echo "---"

ollama run qwen-coder-tools-v2 "$TEST_PROMPT" 2>/dev/null | head -30

echo "---"
echo ""

echo "Testing qwen-coder-function-v2 with a function call request..."
echo ""

FUNCTION_TEST='What is the weather in San Francisco? Use the get_weather function.'

echo "Prompt: $FUNCTION_TEST"
echo ""
echo "Response from qwen-coder-function-v2:"
echo "---"

ollama run qwen-coder-function-v2 "$FUNCTION_TEST" 2>/dev/null | head -30

echo "---"
echo ""
echo "========================================="
echo "🎯 Usage Examples:"
echo "========================================="
echo ""
echo "1. Working tool simulation (V2):"
echo '   ollama run qwen-coder-tools-v2 "Search for React 19 features"'
echo ""
echo "2. Working function calls (V2):"
echo '   ollama run qwen-coder-function-v2 "Get the weather in Tokyo"'
echo ""
echo "3. Maximum context (12k tokens):"
echo '   ollama run qwen-coder-ultra "Analyze this large codebase..."'
echo ""
echo "4. In llamaloop chat:"
echo "   - Press 'm' to select model"
echo "   - Choose 'qwen-coder-tools-v2' for tool simulation"
echo "   - Choose 'qwen-coder-function-v2' for function calls"
echo ""
echo "========================================="
echo "✅ Custom models ready to use!"
echo "========================================="