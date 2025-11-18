# Platform Notes

## Linux

On Linux, `kakei` follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html):

- **Configuration**: `~/.config/kakei/` (or `$XDG_CONFIG_HOME/kakei/`)
- **Data**: `~/.local/share/kakei/` (or `$XDG_DATA_HOME/kakei/`)

## macOS

On macOS, if XDG environment variables are not set, kakei will use:

- **Configuration**: `~/Library/Application Support/kakei/`
- **Data**: `~/Library/Application Support/kakei/`

## Windows

On Windows, if XDG environment variables are not set, kakei will use:

- **Configuration**: `%APPDATA%\kakei\`
- **Data**: `%APPDATA%\kakei\`

## Verifying Installation

After installation, verify that kakei is correctly installed:

```bash
kakei --help
```

You should see the help message with available commands.

## Next Steps

Now that you have kakei installed, proceed to the [Quick Start](../quick-start.md) guide to initialize your database and start tracking your finances!
