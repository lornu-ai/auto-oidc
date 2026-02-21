.PHONY: help install dev build lint test test-all clean format type-check

help:
	@echo "Auto-OIDC Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install        - Install dependencies with Bun"
	@echo ""
	@echo "Development:"
	@echo "  make dev            - Start development server"
	@echo "  make build          - Build for production"
	@echo "  make preview        - Preview production build"
	@echo ""
	@echo "Code Quality:"
	@echo "  make lint           - Run ESLint"
	@echo "  make lint:fix       - Fix ESLint issues"
	@echo "  make format         - Format code with Prettier"
	@echo "  make format:check   - Check code formatting"
	@echo "  make type-check     - Check TypeScript types"
	@echo ""
	@echo "Testing:"
	@echo "  make test           - Run unit tests"
	@echo "  make test:ui        - Run tests with UI"
	@echo "  make test:all       - Run all tests and checks"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean          - Remove node_modules, dist, coverage"

install:
	bun install

dev:
	bun run dev

build:
	bun run build

preview:
	bun run preview

lint:
	bun run lint

lint:fix:
	bun run lint:fix

format:
	bun run format

format:check:
	bun run format:check

type-check:
	bun run type-check

test:
	bun run test

test:ui:
	bun run test:ui

test:all:
	bun run test:all

clean:
	rm -rf node_modules dist coverage playwright-report test-results
