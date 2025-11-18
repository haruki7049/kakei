# Architecture

Understanding kakei's internal architecture helps developers contribute effectively and users understand how the application works.

## Project Structure

kakei is organized as a Rust workspace with multiple crates:

```
kakei/
├── src/                      # Main CLI application
├── crates/
│   └── processor/
│       ├── src/              # Business logic and transformations
│       └── crates/
│           ├── database/     # Database layer (SQLite)
│           ├── money/        # Money type with currency support
│           └── klisp/        # Embedded Lisp dialect
├── tests/                    # Integration tests
├── book/                     # Documentation (mdBook)
└── Cargo.toml               # Workspace configuration
```

## Crate Overview

### kakei (Main Crate)

**Location:** `src/`  
**Purpose:** Command-line interface and user interaction

**Responsibilities:**
- Parse command-line arguments using `clap`
- Handle user input and output
- Coordinate between other crates
- Format output tables using `tabled`
- Manage configuration with `confy`

**Key Files:**
- `src/main.rs`: Application entry point
- `src/commands/`: Command implementations (init, add, list, transform)

### kakei_processor

**Location:** `crates/processor/`  
**Purpose:** Business logic and table transformations

**Responsibilities:**
- Transaction management logic
- Table transformation orchestration
- Integration between database and Lisp interpreter
- Data conversion between formats

**Dependencies:**
- `kakei_database`: For data persistence
- `kakei_lisp`: For Lisp evaluation
- `kakei_money`: For money types

### kakei_database

**Location:** `crates/processor/crates/database/`  
**Purpose:** Database layer with SQLite

**Responsibilities:**
- SQLite database connection management
- Transaction CRUD operations
- Database migrations
- Query execution

**Key Technologies:**
- `sqlx`: Async SQLite driver with compile-time query verification
- `chrono`: Date/time handling

**Schema:**
```sql
CREATE TABLE transactions (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL,
    amount INTEGER NOT NULL,
    category TEXT NOT NULL,
    account TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'JPY',
    memo TEXT
);
```

### kakei_money

**Location:** `crates/processor/crates/money/`  
**Purpose:** Money type with currency support

**Responsibilities:**
- Type-safe money representation
- Currency handling (JPY, USD, EUR, etc.)
- Amount formatting with proper symbols
- Minor unit conversion

**Key Types:**
```rust
pub struct Money {
    amount: i64,        // Amount in minor units
    currency: Currency, // Currency type
}

pub enum Currency {
    JPY,
    USD,
    EUR,
    GBP,
    // ...
}
```

**Features:**
- Amounts stored in minor units (cents, yen, etc.)
- Currency-specific formatting
- Decimal conversion utilities

### kakei_lisp

**Location:** `crates/processor/crates/klisp/`  
**Purpose:** Embedded Lisp dialect for data transformation

**Responsibilities:**
- Lisp parsing (using `nom` parser combinator)
- Expression evaluation
- Built-in function implementations
- Runtime environment

**Language Features:**
- S-expressions: `(func arg1 arg2)`
- Lambda functions: `(lambda (x) (+ x 1))`
- Core functions: `cons`, `car`, `cdr`, `if`, `define`
- Association lists: `assoc`
- Table manipulation: `group-by`

**Architecture:**
```
Input String
    ↓
Parser (nom)
    ↓
AST (Abstract Syntax Tree)
    ↓
Evaluator
    ↓
Result Value
```

## Data Flow

### Adding a Transaction

```
User Command
    ↓
CLI (kakei)
    ↓
Parse Arguments (clap)
    ↓
Create Transaction Object
    ↓
Processor (kakei_processor)
    ↓
Database Layer (kakei_database)
    ↓
SQLite Database
```

### Listing Transactions

```
User Command
    ↓
CLI (kakei)
    ↓
Processor (kakei_processor)
    ↓
Database Layer (kakei_database)
    ↓
Query SQLite
    ↓
Convert to Money Types (kakei_money)
    ↓
Format as Table (tabled)
    ↓
Display to User
```

### Transform Command

```
User Command with Lisp Program
    ↓
CLI (kakei)
    ↓
Processor (kakei_processor)
    ↓
Database: Load Transactions
    ↓
Convert to Lisp Data Structure
    ↓
Lisp Interpreter (kakei_lisp)
    ↓
Parse Lisp Program
    ↓
Evaluate Against Transaction Data
    ↓
Convert Result Back
    ↓
Format as Table
    ↓
Display to User
```

## Key Technologies

### Rust Edition

- **Rust 2024 Edition** (Edition 2024)
- **Minimum Rust Version:** 1.91.1

### Major Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.4.11 | Command-line argument parsing |
| `sqlx` | 0.8.6 | Async SQLite with compile-time checks |
| `tokio` | 1.48.0 | Async runtime |
| `chrono` | 0.4.42 | Date and time handling |
| `tabled` | 0.20.0 | Table formatting |
| `nom` | 8.0.0 | Parser combinators for Lisp |
| `serde` | 1.0.219 | Serialization/deserialization |
| `confy` | 0.6.1 | Configuration management |
| `directories` | 6.0.0 | Platform-appropriate directories |
| `xdg` | 3.0.0 | XDG Base Directory support |

### Testing

- `assert_cmd`: CLI testing
- `predicates`: Assertion helpers
- `tempfile`: Temporary files for tests

### Nix Integration

kakei includes Nix flakes support for reproducible builds:

- `flake.nix`: Flake definition
- `flake.lock`: Locked dependencies
- `default.nix`: Traditional Nix build
- `shell.nix`: Development shell

## Design Patterns

### Workspace Organization

The workspace structure allows:
- **Independent testing** of each component
- **Clear separation of concerns**
- **Reusable components** (e.g., `kakei_money` could be used elsewhere)
- **Faster incremental builds**

### Error Handling

kakei uses `thiserror` for custom error types:

```rust
#[derive(Error, Debug)]
pub enum KakeiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    // ...
}
```

### Async/Await

Database operations are async using `tokio`:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Async operations
}
```

### Type Safety

Strong typing throughout:
- `Money` type ensures currency safety
- `Transaction` structs validate data
- Compile-time SQL query verification

## Configuration Management

### Directory Resolution

```
Check XDG Environment Variables
    ↓ (if not set)
Platform-Specific Defaults
    ↓
Create Directory if Missing
    ↓
Load/Create Config File
```

### Configuration Flow

```
confy crate
    ↓
Load config.toml
    ↓
Parse with TOML parser
    ↓
Deserialize to Config struct
    ↓
Use in Application
```

## Database Management

### Migrations

Migrations are handled at application startup:

1. Check if database exists
2. Create if missing
3. Run pending migrations
4. Initialize default data (categories, accounts)

### Connection Pooling

SQLx provides connection pooling for efficient database access:

```rust
let pool = SqlitePool::connect(&database_url).await?;
```

## Performance Considerations

### Query Optimization

- Indexes on frequently queried columns
- Limit clauses for list operations (default: 20 transactions)
- Efficient date-based queries

### Memory Usage

- Streaming results for large datasets
- Lazy evaluation in Lisp interpreter
- Efficient cons cell representation

### Build Optimization

- Release builds with optimizations enabled
- Incremental compilation in workspace
- Cargo cache for dependencies

## Security Considerations

### Data Safety

- SQLite ACID properties ensure data integrity
- Transactions for atomic operations
- Regular backups recommended

### Input Validation

- Command-line argument validation via `clap`
- SQL injection prevention via parameterized queries (sqlx)
- Type-safe APIs throughout

### File Permissions

- Configuration and database files are user-readable/writable only
- No sensitive data in configuration (categories/accounts only)

## Future Architecture Considerations

Potential areas for expansion:

1. **Plugin System**: Allow custom Lisp functions via plugins
2. **Multiple Databases**: Support for multiple financial ledgers
3. **Import/Export**: CSV/JSON import/export capabilities
4. **Web Interface**: Optional web UI using the same core crates
5. **Sync**: Cloud sync or multi-device support
6. **More Lisp Functions**: Additional built-ins for calculations

## Contributing to Architecture

When contributing, consider:

1. **Maintain separation of concerns** - keep crates focused
2. **Add tests** - especially for new Lisp functions
3. **Document public APIs** - use Rust doc comments
4. **Follow existing patterns** - match the codebase style
5. **Consider performance** - especially for database operations

## See Also

- [Development](./development.md) - Building and testing
- [Contributing](./contributing.md) - Contribution guidelines
- [Repository](https://github.com/haruki7049/kakei) - Source code
