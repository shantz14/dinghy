# Variables
BINARY_DIR := bin
CMD_DIR := cmd

# Automatically detect all subdirectories in cmd/ that contain code
SERVICES := $(notdir $(wildcard $(CMD_DIR)/*))

.PHONY: all build clean test $(SERVICES)

# Default target runs tests and builds all binaries
all: test build

# Dynamic build target that evaluates all detected binaries
build: $(SERVICES)

# Rule for building each individual binary dynamically
$(SERVICES):
	@echo "Building $@..."
	@mkdir -p $(BINARY_DIR)
	go build -o $(BINARY_DIR)/$@ ./$(CMD_DIR)/$@

# Run unit tests across the whole project
test:
	@echo "Running tests..."
	go test ./...

# Clean up built binaries
clean:
	@echo "Cleaning up built binaries..."
	rm -rf $(BINARY_DIR)
