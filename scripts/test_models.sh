#!/bin/bash

# Test all custom models with specific prompts

echo "========================================="
echo "🧪 Testing Custom Qwen-Coder Models"
echo "========================================="
echo ""

# Test 1: Tool calling
echo "1. Testing qwen-coder-tools (Tool Calling)"
echo "-------------------------------------------"
echo '{
  "function_call": {
    "name": "search",
    "arguments": {
      "query": "latest React features"
    }
  }
}' | ollama run qwen-coder-tools "You just received this function call. Process it and explain what you would return." 2>/dev/null | head -10

echo ""
echo "2. Testing qwen-coder-function (OpenAI Format)"
echo "-----------------------------------------------"
ollama run qwen-coder-function "Call the calculate function to compute fibonacci(10)" 2>/dev/null | head -15

echo ""
echo "3. Testing qwen-coder-maxcontext (Large Context)"
echo "-------------------------------------------------"
ollama run qwen-coder-maxcontext "You have 6k context. Acknowledge this and explain how you'd use it for analyzing a large codebase." 2>/dev/null | head -10

echo ""
echo "4. Quick Model Comparison"
echo "--------------------------"
echo ""
echo "Model Sizes:"
ollama list | grep qwen | awk '{print $1 "\t" $3 " " $4}'

echo ""
echo "========================================="
echo "✅ Test complete!"
echo ""
echo "To use in ollamamon:"
echo "1. Launch: ./target/release/ollamamon"
echo "2. Go to Chat (press 4)"
echo "3. Press 'm' to select model"
echo "4. Choose your custom model"
echo "========================================="