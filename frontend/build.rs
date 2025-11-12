/// Build script for compiling Slint UI files.
///
/// This build script is executed during the Cargo build process before the main
/// application is compiled. It uses the `slint_build` crate to compile the Slint
/// UI definition files (`.slint`) into Rust code that can be included in the
/// main application.
///
/// # Process
///
/// 1. Locates the main Slint UI file at `ui/app-window.slint`
/// 2. Compiles the Slint UI definition into generated Rust code
/// 3. The generated code is automatically included in the main application via
///    the `slint::include_modules!()` macro
///
/// # Files Compiled
///
/// * `ui/app-window.slint` - Main application window definition
///   - This file may import other `.slint` files from the `ui/` directory
///   - All imported files are automatically tracked for rebuild detection
///
/// # Panics
///
/// This function will panic if:
/// - The Slint UI file cannot be found at `ui/app-window.slint`
/// - The Slint UI file contains syntax errors or invalid definitions
/// - The Slint compiler encounters an internal error
///
/// When a panic occurs during the build process, Cargo will display the error
/// message and halt the build.
///
/// # Automatic Rebuild
///
/// Cargo will automatically re-run this build script when:
/// - Any `.slint` file in the `ui/` directory is modified
/// - The build script itself is modified
/// - Dependencies in `Cargo.toml` are updated
fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
