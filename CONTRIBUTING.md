# Contributing to Auto-OIDC

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites
- Bun 1.x (or Node.js 20+)
- Git

### Setup

```bash
# Clone repository
git clone https://github.com/lornu-ai/auto-oidc.git
cd auto-oidc

# Install dependencies
bun install

# Copy environment file
cp .env.example .env.local

# Add your Gemini API key to .env.local
```

### Development

```bash
# Start development server
make dev

# The app runs on http://localhost:3000 with hot reload
```

## Code Standards

### TypeScript
- ✅ Strict mode enabled - all files must pass type checking
- ✅ Explicit return types for functions
- ❌ Avoid `any` type - use proper types or `unknown`
- ✅ Use proper imports (no default imports for utilities)

### ESLint & Prettier
- Code is automatically formatted on commit (pre-commit hooks)
- Run `make format` before pushing
- Run `make lint` to check for issues
- Run `make lint:fix` to auto-fix ESLint issues

### File Organization
- Separate concerns: components, hooks, services, utils
- Use `.ts` for logic/utilities, `.tsx` for React components
- Export named functions/constants (not default exports)
- Keep files focused and under 300 lines

### Naming Conventions
- `useXxx` for React hooks
- `XxxService` for service classes
- `Xxx.tsx` for React components
- `xxx.ts` for utilities
- `CONSTANT_NAME` for constants

## Testing

### Unit Tests (Vitest)
```bash
make test           # Run all tests
make test:ui        # Interactive test UI
make test:all       # Full test suite with coverage
```

### E2E Tests (Playwright)
```bash
bun run test:e2e    # Run Playwright tests
```

**Guidelines:**
- Write tests for new hooks and services
- Aim for >70% code coverage
- Use descriptive test names
- Mock external APIs

## Git Workflow

### Branch Names
- Feature: `feat/description`
- Fix: `fix/description`
- Docs: `docs/description`
- Style: `style/description`

### Commits
- Use clear, imperative message format
- Reference issues: `Closes #123`
- Keep commits focused and atomic
- Example: `feat: add security audit button`

### Pull Requests
1. Create feature branch: `git checkout -b feat/description`
2. Make changes and test thoroughly
3. Run `make test:all` before pushing
4. Push to your fork and create PR
5. Ensure CI checks pass
6. Request review from maintainers

## Code Review Checklist

Before submitting a PR, ensure:

- [ ] Code follows project style guide
- [ ] All tests pass: `make test:all`
- [ ] ESLint passes: `make lint`
- [ ] Code is formatted: `make format`
- [ ] TypeScript compiles: `make type-check`
- [ ] No console.log or debug code left
- [ ] Comments added for complex logic
- [ ] PR description explains changes

## Documentation

- Update README.md for user-facing changes
- Add JSDoc comments for public functions
- Update .env.example if adding new environment variables
- Include examples in commit messages

## Questions?

- 💬 Open a [Discussion](https://github.com/lornu-ai/auto-oidc/discussions)
- 🐛 Report [Issues](https://github.com/lornu-ai/auto-oidc/issues)
- 📖 Check [Documentation](./docs)

## Thank You!

Your contributions help make this project better. We appreciate your effort and time!
