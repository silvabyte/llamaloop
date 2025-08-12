# Custom Ollama Models for Enhanced Capabilities

This directory contains custom Modelfiles that enhance base models with tool calling, web search simulation, extended context, and other advanced features.

## 🎯 Model Selection Guide

### Quick Decision Tree:
```
Need tool/function calling? 
  ├─ Simple/LangChain → qwen-coder-function
  └─ Complex/Custom → qwen-coder-tools

Need large context?
  ├─ Quality priority (6k) → qwen-coder-maxcontext  
  └─ Context priority (12k) → qwen-coder-ultra

Need everything?
  → qwen-coder-advanced (kitchen sink)

Just need fast coding?
  → qwen3-coder:30b (base model)
```

## 📦 Available Models

### 1. qwen-coder-tools
**Base**: qwen3-coder:30b  
**Context**: 8,192 tokens  
**Memory**: ~25GB  
**Best For**: 
- API integration projects
- Automated tool usage
- Structured data generation
- CI/CD pipeline scripts

**When to Use**:
- Building agents that need to call external tools
- Creating structured JSON/YAML outputs
- Integrating with existing tool frameworks
- Need reliable function calling

**Example**:
```bash
ollama run qwen-coder-tools "Create a Python script that uses the GitHub API to analyze repository statistics"
```

### 2. qwen-coder-advanced (Kitchen Sink 🍳)
**Base**: qwen3-coder:30b  
**Context**: 16,384 tokens  
**Memory**: ~35GB  
**Best For**:
- System architecture design
- Complex multi-file refactoring
- Technical documentation with diagrams
- Research and exploration tasks

**When to Use**:
- Need Mermaid diagrams for documentation
- Complex problem requiring multiple approaches
- Want web search simulation
- Building comprehensive technical specs

**Special Features**:
- `/search` command for web simulation
- Mermaid diagram generation
- Multi-step reasoning
- Advanced code analysis

**Example**:
```bash
ollama run qwen-coder-advanced "Design a microservices architecture for an e-commerce platform with diagrams"
```

### 3. qwen-coder-function
**Base**: qwen3-coder:30b  
**Context**: 8,192 tokens  
**Memory**: ~25GB  
**Best For**:
- LangChain integration
- AutoGPT compatibility
- Production applications
- Simple, reliable tool calling

**When to Use**:
- Integrating with existing frameworks
- Need OpenAI-compatible function format
- Building production agents
- Want minimal overhead

**Example**:
```python
from langchain.llms import Ollama
llm = Ollama(model="qwen-coder-function")
# Works seamlessly with LangChain agents
```

### 4. qwen-coder-maxcontext
**Base**: qwen3-coder:30b  
**Context**: 6,144 tokens (safe for 48GB RAM)  
**Memory**: ~30GB  
**Best For**:
- Analyzing large files
- Long conversation memory
- Multi-file code reviews
- Extended debugging sessions

**When to Use**:
- Working with files over 2,000 lines
- Need to maintain long conversation context
- Analyzing multiple related files
- Complex refactoring across files

**Example**:
```bash
ollama run qwen-coder-maxcontext "Review this entire codebase and suggest architectural improvements"
```

### 5. qwen-coder-ultra
**Base**: qwen3-coder:30b-q4_K_M (quantized)  
**Context**: 12,288 tokens  
**Memory**: ~25GB (thanks to quantization)  
**Best For**:
- Maximum context when quality can be slightly lower
- Entire file analysis
- Very long conversations
- Large documentation tasks

**When to Use**:
- Need maximum possible context
- Analyzing entire modules/packages
- Long technical discussions
- OK with ~10% quality reduction for 2x context

**Setup Required**:
```bash
# First get the quantized model
ollama pull qwen3-coder:30b-q4_K_M
# Then create ultra version
ollama create qwen-coder-ultra -f modelfiles/qwen-coder-ultra.Modelfile
```

**Example**:
```bash
ollama run qwen-coder-ultra "Analyze these 10 files and create comprehensive documentation"
```

## 📊 Model Comparison Table

| Model | Context | Memory | Speed | Quality | Special Features |
|-------|---------|--------|-------|---------|-----------------|
| **qwen-coder-tools** | 8k | 25GB | Fast | 100% | Tool calling, JSON output |
| **qwen-coder-advanced** | 16k | 35GB | Medium | 100% | Kitchen sink, diagrams, search |
| **qwen-coder-function** | 8k | 25GB | Fast | 100% | OpenAI-compatible, LangChain |
| **qwen-coder-maxcontext** | 6k | 30GB | Fast | 100% | Stable large context |
| **qwen-coder-ultra** | 12k | 25GB | Medium | 90% | Maximum context via Q4 |
| **qwen3-coder:30b** (base) | 4k | 20GB | Fastest | 100% | Default, no modifications |

## 🚀 Quick Start

```bash
# Build all models at once
./scripts/build_custom_models.sh

# Or build specific model
ollama create qwen-coder-advanced -f modelfiles/qwen-coder-advanced.Modelfile

# Test it out
ollama run qwen-coder-advanced "Create a REST API with authentication"
```

## 💡 Usage Scenarios

### Scenario 1: "I need to build an agent"
- **First choice**: `qwen-coder-function` (works with frameworks)
- **Alternative**: `qwen-coder-tools` (more control)

### Scenario 2: "I need to analyze a large codebase"
- **First choice**: `qwen-coder-ultra` (12k context)
- **Alternative**: `qwen-coder-maxcontext` (better quality, less context)

### Scenario 3: "I need system design with diagrams"
- **Only choice**: `qwen-coder-advanced` (has Mermaid support)

### Scenario 4: "I need fast responses for coding"
- **First choice**: Base `qwen3-coder:30b`
- **Alternative**: `qwen-coder-tools` (if you need JSON output)

### Scenario 5: "I'm building a production app"
- **First choice**: `qwen-coder-function` (reliable, compatible)
- **Alternative**: Base model (simplest, fastest)

## Building the Models

Run the build script:
```bash
./scripts/build_custom_models.sh
```

Or build individually:
```bash
ollama create qwen-coder-tools -f modelfiles/qwen-coder-tools.Modelfile
ollama create qwen-coder-advanced -f modelfiles/qwen-coder-advanced.Modelfile
ollama create qwen-coder-function -f modelfiles/qwen-coder-function.Modelfile
```

## Usage Examples

### Basic Tool Calling
```bash
ollama run qwen-coder-tools "Search for the latest React features and create a demo component"
```

### Web Search Simulation
```bash
ollama run qwen-coder-advanced "/search TypeScript 5.0 features"
```

### Function Calling
```bash
ollama run qwen-coder-function "Calculate the fibonacci sequence up to n=10"
```

### With Python (LangChain)
```python
from langchain.llms import Ollama
from langchain.agents import initialize_agent, Tool
from langchain.agents import AgentType

llm = Ollama(model="qwen-coder-function")

tools = [
    Tool(
        name="Calculator",
        func=lambda x: eval(x),
        description="Useful for math calculations"
    ),
]

agent = initialize_agent(
    tools, 
    llm, 
    agent=AgentType.ZERO_SHOT_REACT_DESCRIPTION,
    verbose=True
)

agent.run("What is 25 * 4 + 10?")
```

### With curl (API)
```bash
curl http://localhost:11434/api/chat -d '{
  "model": "qwen-coder-tools",
  "messages": [
    {
      "role": "user",
      "content": "Use the search function to find information about Rust async/await"
    }
  ],
  "stream": false
}'
```

## Tool Calling Format

The models recognize and generate this format:

```json
{
  "function_call": {
    "name": "search",
    "arguments": {
      "query": "Rust async await tutorial"
    }
  }
}
```

Or for multiple tools:

```json
{
  "tool_calls": [
    {
      "id": "call_1",
      "type": "function",
      "function": {
        "name": "get_weather",
        "arguments": {
          "location": "San Francisco"
        }
      }
    }
  ]
}
```

## Limitations

1. **Not true multimodal**: These models simulate multimodal capabilities through structured text
2. **No real web access**: Web search is simulated based on training data
3. **Tool execution**: The model generates tool calls but doesn't execute them - your application needs to handle execution

## Performance Tips

1. **Context Length**: These models support up to 16k context, but performance is best under 8k
2. **Temperature**: Lower (0.5-0.7) for consistent tool calling, higher (0.8-1.0) for creative tasks
3. **GPU**: These are 30B models, requiring significant VRAM (~20GB for full precision)

## Troubleshooting

### Model runs slow
- Reduce context length: `PARAMETER num_ctx 4096`
- Use quantized version: `ollama pull qwen3-coder:30b-q4_K_M`

### Tool calls not recognized
- Ensure you're using the `/api/chat` endpoint, not `/api/generate`
- Check that your prompt clearly requests tool usage
- Try the simpler `qwen-coder-function` model

### Out of memory
- Use quantized version
- Reduce batch size: `PARAMETER num_batch 256`
- Limit GPU layers: `PARAMETER num_gpu 20`

## Contributing

Feel free to create new Modelfiles for different use cases! Some ideas:
- SQL-specialized version
- Documentation writer
- Test generator
- Security analyzer
