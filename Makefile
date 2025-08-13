# 🦙✨ Ollamamon Development Makefile
# A delightful way to develop your Ollama TUI

# Colors for beautiful output
CYAN := \033[0;36m
MAGENTA := \033[0;35m
YELLOW := \033[1;33m
GREEN := \033[0;32m
RED := \033[0;31m
RESET := \033[0m
BOLD := \033[1m

# Default target shows help
.DEFAULT_GOAL := help

.PHONY: help
help: ## 🌙 Show this help message
	@echo "$(BOLD)$(MAGENTA)✨ Ollamamon Development Commands ✨$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Happy coding! 🚀$(RESET)"

.PHONY: build
build: ## 🔨 Build the project in debug mode
	@echo "$(CYAN)Building Ollamamon...$(RESET)"
	@cargo build
	@echo "$(GREEN)✓ Build complete!$(RESET)"

.PHONY: release
release: ## 🚀 Build optimized release version
	@echo "$(MAGENTA)Building release version...$(RESET)"
	@cargo build --release
	@echo "$(GREEN)✓ Release build complete!$(RESET)"
	@ls -lh target/release/llamaloop | awk '{print "$(YELLOW)Binary size: " $$5 "$(RESET)"}'

.PHONY: run
run: ## ▶️  Run the TUI application
	@echo "$(CYAN)Starting Ollamamon...$(RESET)"
	@cargo run

.PHONY: dev
dev: ## 🔄 Run with auto-reload on changes (requires cargo-watch)
	@command -v cargo-watch >/dev/null 2>&1 || (echo "$(YELLOW)Installing cargo-watch...$(RESET)" && cargo install cargo-watch)
	@echo "$(CYAN)Watching for changes...$(RESET)"
	@cargo watch -x run

.PHONY: test
test: ## 🧪 Run tests
	@echo "$(CYAN)Running tests...$(RESET)"
	@cargo test -- --nocapture
	@echo "$(GREEN)✓ Tests complete!$(RESET)"

.PHONY: check
check: ## ✅ Check code without building
	@echo "$(CYAN)Checking code...$(RESET)"
	@cargo check
	@echo "$(GREEN)✓ Check complete!$(RESET)"

.PHONY: lint
lint: ## 🎨 Run clippy linter
	@echo "$(MAGENTA)Running clippy...$(RESET)"
	@cargo clippy -- -D warnings 2>/dev/null || cargo clippy
	@echo "$(GREEN)✓ Linting complete!$(RESET)"

.PHONY: fmt
fmt: ## 💅 Format code with rustfmt
	@echo "$(CYAN)Formatting code...$(RESET)"
	@cargo fmt
	@echo "$(GREEN)✓ Code formatted!$(RESET)"

.PHONY: fmt-check
fmt-check: ## 📋 Check if code is formatted
	@echo "$(CYAN)Checking formatting...$(RESET)"
	@cargo fmt -- --check || (echo "$(RED)✗ Code needs formatting. Run 'make fmt'$(RESET)" && exit 1)
	@echo "$(GREEN)✓ Code is properly formatted!$(RESET)"

.PHONY: clean
clean: ## 🧹 Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	@cargo clean
	@echo "$(GREEN)✓ Cleaned!$(RESET)"

.PHONY: install
install: release ## 📦 Install llamaloop to cargo bin directory
	@echo "$(MAGENTA)Installing llamaloop...$(RESET)"
	@cargo install --path .
	@echo "$(GREEN)✓ Installed to cargo bin!$(RESET)"
	@echo "$(YELLOW)Run 'llamaloop' from anywhere!$(RESET)"

.PHONY: uninstall
uninstall: ## 🗑️  Uninstall llamaloop
	@echo "$(YELLOW)Uninstalling llamaloop...$(RESET)"
	@cargo uninstall llamaloop
	@echo "$(GREEN)✓ Uninstalled!$(RESET)"

.PHONY: bench
bench: ## ⚡ Run benchmarks
	@echo "$(CYAN)Running benchmarks...$(RESET)"
	@cargo bench
	@echo "$(GREEN)✓ Benchmarks complete!$(RESET)"

.PHONY: doc
doc: ## 📚 Generate and open documentation
	@echo "$(CYAN)Generating documentation...$(RESET)"
	@cargo doc --open
	@echo "$(GREEN)✓ Documentation opened in browser!$(RESET)"

.PHONY: deps
deps: ## 📊 Show dependency tree
	@command -v cargo-tree >/dev/null 2>&1 || cargo install cargo-tree
	@echo "$(CYAN)Dependency tree:$(RESET)"
	@cargo tree

.PHONY: update
update: ## 🔄 Update dependencies
	@echo "$(YELLOW)Updating dependencies...$(RESET)"
	@cargo update
	@echo "$(GREEN)✓ Dependencies updated!$(RESET)"

.PHONY: size
size: release ## 📏 Analyze binary size
	@echo "$(CYAN)Binary size analysis:$(RESET)"
	@ls -lh target/release/llamaloop
	@echo ""
	@echo "$(YELLOW)Top 10 largest functions:$(RESET)"
	@cargo bloat --release -n 10 2>/dev/null || (echo "Install cargo-bloat for detailed analysis: cargo install cargo-bloat" && ls -lh target/release/llamaloop)

.PHONY: pre-commit
pre-commit: fmt-check lint test ## 🎯 Run all checks before committing
	@echo "$(GREEN)✨ All pre-commit checks passed!$(RESET)"

.PHONY: quick
quick: fmt build run ## ⚡ Format, build and run quickly

.PHONY: ollama-start
ollama-start: ## 🦙 Start Ollama service
	@echo "$(CYAN)Starting Ollama...$(RESET)"
	@ollama serve > /dev/null 2>&1 & 
	@sleep 2
	@echo "$(GREEN)✓ Ollama started!$(RESET)"

.PHONY: ollama-stop
ollama-stop: ## 🛑 Stop Ollama service
	@echo "$(YELLOW)Stopping Ollama...$(RESET)"
	@pkill ollama || true
	@echo "$(GREEN)✓ Ollama stopped!$(RESET)"

.PHONY: demo
demo: ollama-start release ## 🎬 Run a demo (starts Ollama and runs the app)
	@echo "$(MAGENTA)✨ Starting Ollamamon Demo ✨$(RESET)"
	@sleep 1
	@./target/release/llamaloop

.PHONY: sparkle
sparkle: ## ✨ A surprise!
	@echo "$(MAGENTA)"
	@echo "       ✨  ✨  ✨       "
	@echo "    ✨   🦙  ✨        "
	@echo "  ✨  Ollamamon  ✨    "
	@echo "    ✨  Rocks!  ✨     "
	@echo "       ✨  ✨  ✨       "
	@echo "$(RESET)"
	@echo "$(CYAN)Keep coding with joy! 🚀$(RESET)"

# Special targets for development workflow
.PHONY: save
save: fmt pre-commit ## 💾 Format and verify before saving
	@echo "$(GREEN)✓ Code is clean and ready to commit!$(RESET)"

.PHONY: reset
reset: clean ## 🔄 Full reset (clean everything)
	@rm -rf Cargo.lock
	@echo "$(GREEN)✓ Project reset complete!$(RESET)"

# Watch for specific file types
.PHONY: watch-ui
watch-ui: ## 👁️  Watch and rebuild on UI changes
	@command -v cargo-watch >/dev/null 2>&1 || cargo install cargo-watch
	@cargo watch -w src/ui.rs -w src/theme.rs -x run

# Performance profiling (macOS)
.PHONY: profile
profile: ## 📈 Profile the application (macOS)
	@echo "$(CYAN)Building with profiling...$(RESET)"
	@cargo build --release
	@echo "$(YELLOW)Run: cargo instruments -t 'CPU Profiler' --bin llamaloop$(RESET)"
	@echo "$(YELLOW)Or use: samply record ./target/release/llamaloop$(RESET)"

.PHONY: todo
todo: ## 📝 Show all TODOs in the codebase
	@echo "$(MAGENTA)TODOs in codebase:$(RESET)"
	@grep -r "TODO\|FIXME\|HACK\|NOTE" --include="*.rs" src/ || echo "$(GREEN)No TODOs found!$(RESET)"