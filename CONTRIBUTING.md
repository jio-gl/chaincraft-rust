# Contributing to ChainCraft Rust

Thank you for your interest in contributing to ChainCraft Rust! This document provides guidelines and information for contributors.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please treat all contributors with respect and create a welcoming environment for everyone.

## How to Contribute

### Reporting Issues

- Use the GitHub issue tracker to report bugs or request features
- Search existing issues before creating a new one
- Provide clear, detailed information including:
  - Steps to reproduce the issue
  - Expected vs actual behavior  
  - Your environment (OS, Rust version, etc.)
  - Relevant code samples or error messages

### Submitting Changes

1. **Fork the repository** and create a new branch from `main`
2. **Make your changes** with clear, focused commits
3. **Add tests** for any new functionality
4. **Update documentation** as needed
5. **Run the test suite** to ensure everything passes
6. **Submit a pull request** with a clear description

### Development Setup

#### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

#### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/chaincraft-rust.git
cd chaincraft-rust

# Create a feature branch
git checkout -b feature/your-feature-name

# Install dependencies and build
cargo build

# Run tests
cargo test
```

#### Development Workflow

1. **Write tests first** when adding new functionality
2. **Keep commits focused** - one logical change per commit
3. **Write clear commit messages**:
   ```
   feat: add new consensus mechanism
   
   - Implement PBFT consensus algorithm
   - Add configuration options
   - Include comprehensive tests
   ```

4. **Follow Rust conventions**:
   - Run `cargo fmt` before committing
   - Run `cargo clippy` and fix any warnings
   - Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

#### Testing

- Write unit tests for individual components
- Write integration tests for cross-component functionality
- Ensure all tests pass: `cargo test --all-features`
- Add benchmarks for performance-critical code

#### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Update the README if adding user-facing features
- Build docs locally: `cargo doc --open --all-features`

### Coding Standards

#### Style Guidelines

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` and address all warnings
- Prefer explicit types over `auto` when it improves clarity
- Use meaningful variable and function names
- Keep functions focused and reasonably sized

#### Architecture Principles

- **Modularity**: Keep components loosely coupled
- **Testability**: Design for easy unit testing
- **Performance**: Consider performance implications
- **Security**: Always validate inputs and handle errors
- **Documentation**: Code should be self-documenting

#### Error Handling

- Use the `?` operator for error propagation
- Create custom error types when appropriate
- Provide meaningful error messages
- Handle all error cases explicitly

### Pull Request Process

1. **Update documentation** for any new features
2. **Add appropriate tests** with good coverage
3. **Ensure CI passes** - all tests, formatting, and linting
4. **Keep PRs focused** - one feature or fix per PR
5. **Write a clear description** explaining the changes

#### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature  
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new compiler warnings
```

### Release Process

Releases follow semantic versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)  
- **PATCH**: Bug fixes (backward compatible)

Only maintainers can create releases. The process is automated through GitHub Actions.

### Performance Considerations

- Profile performance-critical code
- Use benchmarks to validate optimizations
- Consider memory usage and allocation patterns
- Prefer zero-copy operations when possible

### Security Guidelines

- Never commit secrets or sensitive data
- Validate all external inputs
- Use secure cryptographic libraries
- Follow security best practices for blockchain applications

### Getting Help

- **Documentation**: Check the README and docs.rs
- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Use GitHub Issues for bugs and feature requests
- **Code Review**: Maintainers will review PRs and provide feedback

### Recognition

Contributors will be recognized in:
- The AUTHORS file
- Release notes for significant contributions
- The project README

Thank you for contributing to ChainCraft Rust! 🎉 