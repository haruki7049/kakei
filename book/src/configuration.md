# Configuration

Learn how to configure kakei to match your financial tracking needs.

## Configuration File

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

2. **Edit with your favorite text editor**:
   ```bash
   # Linux/macOS
   nano ~/.config/kakei/config.toml
   
   # Or use any editor you prefer
   vim ~/.config/kakei/config.toml
   code ~/.config/kakei/config.toml
   ```

3. **Add your custom categories and accounts**

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

## Configuration Options

### default_categories

**Type:** Array of strings  
**Required:** Yes  
**Description:** List of transaction categories available when adding transactions.

Categories help organize your transactions by type (e.g., Food, Transport, Salary). Choose categories that match your tracking needs.

**Tips:**
- Include both expense categories (Food, Transport) and income categories (Salary, Freelance)
- Be specific enough to be useful, but not so detailed that categorization becomes burdensome
- You can always add transactions with categories not in this list, but these are the defaults

### default_accounts

**Type:** Array of strings  
**Required:** Yes  
**Description:** List of accounts used to track where money comes from or goes to.

Accounts represent the different financial accounts or payment methods you use (e.g., Cash, Bank, Credit Card).

**Tips:**
- Include all payment methods you regularly use
- Consider separate accounts for different bank accounts or credit cards
- Use descriptive names like "Main Bank" vs. "Savings" instead of actual bank names for privacy

## Database Location

The SQLite database file stores all your transaction data.

### Default Location

- **Linux (XDG)**: `~/.local/share/kakei/kakei.db` (or `$XDG_DATA_HOME/kakei/kakei.db`)
- **macOS**: `~/Library/Application Support/kakei/kakei.db`
- **Windows**: `%APPDATA%\kakei\kakei.db`

### Customizing Database Location

Currently, kakei automatically determines the database location based on your operating system's conventions. To use a different location:

1. **Set the XDG_DATA_HOME environment variable** (Linux):
   ```bash
   export XDG_DATA_HOME=/path/to/custom/location
   ```

2. **Run kakei commands** - they will now use the custom location

### Backing Up Your Database

Since your financial data is stored in a single SQLite file, backing up is simple:

```bash
# Linux/macOS
cp ~/.local/share/kakei/kakei.db ~/backup/kakei-backup-$(date +%Y%m%d).db

# Or create a scheduled backup script
#!/bin/bash
BACKUP_DIR=~/kakei-backups
mkdir -p "$BACKUP_DIR"
cp ~/.local/share/kakei/kakei.db "$BACKUP_DIR/kakei-$(date +%Y%m%d-%H%M%S).db"
```

### Database Schema

The database is a standard SQLite database. You can inspect it directly using SQLite tools:

```bash
sqlite3 ~/.local/share/kakei/kakei.db

# Inside sqlite3:
.tables          # List tables
.schema          # Show schema
SELECT * FROM transactions LIMIT 5;  # Query transactions
```

**Warning:** Directly modifying the database outside of kakei commands may cause data corruption. Always use kakei commands for data manipulation, and only use direct SQL for read-only queries.

## XDG Base Directory Specification

On Linux, kakei follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html), which defines standard locations for user files:

| XDG Variable | Default | Purpose | Kakei Usage |
|-------------|---------|---------|-------------|
| `XDG_CONFIG_HOME` | `~/.config` | Configuration files | `kakei/config.toml` |
| `XDG_DATA_HOME` | `~/.local/share` | Data files | `kakei/kakei.db` |

### Customizing XDG Directories

You can customize these locations by setting environment variables:

```bash
# In your ~/.bashrc or ~/.zshrc
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"
```

Or set them for a single command:

```bash
XDG_DATA_HOME=/custom/path kakei list
```

## Environment Variables

### Supported Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `XDG_CONFIG_HOME` | Configuration directory (Linux) | `~/.config` |
| `XDG_DATA_HOME` | Data directory (Linux) | `~/.local/share` |

### Platform-Specific Behavior

**Linux:**
- Respects XDG environment variables
- Falls back to standard XDG defaults if not set

**macOS:**
- Uses `~/Library/Application Support` by default
- Can optionally respect XDG variables if set

**Windows:**
- Uses `%APPDATA%` directory
- Does not use XDG variables

## Configuration Best Practices

1. **Start with defaults**: The default categories and accounts are a good starting point

2. **Evolve over time**: Add categories as you identify new tracking needs

3. **Keep it simple**: Too many categories can make data entry tedious

4. **Regular backups**: Back up your database regularly to prevent data loss

5. **Version control**: Consider keeping your config.toml in version control (without sensitive data)

6. **Document your categories**: If using custom categories, document what each represents

## Troubleshooting

### Configuration not loading

**Problem:** Changes to `config.toml` don't seem to take effect.

**Solution:** 
- Verify the file is in the correct location for your OS
- Check file permissions (should be readable by your user)
- Verify TOML syntax is correct (no typos, proper quotes)

### Database not found

**Problem:** kakei reports it can't find the database.

**Solution:**
- Run `kakei init` to create the database
- Check XDG environment variables if on Linux
- Verify permissions on the data directory

### Cannot create configuration directory

**Problem:** Error creating config directory.

**Solution:**
- Check permissions on parent directories
- Ensure disk space is available
- On Linux, verify XDG_CONFIG_HOME is set correctly

## See Also

- [Installation](./installation.md) - Installing kakei
- [Quick Start](./quick-start.md#initialize-database) - Initializing the database
- [Commands](./commands.md#init) - The init command
