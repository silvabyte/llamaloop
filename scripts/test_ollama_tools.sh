#!/bin/bash

# Test Ollama Tool Calling Capabilities

echo "========================================="
echo "Ollama Tool Calling Test"
echo "========================================="
echo ""

# Function to test tool calling with a model
test_tool_calling() {
    local model=$1
    echo "Testing $model..."
    
    response=$(curl -s http://localhost:11434/api/chat \
    -H "Content-Type: application/json" \
    -d '{
        "model": "'$model'",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant that can use tools."
            },
            {
                "role": "user", 
                "content": "What is the weather in San Francisco?"
            }
        ],
        "tools": [
            {
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "description": "Get the current weather for a location",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "location": {
                                "type": "string",
                                "description": "The city and state"
                            }
                        },
                        "required": ["location"]
                    }
                }
            }
        ],
        "stream": false
    }' 2>/dev/null)
    
    if [ -z "$response" ]; then
        echo "  ❌ No response or error"
        return 1
    fi
    
    # Check if response contains tool_calls
    if echo "$response" | grep -q "tool_calls"; then
        echo "  ✅ Tool calling supported!"
        echo "  Response contains tool calls"
        return 0
    else
        echo "  ⚠️  Tool calling may not be supported"
        echo "  Model responded but without tool calls"
        return 1
    fi
}

# Models that typically support tool calling
echo "Recommended models with tool calling support:"
echo "---------------------------------------------"
declare -a tool_models=(
    "llama3.1:latest"
    "llama3.2:latest"
    "qwen2.5:latest"
    "mistral:latest"
    "gemma2:latest"
)

for model in "${tool_models[@]}"; do
    # Check if model is installed
    if ollama list | grep -q "$model"; then
        test_tool_calling "$model"
    else
        echo "Model $model not installed. Pull with: ollama pull $model"
    fi
    echo ""
done

echo ""
echo "========================================="
echo "How to use tool calling with Ollama:"
echo "========================================="
echo ""
echo "1. Use the /api/chat endpoint (not /api/generate)"
echo "2. Include tools in your request payload"
echo "3. Models will return tool_calls in their response"
echo ""
echo "Example Python code:"
echo "-------------------"
cat << 'PYTHON'
import ollama

response = ollama.chat(
    model='llama3.1:latest',
    messages=[{
        'role': 'user',
        'content': 'What is the weather in Toronto?'
    }],
    tools=[{
        'type': 'function',
        'function': {
            'name': 'get_weather',
            'description': 'Get the weather for a location',
            'parameters': {
                'type': 'object',
                'properties': {
                    'location': {'type': 'string'}
                },
                'required': ['location']
            }
        }
    }]
)

# Check if model made tool calls
if response.message.tool_calls:
    print("Tool calls:", response.message.tool_calls)
PYTHON

echo ""
echo "========================================="
echo "✅ Test complete!"
echo "========================================="