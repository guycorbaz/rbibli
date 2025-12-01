# SQLx CLI Installation Guide

This guide covers installing `sqlx-cli`, the command-line tool for managing database migrations with SQLx in the rbibli project.

## What is sqlx-cli?

`sqlx-cli` is a command-line utility for SQLx that provides:
- Database migration management (create, run, revert)
- Database creation and deletion
- Migration script generation
- Offline query verification

## Prerequisites

Before installing sqlx-cli, ensure you have:

1. **Rust toolchain** (1.91.0 or later)
   ```bash
   rustc --version
   cargo --version
   ```

2. **MariaDB client libraries** (required for compilation)

### Installing MariaDB Client Libraries

#### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install libmariadb-dev libmariadb-dev-compat
```

#### Linux (Fedora/RHEL/CentOS)
```bash
sudo dnf install mariadb-devel
```

#### macOS
```bash
brew install mariadb-connector-c
```

#### Windows
- Download and install [MariaDB Connector/C](https://mariadb.com/downloads/connectors/connectors-data-access/c-connector/)
- Or install the full MariaDB server which includes the client libraries

## Installation

### Standard Installation (Recommended)

Install sqlx-cli with **only** MySQL/MariaDB support (faster compilation):

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

**Why `--no-default-features`?**
- By default, sqlx-cli includes support for PostgreSQL, MySQL, and SQLite
- This requires multiple database client libraries
- Using `--no-default-features --features mysql` installs only MySQL/MariaDB support
- Results in faster compilation and fewer dependencies

### Alternative: Install with Multiple Database Support

If you work with multiple databases:

```bash
# PostgreSQL and MySQL
cargo install sqlx-cli --features postgres,mysql

# All databases
cargo install sqlx-cli
```

### Installation from Specific Version

```bash
cargo install sqlx-cli --version 0.7.4 --no-default-features --features mysql
```

### Update Existing Installation

```bash
cargo install sqlx-cli --no-default-features --features mysql --force
```

## Verify Installation

After installation, verify sqlx-cli is available:

```bash
sqlx --version
```

You should see output like:
```
sqlx-cli 0.7.x
```

Check available commands:

```bash
sqlx --help
```

## Common Installation Issues

### Issue 1: "error: linker `cc` not found"

**Solution**: Install build essentials

```bash
# Debian/Ubuntu
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# macOS
xcode-select --install
```

### Issue 2: "failed to run custom build command for `mysqlclient-sys`"

**Cause**: Missing MariaDB/MySQL client libraries

**Solution**: Install development libraries (see Prerequisites section above)

### Issue 3: "could not find system library 'mariadb' required by the 'mysqlclient-sys' crate"

**Solution (Linux)**:
```bash
# Debian/Ubuntu
sudo apt install libmariadb-dev pkg-config

# Fedora/RHEL
sudo dnf install mariadb-devel
```

**Solution (macOS)**:
```bash
brew install mariadb-connector-c pkg-config

# If still failing, set library path:
export PATH="/usr/local/opt/mariadb-connector-c/bin:$PATH"
export PKG_CONFIG_PATH="/usr/local/opt/mariadb-connector-c/lib/pkgconfig"
```

**Solution (Windows)**:
- Ensure MariaDB Connector/C is installed
- Add the library path to your system PATH
- You may need to set environment variables:
  ```cmd
  set MYSQLCLIENT_LIB_DIR=C:\Program Files\MariaDB\MariaDB Connector C\lib
  ```

### Issue 4: Installation Takes Too Long

**Solution**: Use pre-built binaries (if available) or ensure you're not building all database backends:

```bash
# Correct - only MySQL
cargo install sqlx-cli --no-default-features --features mysql

# Incorrect - builds all backends (slow)
cargo install sqlx-cli
```

### Issue 5: "error: failed to compile `sqlx-cli`"

**Solutions**:
1. Update Rust: `rustup update`
2. Clear cargo cache: `cargo clean`
3. Try installing an earlier version:
   ```bash
   cargo install sqlx-cli --version 0.7.3 --no-default-features --features mysql
   ```

## Using sqlx-cli

Once installed, you can use sqlx-cli from anywhere in your project:

### Set Database URL

Create or edit `.env` file in the backend directory:

```bash
DATABASE_URL=mysql://rbibli_user:password@localhost/rbibli
```

Or export as environment variable:

```bash
export DATABASE_URL="mysql://rbibli_user:password@localhost/rbibli"
```

### Common Commands

```bash
# Create a new migration
sqlx migrate add create_users_table

# Run all pending migrations
sqlx migrate run

# Revert the last migration
sqlx migrate revert

# Show migration status
sqlx migrate info

# Create database (if it doesn't exist)
sqlx database create

# Drop database (careful!)
sqlx database drop

# Reset database (drop + create + migrate)
sqlx database reset
```

### Migration Workflow

1. **Create migration**:
   ```bash
   cd backend
   sqlx migrate add create_titles_table
   ```
   This creates two files:
   - `migrations/TIMESTAMP_create_titles_table.up.sql` (apply)
   - `migrations/TIMESTAMP_create_titles_table.down.sql` (revert)

2. **Write SQL** in the `.up.sql` and `.down.sql` files

3. **Apply migration**:
   ```bash
   sqlx migrate run
   ```

4. **Revert if needed**:
   ```bash
   sqlx migrate revert
   ```

## Project Setup with sqlx-cli

For the rbibli project:

```bash
# 1. Navigate to backend directory
cd backend

# 2. Ensure .env file exists with DATABASE_URL
cat .env

# 3. Create database
sqlx database create

# 4. Run migrations
sqlx migrate run

# 5. Verify migrations
sqlx migrate info
```

## Offline Mode (Advanced)

SQLx can verify queries at compile time without database connection:

```bash
# Prepare offline query data
cargo sqlx prepare

# This creates sqlx-data.json for offline compilation
```

## Uninstalling

To remove sqlx-cli:

```bash
cargo uninstall sqlx-cli
```

## Additional Resources

- [SQLx GitHub Repository](https://github.com/launchbadge/sqlx)
- [SQLx CLI Documentation](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
- [SQLx Book](https://docs.rs/sqlx/latest/sqlx/)

## Quick Reference Card

```bash
# Installation
cargo install sqlx-cli --no-default-features --features mysql

# Setup
export DATABASE_URL="mysql://user:pass@localhost/dbname"

# Database operations
sqlx database create        # Create database
sqlx database drop          # Drop database
sqlx database reset         # Drop, create, and migrate

# Migration operations
sqlx migrate add <name>     # Create new migration
sqlx migrate run            # Run pending migrations
sqlx migrate revert         # Revert last migration
sqlx migrate info           # Show migration status

# Query verification
cargo sqlx prepare          # Generate offline query metadata
```

## For rbibli Project

Complete setup commands for rbibli:

```bash
# 1. Install sqlx-cli
cargo install sqlx-cli --no-default-features --features mysql

# 2. Navigate to backend
cd backend

# 3. Configure database URL in .env
echo 'DATABASE_URL=mysql://rbibli_user:your_password@localhost/rbibli' > .env

# 4. Create database
sqlx database create

# 5. Run migrations
sqlx migrate run

# 6. Verify
sqlx migrate info

# 7. Create runtime configuration (required for running the app)
echo '[application]
port = 8000
host = "127.0.0.1"

[database]
username = "rbibli_user"
password = "your_password"
port = 3306
host = "127.0.0.1"
database_name = "rbibli"' > configuration.toml

# 8. Test backend
cargo run
```

Now you're ready to use sqlx-cli for database management in the rbibli project!
