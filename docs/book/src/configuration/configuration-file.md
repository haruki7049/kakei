# Configuration File

kakei uses a TOML configuration file to customize default categories and accounts.

### Location

The configuration file location depends on your operating system:

- **Linux (XDG)**: `~/.config/kakei/config.toml` (or `$XDG_CONFIG_HOME/kakei/config.toml`)
- **macOS**: `~/Library/Application Support/kakei/config.toml`
- **Windows**: `%APPDATA%\kakei\config.toml`

### Format

The configuration file uses [TOML](https://toml.io/) format:

```toml
default_categories = ["Food", "Transport", "Daily Goods", "Hobby", "Salary"]
default_accounts = ["Cash", "Bank"]
```

### Creating a Configuration File

kakei automatically creates a default configuration when you run `kakei init`. If you want to customize it:

1. **Locate the config file** (see locations above)

1. **Edit with your favorite text editor**:

   ```bash
   # Linux/macOS
   nano ~/.config/kakei/config.toml

   # Or use any editor you prefer
   vim ~/.config/kakei/config.toml
   code ~/.config/kakei/config.toml
   ```

1. **Add your custom categories and accounts**

### Example Configurations

#### Personal Finance Tracking

```toml
default_categories = [
    "Food",
    "Transport",
    "Housing",
    "Utilities",
    "Healthcare",
    "Entertainment",
    "Shopping",
    "Salary",
    "Freelance",
    "Investment"
]

default_accounts = [
    "Cash",
    "Bank",
    "Credit Card",
    "Savings",
    "Investment Account"
]
```

#### Business Expense Tracking

```toml
default_categories = [
    "Office Supplies",
    "Software",
    "Travel",
    "Meals",
    "Marketing",
    "Payroll",
    "Revenue",
    "Consulting"
]

default_accounts = [
    "Business Checking",
    "Business Savings",
    "Business Credit Card",
    "Petty Cash"
]
```

#### Minimal Setup

```toml
default_categories = ["Expense", "Income"]
default_accounts = ["Main"]
```
