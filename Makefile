# The shell to run the makefile with must be defined to work properly in Linux systems
SHELL := /bin/bash

# all the recipes are phony (no files to check).
.PHONY: build dev test fmt clean help local pre-commit

.DEFAULT_GOAL := help

# Output descriptions of all commands
help:
	@echo "Please use 'make <target>', where <target> is one of"
	@echo ""
	@echo "  help                             outputs this helper"
	@echo "  build						      builds the project"
	@echo "  dev 							  runs the project in development mode"
	@echo "  test							  runs the tests"
	@echo "  fmt						  	  runs the golang fmt"
	@echo "  clean						  	  cleans the project"
	@echo "  all-local						  runs all the local targets"
	@echo "  pre-commit						  runs the pre-commit hooks"
	@echo ""
	@echo "Check the Makefile to know exactly what each target is doing."

# Build the project
build:
	@echo "Building the project..."
	@cargo build --release

# Run the project in development mode
dev:
	@echo "Running the project in development mode..."
	@cargo run

# Run the tests
test:
	@echo "Running the tests..."
	@cargo test

# Run the golang fmt
fmt:
	@echo "Running the cargo fmt..."
	@cargo fmt

# Clean the project
clean:
	@echo "Cleaning the project..."
	@cargo clean


# Run all-local targets
#dev is commented out because our project doesn't have a main function
all-local: clean fmt build #dev

# Run the pre-commit hooks
pre-commit: fmt test



