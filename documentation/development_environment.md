# Development environment

To develop rbibli, certain tools need to be set up.

## Rust

We assume that Rust is already installed. The project uses:
- **Rust**: 1.91.0 or later (stable channel)
- **Cargo**: 1.91.0 or later

To check your Rust version:
```bash
rustc --version
cargo --version
```

To update Rust:
```bash
rustup update
```

## WebAssembly Tooling

Since the frontend is compiled to WebAssembly, you need WASM tools:

```bash
# Add WASM target to Rust
rustup target add wasm32-unknown-unknown

# Install wasm-pack (WASM build tool)
cargo install wasm-pack

# Install wasm-bindgen-cli (optional, for advanced use)
cargo install wasm-bindgen-cli
```

## Slint

Slint is the UI framework used for the web interface, compiled to WebAssembly. Slint is included as a Cargo dependency, so no separate installation is required.

### Slint Components

The project uses two main Slint crates:
- `slint = "1.14.1"` - Runtime library for Slint UI (WASM target)
- `slint-build = "1.14.1"` - Build-time compilation of `.slint` files to WASM
- `wasm-bindgen = "*"` - JavaScript/WASM interop layer

These are automatically installed when you run `cargo build` or `wasm-pack build`.

### Slint Files

UI components are defined in `.slint` files located in the `ui/` directory. These files use Slint's declarative language and are compiled to WebAssembly at build time via the `build.rs` script and `wasm-pack`.

### Slint Documentation

For more information about Slint:
- [Slint Documentation](https://slint.dev/docs)
- [Slint Rust API](https://slint.dev/docs/rust/)
- [Slint Language Reference](https://slint.dev/docs/slint/)
- [Slint WASM Guide](https://slint.dev/docs/rust/slint/wasm_interpreter/)

## IDE Setup

### VS Code (Recommended)

For VS Code, install the following extensions:
- **rust-analyzer**: Rust language support
- **Slint**: Syntax highlighting and preview for `.slint` files

The Slint extension provides:
- Syntax highlighting for `.slint` files
- Live preview of UI components
- Code completion and validation

### Other IDEs

For other editors, you can use:
- rust-analyzer for Rust support (available for many editors)
- Basic syntax highlighting for `.slint` files

## Building and Running

### Frontend (Web Application)

```bash
# Navigate to the frontend directory
cd frontend

# Build for web (development mode)
wasm-pack build --target web --dev

# Build for web (production/release mode)
wasm-pack build --target web --release

# The build output will be in pkg/ directory
```

### Serving the Web Application

After building, you need a web server to serve the application:

```bash
# Option 1: Using Python's built-in HTTP server
python3 -m http.server 8080

# Option 2: Using miniserve (better for development)
cargo install miniserve
miniserve . --index index.html -p 8080

# Option 3: Using basic-http-server
cargo install basic-http-server
basic-http-server .
```

Then open your browser to `http://localhost:8080`

### Backend (API Server)

In a separate terminal:

```bash
# Navigate to backend directory
cd backend

# Run the API server
cargo run

# Run in release mode
cargo run --release
```

The backend will run on `http://localhost:8000` by default.

### Development with Auto-Rebuild

For frontend auto-rebuild on file changes:

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch and rebuild on changes
cd frontend
cargo watch -s "wasm-pack build --target web --dev"
```

For backend auto-rebuild:

```bash
cd backend
cargo watch -x run
```

## Code Quality Tools

### Clippy (Linting)

```bash
cargo clippy
```

### Rustfmt (Formatting)

```bash
# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## Platform-Specific Notes

### Windows

The project includes a special configuration for Windows in `.cargo/config.toml` to increase the stack size. For WASM development, no additional tools are needed beyond the standard Rust toolchain.

### Linux

Make sure you have standard development tools installed:
```bash
# Debian/Ubuntu
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"
```

### macOS

Xcode Command Line Tools should be installed:
```bash
xcode-select --install
```

### Browser Requirements

The web application requires a modern browser with WebAssembly support:
- **Chrome/Chromium**: 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Edge**: 16+

For best development experience, use the latest version of Chrome or Firefox with developer tools.

## Database Setup - MariaDB

The project uses **MariaDB** for data persistence.

### Installing MariaDB

#### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install mariadb-server mariadb-client
sudo systemctl start mariadb
sudo systemctl enable mariadb

# Secure installation
sudo mysql_secure_installation
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install mariadb-server
sudo systemctl start mariadb
sudo systemctl enable mariadb
sudo mysql_secure_installation
```

#### macOS
```bash
brew install mariadb
brew services start mariadb
mysql_secure_installation
```

#### Windows
- Download MariaDB installer from https://mariadb.org/download/
- Run installer and follow setup wizard
- Start MariaDB service from Services panel

### Database Configuration

Create database and user:

```sql
-- Connect as root
sudo mysql

-- Create database
CREATE DATABASE rbibli CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Create user (replace password)
CREATE USER 'rbibli_user'@'localhost' IDENTIFIED BY 'your_secure_password';

-- Grant privileges
GRANT ALL PRIVILEGES ON rbibli.* TO 'rbibli_user'@'localhost';
FLUSH PRIVILEGES;

-- Exit
EXIT;
```

### SQLx CLI for Migrations

**For detailed installation instructions, troubleshooting, and complete usage guide, see [sqlx_installation.md](sqlx_installation.md)**

Quick install with MariaDB support:

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

Set database URL environment variable:

```bash
```bash
# Add to your .env file or export (REQUIRED for sqlx compilation)
export DATABASE_URL="mysql://rbibli_user:your_secure_password@localhost/rbibli"
```

### Application Configuration

The application uses a TOML file for runtime configuration. Create `backend/configuration.toml`:

```toml
[application]
port = 8000
host = "127.0.0.1"

[database]
username = "rbibli_user"
password = "your_secure_password"
port = 3306
host = "127.0.0.1"
database_name = "rbibli"
```

You can also specify a custom configuration file at runtime:
```bash
cargo run -- --config my_config.toml
```

### Running Migrations

```bash
# Create a new migration
sqlx migrate add create_titles_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Backend Dependencies

Add to `backend/Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "mysql", "chrono", "uuid"] }
```

### Connection in Code

```rust
use sqlx::mysql::MySqlPoolOptions;

let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
```

### Development Tools

Recommended database management tools:
- **DBeaver** (cross-platform, free)
- **MySQL Workbench** (MySQL/MariaDB specific)
- **HeidiSQL** (Windows)
- **Sequel Ace** (macOS)
- **phpMyAdmin** (web-based)

## Internationalization (Planned)

The project is designed to support French and English languages. Translation files will be located in the `lang/` directory. Slint's built-in `@tr()` macro is used for translatable strings in the UI.

## Debugging WASM

### Browser DevTools

Use browser developer tools for debugging:

```javascript
// In browser console, you can:
// 1. View console.log from Rust (via web_sys::console)
// 2. Inspect WASM memory
// 3. Use browser debugger (with source maps)
```

### Logging in Rust

Add logging to your WASM code:

```rust
use web_sys::console;

console::log_1(&"Debug message".into());
```

Or use the `console_log` crate for more convenient logging.

### Performance Profiling

Use browser performance tools to profile WASM execution:
- Chrome DevTools > Performance tab
- Firefox Developer Tools > Performance tab

## Common Issues

### WASM Build Fails

If `wasm-pack build` fails:
1. Ensure `wasm32-unknown-unknown` target is installed: `rustup target add wasm32-unknown-unknown`
2. Update wasm-pack: `cargo install wasm-pack --force`
3. Clean build artifacts: `cargo clean`

### CORS Issues

If the frontend can't connect to the backend API, you may need to configure CORS in the actix-web backend:

```rust
// In backend/src/main.rs
use actix_cors::Cors;

let cors = Cors::permissive(); // For development only
```

### Browser Cache

If changes don't appear, clear browser cache or use hard refresh:
- Chrome/Firefox: Ctrl+Shift+R (Cmd+Shift+R on Mac)
- Or open DevTools and disable cache