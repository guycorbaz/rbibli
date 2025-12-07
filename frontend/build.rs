//! Build script for compiling Slint UI files.
//!
//! This build script is executed during the Cargo build process before the main
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
    // Compile translations
    let lang_dir = std::path::Path::new("lang");
    if lang_dir.exists() {
        let po_path = lang_dir.join("fr.po");
        
        // Create correct directory structure for gettext: lang/fr/LC_MESSAGES/frontend.mo
        // Note: Slint uses the package name (not lib name) as the translation domain
        let out_dir = lang_dir.join("fr").join("LC_MESSAGES");
        std::fs::create_dir_all(&out_dir).unwrap();
        let mo_path = out_dir.join("frontend.mo");

        if po_path.exists() {
            println!("cargo:rerun-if-changed={}", po_path.display());

            // Copy .po file to LC_MESSAGES directory for Slint bundled translations
            let po_dest = out_dir.join("frontend.po");
            std::fs::copy(&po_path, &po_dest).unwrap();
            println!("Copied .po file for Slint: {}", po_dest.display());

            // Compile .mo for gettextrs
            let status = std::process::Command::new("msgfmt")
                .arg("-o")
                .arg(&mo_path)
                .arg(&po_path)
                .status();

            match status {
                Ok(s) if s.success() => println!("Compiled .mo file: {}", mo_path.display()),
                Ok(s) => println!("cargo:warning=msgfmt failed with status: {}", s),
                Err(_) => println!("cargo:warning=msgfmt not found. Install gettext to compile translations."),
            }
        }
    }
    
    // Compile German translations
    let lang_dir = std::path::Path::new("lang");
    if lang_dir.exists() {
        let po_path = lang_dir.join("de.po");
        
        // Create correct directory structure for gettext: lang/de/LC_MESSAGES/frontend.mo
        // Note: Slint uses the package name (not lib name) as the translation domain
        let out_dir = lang_dir.join("de").join("LC_MESSAGES");
        std::fs::create_dir_all(&out_dir).unwrap();
        let mo_path = out_dir.join("frontend.mo");
        let mo_path_rbibli = out_dir.join("rbibli.mo");

        if po_path.exists() {
            println!("cargo:rerun-if-changed={}", po_path.display());

            // Copy .po file to LC_MESSAGES directory for Slint bundled translations
            let po_dest = out_dir.join("frontend.po");
            std::fs::copy(&po_path, &po_dest).unwrap();
            println!("Copied .po file for Slint: {}", po_dest.display());

            // Compile .mo for gettextrs
            let status = std::process::Command::new("msgfmt")
                .arg("-o")
                .arg(&mo_path)
                .arg(&po_path)
                .status();

             // Also compile rbibli.mo just in case
            let _ = std::process::Command::new("msgfmt")
                .arg("-o")
                .arg(&mo_path_rbibli)
                .arg(&po_path)
                .status();

            match status {
                Ok(s) if s.success() => println!("Compiled .mo file (de): {}", mo_path.display()),
                Ok(s) => println!("cargo:warning=msgfmt failed with status: {}", s),
                Err(_) => println!("cargo:warning=msgfmt not found. Install gettext to compile translations."),
            }
        }
    }

    slint_build::compile_with_config(
        "ui/app-window.slint",
        slint_build::CompilerConfiguration::new()
            .with_bundled_translations(std::path::Path::new("lang")),
    ).expect("Slint build failed");
}
