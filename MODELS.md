# 🔄 llamaloop Custom Models

## Your Enhanced Model Arsenal

### Successfully Built Models:

| Model | Context | Memory | Purpose |
|-------|---------|--------|---------|
| **qwen-coder-ultra** | 12,288 tokens | ~25GB | Maximum context using your Q4 model |
| **qwen-coder-tools-v2** ✅ | 8,192 tokens | ~25GB | Working tool simulation for Ollama |
| **qwen-coder-function-v2** ✅ | 8,192 tokens | ~25GB | Working function calls for Ollama |
| **qwen-coder-tools** | 8,192 tokens | ~25GB | Tool calling (original) |
| **qwen-coder-function** | 8,192 tokens | ~25GB | Function calling (original) |
| **qwen-coder-maxcontext** | 6,144 tokens | ~30GB | Stable large context |
| **qwen3-coder:30b** | 4,096 tokens | ~20GB | Base model (fastest) |
| **qwen3-coder:30b-a3b-q4_K_M** | 4,096 tokens | ~10GB | Your quantized base |

## Quick Usage in llamaloop:

```bash
# Launch llamaloop
./target/release/llamaloop

# In Chat (press 4):
# - Press 'm' to select model
# - Choose qwen-coder-ultra for maximum context
# - Choose qwen-coder-tools for function calling
```

## Testing Your Models:

### V2 Models (Working with Ollama!) ✅

```bash
# Test working tool simulation
ollama run qwen-coder-tools-v2 "Use the web_search tool to find Python 3.12 features"
# Output will include:
# ```tool:web_search
# query: Python 3.12 new features latest updates
# ```
# [Followed by simulated results]

# Test working function calls
ollama run qwen-coder-function-v2 "What's the weather in Tokyo?"
# Output will include:
# ### FUNCTION_CALL_START
# function_name: get_weather
# parameters:
#   location: Tokyo, Japan
# ### FUNCTION_CALL_END
# [Followed by simulated weather data]

# Test ultra context (12k tokens!)
ollama run qwen-coder-ultra "Analyze this entire codebase..."
```

### Original Models (JSON format)

```bash
# These output JSON but may not generate actual tool calls
ollama run qwen-coder-tools "Create a function to search"
ollama run qwen-coder-function "Call the weather API"
```

## Model Selection Guide:

```
┌─────────────────────────────────┐
│ What do you need?               │
└─────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────┐
│ Maximum Context?                │
│ → qwen-coder-ultra (12k)        │
└─────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────┐
│ Tool/Function Calling?          │
│ → qwen-coder-tools              │
│ → qwen-coder-function           │
└─────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────┐
│ Stable Large Files?             │
│ → qwen-coder-maxcontext (6k)    │
└─────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────┐
│ Fast General Coding?            │
│ → qwen3-coder:30b               │
└─────────────────────────────────┘
```

## The llamaloop Advantage:

With these custom models in llamaloop, you can:
- **Handle entire codebases** with qwen-coder-ultra (12k context!)
- **Build agents** with qwen-coder-tools
- **Integrate with frameworks** using qwen-coder-function
- **Analyze large files** with qwen-coder-maxcontext
- **Switch models on the fly** in the chat interface

## Pro Tips:

1. **For maximum speed**: Use the base quantized model
2. **For maximum context**: Use qwen-coder-ultra
3. **For tool calling**: Use qwen-coder-function with LangChain
4. **For stability**: Use qwen-coder-maxcontext

Your llamaloop is now supercharged with custom models! 🚀