# Initialize Database

Before you can start tracking transactions, you need to initialize the database:

```bash
kakei init
```

This command:

- Creates the database file at the appropriate location for your platform
  - Linux (XDG): `~/.local/share/kakei/kakei.db`
  - macOS: `~/Library/Application Support/kakei/kakei.db`
  - Windows: `%APPDATA%\kakei\kakei.db`
- Runs database migrations to set up the schema
- Initializes default categories: **Food**, **Transport**, **Daily Goods**, **Hobby**, **Salary**
- Initializes default accounts: **Cash**, **Bank**

You only need to run this command once when you first start using kakei.
