#!/bin/bash

# Check and calculate context window limits for models

echo "========================================="
echo "🧮 Context Window Calculator for Ollama"
echo "========================================="
echo ""

# Get system memory
if [[ "$OSTYPE" == "darwin"* ]]; then
    TOTAL_RAM_BYTES=$(sysctl -n hw.memsize)
    GPU_CHECK=$(system_profiler SPDisplaysDataType 2>/dev/null | grep -i "Metal\|GPU" | head -1)
else
    TOTAL_RAM_BYTES=$(grep MemTotal /proc/meminfo | awk '{print $2 * 1024}')
    GPU_CHECK=$(nvidia-smi 2>/dev/null | grep "NVIDIA" | head -1)
fi

TOTAL_RAM_GB=$((TOTAL_RAM_BYTES / 1073741824))

echo "System Specifications:"
echo "---------------------"
echo "Total RAM: ${TOTAL_RAM_GB}GB"
echo "GPU: ${GPU_CHECK:-Not detected}"
echo ""

echo "Memory Requirements by Context Length:"
echo "--------------------------------------"
echo "Model: qwen3-coder:30b (30B parameters)"
echo ""

# Calculate memory requirements
# Formula: Additional memory ≈ 4 * num_layers * context_length * context_length * 2 bytes / 1GB
# For 30B model, assuming ~80 layers

calculate_memory() {
    local context=$1
    local layers=80
    local bytes_per_element=2
    
    # KV cache memory in GB
    local kv_cache_gb=$(echo "scale=2; (4 * $layers * $context * $context * $bytes_per_element) / 1073741824" | bc)
    
    # Activation memory (roughly 10% of KV cache)
    local activation_gb=$(echo "scale=2; $kv_cache_gb * 0.1" | bc)
    
    # Total additional memory
    local total_gb=$(echo "scale=2; $kv_cache_gb + $activation_gb" | bc)
    
    echo "$total_gb"
}

echo "Context | Additional RAM | Total RAM Needed | Feasible?"
echo "--------|---------------|------------------|----------"

for context in 2048 4096 8192 16384 32768 65536 131072 262144; do
    additional_ram=$(calculate_memory $context)
    base_model_ram=20  # Base model needs ~20GB
    total_ram=$(echo "scale=2; $base_model_ram + $additional_ram" | bc)
    
    if (( $(echo "$total_ram <= $TOTAL_RAM_GB" | bc -l) )); then
        feasible="✅ Yes"
    else
        feasible="❌ No"
    fi
    
    printf "%-7s | %-13s | %-16s | %s\n" \
        "${context}" \
        "${additional_ram}GB" \
        "${total_ram}GB" \
        "$feasible"
done

echo ""
echo "========================================="
echo "🎯 Actual Model Limits"
echo "========================================="
echo ""

# Check actual model configuration
if ollama show qwen3-coder:30b --modelfile 2>/dev/null | grep -q "PARAMETER"; then
    echo "Current qwen3-coder:30b configuration:"
    ollama show qwen3-coder:30b --modelfile 2>/dev/null | grep "num_ctx"
else
    echo "Model qwen3-coder:30b not found locally"
fi

echo ""
echo "Maximum theoretical context for Qwen3:"
echo "- Training limit: ~32,768 tokens (32k)"
echo "- Practical limit on your system: $([ $TOTAL_RAM_GB -ge 64 ] && echo "32,768" || echo "16,384") tokens"
echo "- Recommended for speed: 8,192 tokens"
echo ""

echo "========================================="
echo "⚠️  Important Notes"
echo "========================================="
echo ""
echo "1. Models CANNOT exceed their training context!"
echo "   - Qwen3 was trained with 32k max context"
echo "   - Using more will cause errors or nonsense"
echo ""
echo "2. Quadratic scaling means:"
echo "   - 2x context = 4x memory"
echo "   - 2x context = 4x slower"
echo ""
echo "3. For 256k context, you'd need:"
echo "   - A model trained for 256k (like Claude)"
echo "   - ~1TB of RAM"
echo "   - Extreme patience (hours per response)"
echo ""
echo "========================================="