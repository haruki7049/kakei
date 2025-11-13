# kakei

A kakeibo cli application.

## Configuration

kakei uses a configuration file located at `~/.config/kakei/config.toml` to customize your expense tracking.

### Configuration Structure

The configuration file supports the following options:

```toml
# Expense categories - Categories for money going out
expense_categories = ["Food", "Transport", "Daily Goods", "Hobby"]

# Income categories - Categories for money coming in
income_categories = ["Salary", "Bonus"]

# Default accounts - Where money is stored
default_accounts = ["Cash", "Bank"]
```

See `config.toml.example` for a more complete example with additional categories.

### Customizing Categories

You can freely define your own expense and income categories in the configuration file. Categories are automatically assigned the correct type (expense or income) based on which list they appear in, eliminating the need for hardcoded category names.

## Usage

### Initialize the database

```bash
kakei init
```

This command initializes the database with the categories and accounts defined in your configuration file.

### Add a transaction

```bash
# Add an expense
kakei add --date 2025-01-15 --amount -1500 --category Food --account Cash --memo "Lunch"

# Add income
kakei add --date 2025-01-01 --amount 300000 --category Salary --account Bank --memo "Monthly salary"
```

### List recent transactions

```bash
kakei list
```
