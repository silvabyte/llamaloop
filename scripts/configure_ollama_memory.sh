#!/bin/bash

# Ollama Memory and Context Configuration Script

echo "========================================="
echo "Ollama Memory & Context Configuration"
echo "========================================="
echo ""

# Get system info
TOTAL_RAM=$(sysctl -n hw.memsize 2>/dev/null || grep MemTotal /proc/meminfo | awk '{print $2*1024}' 2>/dev/null)
TOTAL_RAM_GB=$((TOTAL_RAM / 1073741824))

echo "System RAM: ${TOTAL_RAM_GB}GB"
echo ""

echo "Current Ollama settings:"
echo "------------------------"
ollama show gpt-oss:20b --modelfile 2>/dev/null || echo "Model not yet pulled"
echo ""

# Create optimized Modelfile for large models
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cat > "$SCRIPT_DIR/Modelfile-optimized" << 'EOF'
# Optimized configuration for large models

FROM gpt-oss:20b

# Context window settings
PARAMETER num_ctx 8192          # Increase context window (default is often 2048)
PARAMETER num_batch 512         # Batch size for prompt processing
PARAMETER num_gpu 999           # Use all available GPU layers (if you have GPU)

# Memory and performance settings
PARAMETER num_thread 8          # Number of threads (adjust based on CPU cores)
PARAMETER num_keep 24           # Number of tokens to keep from initial prompt

# Temperature and generation settings
PARAMETER temperature 0.7
PARAMETER top_k 40
PARAMETER top_p 0.9
PARAMETER repeat_penalty 1.1

# Memory allocation settings
PARAMETER mmap true             # Use memory mapping for efficient loading
PARAMETER f16_kv true           # Use 16-bit floats for key/value cache (saves memory)

# System prompt (optional)
SYSTEM "You are a helpful AI assistant."
EOF

echo "Created optimized Modelfile at: $SCRIPT_DIR/Modelfile-optimized"
echo ""

echo "========================================="
echo "Environment Variables for Ollama"
echo "========================================="
echo ""

# Create environment configuration script
cat > "$SCRIPT_DIR/ollama_env.sh" << 'EOF'
#!/bin/bash

# Ollama Environment Configuration

# Memory settings
export OLLAMA_MAX_LOADED_MODELS=1      # Only keep 1 model loaded at a time
export OLLAMA_NUM_PARALLEL=1           # Number of parallel requests
export OLLAMA_MAX_QUEUE=10             # Maximum queue size

# GPU settings (if applicable)
export OLLAMA_GPU_LAYERS=999           # Use all available GPU layers
# export CUDA_VISIBLE_DEVICES=0        # For NVIDIA GPUs
# export HSA_OVERRIDE_GFX_VERSION=10.3.0  # For AMD GPUs

# Memory limits
export OLLAMA_MODELS=/usr/local/share/ollama/models  # Model storage location
export OLLAMA_KEEP_ALIVE=5m            # How long to keep models loaded

# Context and generation
export OLLAMA_NUM_CTX=8192             # Default context size for all models

# Network settings (for serving)
export OLLAMA_HOST=0.0.0.0             # Allow network access
export OLLAMA_ORIGINS="*"              # Allow all origins (be careful in production)

echo "Ollama environment variables set!"
echo ""
echo "Current settings:"
env | grep OLLAMA
EOF

chmod +x "$SCRIPT_DIR/ollama_env.sh"

echo "========================================="
echo "How to use these configurations:"
echo "========================================="
echo ""
echo "1. Set environment variables before starting Ollama:"
echo "   source $SCRIPT_DIR/ollama_env.sh"
echo "   ollama serve"
echo ""
echo "2. Create an optimized version of a model:"
echo "   ollama create model-optimized -f $SCRIPT_DIR/Modelfile-optimized"
echo ""
echo "3. For permanent configuration on macOS, update the plist file"
echo ""

# Create plist with memory settings for macOS
if [ "$(uname -s)" = "Darwin" ]; then
cat > "$SCRIPT_DIR/com.ollama.server-optimized.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.ollama.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/ollama</string>
        <string>serve</string>
    </array>
    <key>EnvironmentVariables</key>
    <dict>
        <key>OLLAMA_HOST</key>
        <string>0.0.0.0</string>
        <key>OLLAMA_MAX_LOADED_MODELS</key>
        <string>1</string>
        <key>OLLAMA_NUM_PARALLEL</key>
        <string>1</string>
        <key>OLLAMA_KEEP_ALIVE</key>
        <string>5m</string>
        <key>OLLAMA_NUM_CTX</key>
        <string>8192</string>
        <key>OLLAMA_GPU_LAYERS</key>
        <string>999</string>
    </dict>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/ollama.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/ollama.error.log</string>
</dict>
</plist>
EOF

echo "   cp $SCRIPT_DIR/com.ollama.server-optimized.plist ~/Library/LaunchAgents/com.ollama.server.plist"
echo "   launchctl unload ~/Library/LaunchAgents/com.ollama.server.plist"
echo "   launchctl load ~/Library/LaunchAgents/com.ollama.server.plist"
fi

echo ""
echo "========================================="
echo "Memory Optimization Tips for Large Models:"
echo "========================================="
echo ""
echo "1. Use quantized versions:"
echo "   ollama pull model:q4_0    # 4-bit quantization (uses less memory)"
echo "   ollama pull model:q5_K_M  # 5-bit quantization (balanced)"
echo ""
echo "2. Reduce context window if needed:"
echo "   Set num_ctx to 4096 or 2048 in Modelfile"
echo ""
echo "3. Use CPU offloading if running out of GPU memory:"
echo "   Set num_gpu to a lower value (e.g., 20 instead of 999)"
echo ""
echo "4. Monitor memory usage:"
echo "   ollama ps                        # Show loaded models"
echo "   top -pid \$(pgrep ollama)        # Monitor Ollama process"
echo ""
echo "5. Unload models when not in use:"
echo "   curl -X DELETE http://localhost:11434/api/generate -d '{\"model\": \"model-name\"}'"
echo ""

echo "========================================="
echo "Quick memory calculation:"
echo "========================================="
echo ""
echo "Model size requirements (approximate):"
echo "- Full precision (FP16): 2x model parameters in GB"
echo "- 8-bit quantization: 1x model parameters in GB"
echo "- 4-bit quantization: 0.5x model parameters in GB"
echo "- Plus context memory: ~2-4GB for 8k context"
echo ""
echo "Your system has ${TOTAL_RAM_GB}GB RAM"
if [ $TOTAL_RAM_GB -lt 16 ]; then
    echo "⚠️  Recommended: Use smaller or quantized models"
elif [ $TOTAL_RAM_GB -lt 32 ]; then
    echo "⚠️  Recommended: Use quantized versions for large models"
else
    echo "✅ Should handle most models with proper settings"
fi
echo ""
echo "========================================="
echo "✅ Configuration complete!"
echo "========================================="