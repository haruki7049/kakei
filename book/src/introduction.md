# Introduction

Welcome to the **kakei** documentation!

`kakei` is a command-line application for managing personal finances using the Japanese kakeibo (家計簿) method. It provides transaction tracking, categorization, and powerful Lisp-based table transformations for flexible analysis and reporting.

## What is Kakeibo?

Kakeibo (家計簿) is a Japanese household financial ledger system that emphasizes mindful spending and saving. The word combines:
- 家計 (kakei): household finances
- 簿 (bo): ledger/account book

## Key Features

- **📊 Transaction Management**: Add, list, and manage financial transactions
- **🏷️ Category & Account Organization**: Organize transactions by customizable categories and accounts
- **🔄 Lisp-Based Transformations**: Transform and analyze transaction data using a small Lisp dialect
- **📋 Table Display**: Beautiful table formatting using the `tabled` crate
- **💾 SQLite Database**: Persistent storage with automatic migrations
- **⚙️ Configuration**: Customizable categories and accounts

## Why kakei?

Traditional financial tracking tools can be rigid and inflexible. `kakei` takes a different approach:

1. **Flexible Data Analysis**: Use Lisp expressions to transform and analyze your financial data in any way you need
2. **Command-Line First**: Designed for developers and power users who prefer the command line
3. **Open and Portable**: Your data is stored in a standard SQLite database that you fully control
4. **Extensible**: The Lisp-based transformation system allows for unlimited customization

## Getting Started

If you're new to kakei, we recommend:

1. Start with the [Installation](./installation.md) guide to get kakei installed on your system
2. Follow the [Quick Start](./quick-start.md) guide to initialize your database and add your first transactions
3. Explore the [Commands](./commands.md) reference to learn all available commands
4. Learn about [Lisp Functions](./lisp-functions.md) to unlock powerful data transformations

## Contributing

kakei is open source and welcomes contributions! See the [Contributing](./contributing.md) guide for more information.

## License

kakei is licensed under the MIT License. See the repository's LICENSE file for details.

## Author

haruki7049 <tontonkirikiri@gmail.com>

## Repository

<https://github.com/haruki7049/kakei>
