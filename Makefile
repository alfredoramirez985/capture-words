.PHONY: build run debug build-debug dev clean

# Default target
all: build

# Install dependencies (if needed)
install:
	npm install

# Build the Tauri application for release
build:
	npm run tauri build

# Run the Tauri application in development mode (hot-reload)
run:
	npm run tauri dev

# Run alias for development mode
dev: run

# Build the Tauri application in debug mode
build-debug:
	npm run tauri build -- --debug

# Alias if "mode debug" meant dev or build-debug
debug: build-debug

# Clean build artifacts and dependencies
clean:
	rm -rf node_modules .svelte-kit src-tauri/target
