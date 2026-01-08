# Contributing to nasdaqball

Thank you for your interest in contributing to nasdaqball!

## Code of Conduct

Be respectful, constructive, and professional.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported
2. Create a new issue with:
   - Clear title
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details

### Suggesting Features

1. Open an issue with the `enhancement` label
2. Describe the feature and use case
3. Discuss with maintainers before implementing

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass: `pnpm test`
6. Commit with clear messages
7. Push to your fork
8. Open a pull request

## Development Setup

```bash
# Clone the repo
git clone https://github.com/rthefinder/nasdaqball.git
cd nasdaqball

# Install dependencies
pnpm install

# Run tests
pnpm test

# Start dev server
pnpm dev
```

## Coding Standards

### TypeScript/JavaScript
- Use TypeScript for all new code
- Follow ESLint rules
- Use Prettier for formatting
- Add JSDoc comments for public APIs

### Rust
- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add comprehensive comments

### Testing
- Write tests for new features
- Maintain or improve coverage
- Test edge cases

## Commit Messages

Follow conventional commits:

```
feat: add new feature
fix: bug fix
docs: documentation update
test: add tests
chore: maintenance
```

## Questions?

Open an issue or reach out to maintainers.

Thank you for contributing! 🚀
