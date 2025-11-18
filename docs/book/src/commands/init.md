# init

Initialize the database and configuration.

### Usage

```bash
kakei init
```

### Description

The `init` command sets up kakei for first-time use:

1. **Creates the database file** at the platform-appropriate location:

   - Linux (XDG): `~/.local/share/kakei/kakei.db`
   - macOS: `~/Library/Application Support/kakei/kakei.db`
   - Windows: `%APPDATA%\kakei\kakei.db`

1. **Runs database migrations** to create the necessary tables and schema

1. **Initializes default categories**:

   - Food
   - Transport
   - Daily Goods
   - Hobby
   - Salary

1. **Initializes default accounts**:

   - Cash
   - Bank

### When to Use

- Run this command **once** when you first install kakei
- If you delete your database and want to start fresh
- After moving kakei to a new system

### Example

```bash
$ kakei init
Database initialized successfully!
```
