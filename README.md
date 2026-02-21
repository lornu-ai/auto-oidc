<div align="center">
<img width="1200" height="475" alt="GHBanner" src="https://github.com/user-attachments/assets/0aa67016-6eaf-458a-adb2-6e31a0763ed6" />
</div>

# The Secure Relay - OIDC Distributed Trust Hub

An interactive architectural visualization of a distributed OIDC trust hub using Cloudflare Zero Trust, Workers, and IAM integration.

## Quick Start

### Prerequisites
- **Bun 1.x** (or Node.js 20+)
- **Git**

### Installation & Development

```bash
# Clone repository
git clone https://github.com/lornu-ai/auto-oidc.git
cd auto-oidc

# Install dependencies (with Bun)
bun install
# or with npm
npm install

# Configure environment
cp .env.example .env.local
# Edit .env.local and add your VITE_GEMINI_API_KEY

# Start development server
make dev
# or directly:
bun run dev

# Open http://localhost:3000
```

### Available Commands

```bash
make dev              # Start development server
make build            # Build for production
make lint             # Run ESLint checks
make format           # Format code with Prettier
make type-check       # Check TypeScript types
make test             # Run unit tests
make test:all         # Run all tests and checks
make help             # Show all available commands
```

## Features

- ✨ Interactive topology visualization of OIDC architecture
- 🔐 Real-time security audit via Gemini AI
- 🌐 Cloudflare Worker code examples
- 📱 Responsive design with Tailwind CSS 4
- 🎯 Modern TypeScript with strict mode
- 🧪 Comprehensive testing (unit + E2E ready)
- 🚀 Fast development with Vite 7
- 📦 Optimized production builds

## Architecture

The application visualizes a sovereign identity architecture using:

- **Zero Trust**: Cloudflare Access with identity verification
- **Token Relay**: Cloudflare Workers for JWT mutation
- **IAM Integration**: Central identity provider (Auth0/Okta)
- **Multi-Cloud**: Support for AWS, GCP, Azure

## Tech Stack

| Category | Technology |
|----------|------------|
| **Frontend** | React 19, TypeScript 5.9 |
| **Build** | Vite 7, Tailwind CSS 4 |
| **Package Manager** | Bun 1.x |
| **Code Quality** | ESLint 9, Prettier 3 |
| **Type Checking** | TypeScript strict mode |
| **Testing** | Vitest, Playwright (ready) |

## Environment Variables

Create `.env.local` file based on `.env.example`:

```env
# Required: Get from https://aistudio.google.com/app/apikey
VITE_GEMINI_API_KEY=your-api-key

# Optional: API configuration
VITE_API_BASE_URL=http://localhost:3000
VITE_DEBUG=false
```

## Project Structure

```
auto-oidc/
├── src/
│   ├── components/      # (Ready for component extraction)
│   ├── pages/           # Page components (App.tsx)
│   ├── hooks/           # Custom React hooks
│   ├── services/        # Business logic services
│   ├── utils/           # Utilities and constants
│   ├── styles/          # Global styles
│   └── main.tsx         # Application entry point
├── .github/workflows/   # CI/CD pipelines
├── Makefile             # Development commands
├── vite.config.ts       # Vite configuration
└── tsconfig.*.json      # TypeScript configurations
```

## Development Workflow

### Code Quality

```bash
# Lint code
make lint

# Fix linting issues
make lint:fix

# Format code
make format

# Check formatting
make format:check

# Type check
make type-check
```

### Pre-commit Hooks

Install pre-commit hooks to automate checks:

```bash
pre-commit install
```

Hooks run automatically on commit and will:
- Format code with Prettier
- Fix ESLint issues
- Detect secrets and large files
- Validate YAML and JSON

## CI/CD Pipelines

Automated workflows on GitHub Actions:

### Lint & Format Check (`lint-and-format.yml`)
- Prettier format validation
- ESLint code quality checks
- TypeScript strict mode validation
- Runs on: PR, push to main/develop

### Test & Build (`test-and-build.yml`)
- Production build verification
- Artifact upload for deployment
- Smart path-based triggering (skip docs-only changes)
- Runs on: PR, push to main/develop

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:
- Code standards
- Development setup
- Testing requirements
- Pull request process

## License

[View LICENSE](./LICENSE)

## Getting Help

- 📖 [Project Documentation](./docs)
- 🐛 [Report Issues](https://github.com/lornu-ai/auto-oidc/issues)
- 💬 [Discussions](https://github.com/lornu-ai/auto-oidc/discussions)

## Acknowledgments

Built with best practices from:
- [stevedores-org](https://github.com/stevedores-org)
- [lornu-ai](https://github.com/lornu-ai)
- [Cloudflare Workers](https://developers.cloudflare.com/workers/)
- [OIDC Specification](https://openid.net/connect/)
