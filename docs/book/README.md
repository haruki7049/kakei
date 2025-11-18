# kakei Documentation Book

This directory contains the source files for the kakei documentation book, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Target Audience

This documentation is designed for:

- **New Users**: People who want to start using kakei to manage their personal finances
  - Installation guides and quick start tutorials
  - Basic command usage and examples
  
- **Developers**: Contributors who want to understand and extend kakei
  - Architecture overview
  - Development setup and testing
  - Contribution guidelines
  
- **Power Users**: Advanced users who want to leverage kakei's full potential
  - Lisp-based data transformations
  - Configuration options
  - Real-world usage examples

## Building the Documentation

### Prerequisites

- [mdBook](https://rust-lang.github.io/mdBook/) - Install with `cargo install mdbook`

### Build and Preview Locally

```bash
# Navigate to the book directory
cd docs/book

# Build the book (output goes to docs/book/book/)
mdbook build

# Or serve it locally with live reload
mdbook serve

# Open http://localhost:3000 in your browser
```

### Build Output

The built documentation will be generated in the `docs/book/book/` directory (configured in `book.toml`).

## Structure

- `book.toml` - Configuration file for mdBook
- `src/` - Source markdown files for each chapter
  - `SUMMARY.md` - Table of contents
  - `*.md` - Individual chapter files

## Deployment

The documentation is automatically built and deployed to GitHub Pages via GitHub Actions when changes are pushed to the `main` branch. See `.github/workflows/deploy-docs.yml` for details.

## Contributing

To contribute to the documentation:

1. Edit the relevant `.md` files in the `src/` directory
2. Preview your changes locally with `mdbook serve`
3. Submit a pull request

For more details, see the [Contributing Guide](src/contributing.md).
