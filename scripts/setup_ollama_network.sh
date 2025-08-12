#!/bin/bash

# Ollama Network Setup Script
# Configures Ollama for network access

echo "========================================="
echo "Ollama Network Configuration Setup"
echo "========================================="
echo ""

# Detect OS
OS=$(uname -s)

if [ "$OS" = "Darwin" ]; then
    echo "🍎 Detected macOS"
    echo ""
    
    # Create launchd plist for macOS
    PLIST_PATH="$HOME/Library/LaunchAgents/com.ollama.server.plist"
    
    echo "Creating launchd service configuration..."
    
    cat > "$PLIST_PATH" << 'EOF'
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
    
    echo "✅ Created plist at: $PLIST_PATH"
    echo ""
    echo "To start Ollama with network access:"
    echo "1. Stop current Ollama if running: pkill ollama"
    echo "2. Load the service: launchctl load $PLIST_PATH"
    echo "3. Start the service: launchctl start com.ollama.server"
    echo ""
    echo "To stop the service:"
    echo "   launchctl stop com.ollama.server"
    echo "   launchctl unload $PLIST_PATH"
    
elif [ "$OS" = "Linux" ]; then
    echo "🐧 Detected Linux"
    echo ""
    
    # Create systemd service for Linux
    SERVICE_PATH="/etc/systemd/system/ollama.service"
    
    echo "Creating systemd service configuration..."
    echo "Note: This requires sudo privileges"
    
    sudo tee "$SERVICE_PATH" > /dev/null << 'EOF'
[Unit]
Description=Ollama Service
After=network.target

[Service]
Type=simple
User=$USER
Environment="OLLAMA_HOST=0.0.0.0"
ExecStart=/usr/local/bin/ollama serve
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF
    
    echo "✅ Created service at: $SERVICE_PATH"
    echo ""
    echo "To start Ollama with network access:"
    echo "1. Reload systemd: sudo systemctl daemon-reload"
    echo "2. Enable service: sudo systemctl enable ollama"
    echo "3. Start service: sudo systemctl start ollama"
    echo ""
    echo "To check status:"
    echo "   sudo systemctl status ollama"
else
    echo "⚠️  Unsupported OS: $OS"
    echo ""
    echo "Manual setup required:"
    echo "1. Set OLLAMA_HOST=0.0.0.0 environment variable"
    echo "2. Run: ollama serve"
fi

echo ""
echo "========================================="
echo "🔒 Security Considerations"
echo "========================================="
echo "⚠️  WARNING: Setting OLLAMA_HOST=0.0.0.0 exposes Ollama to your network!"
echo ""
echo "Security recommendations:"
echo "1. Use a firewall to restrict access to trusted IPs"
echo "2. Consider using a reverse proxy with authentication"
echo "3. Monitor access logs regularly"
echo ""

# Get local IP address
if [ "$OS" = "Darwin" ]; then
    LOCAL_IP=$(ipconfig getifaddr en0 2>/dev/null || ipconfig getifaddr en1 2>/dev/null || echo "YOUR_IP")
else
    LOCAL_IP=$(hostname -I | awk '{print $1}' 2>/dev/null || echo "YOUR_IP")
fi

echo "========================================="
echo "📱 Testing Network Access"
echo "========================================="
echo ""
echo "Your local IP appears to be: $LOCAL_IP"
echo ""
echo "After starting Ollama with network access, test from another device:"
echo "   curl http://$LOCAL_IP:11434/api/tags"
echo ""
echo "Or test the chat endpoint:"
echo "   curl http://$LOCAL_IP:11434/api/generate -d '{"
echo '     "model": "gemma2:9b",'
echo '     "prompt": "Hello!",'
echo '     "stream": false'
echo "   }'"
echo ""
echo "========================================="
echo "✅ Setup script complete!"
echo "========================================="