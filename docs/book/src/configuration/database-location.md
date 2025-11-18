# Database Location

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

1. **Run kakei commands** - they will now use the custom location

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
