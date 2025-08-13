# 🔄 llamaloop Custom Models

## Your Enhanced Model Arsenal

### Successfully Built Models:

| Model | Context | Memory | Purpose |
|-------|---------|--------|---------|
| **qwen-coder-ultra** | 12,288 tokens | ~25GB | Maximum context using your Q4 model |
| **qwen-coder-tools** | 8,192 tokens | ~25GB | Tool calling & JSON output |
| **qwen-coder-function** | 8,192 tokens | ~25GB | OpenAI-compatible functions |
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

```bash
# Test ultra context (12k tokens!)
ollama run qwen-coder-ultra "You have 12k context. Analyze this entire codebase..."

# Test tool calling
ollama run qwen-coder-tools "Create a function to search and return JSON"

# Test function calling
ollama run qwen-coder-function "Call the weather API for San Francisco"
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