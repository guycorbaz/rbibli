# Translation Guide for rbibli Frontend

This document explains how to manage translations for the rbibli frontend application.

## Translation System Overview

The frontend uses **Slint's bundled translations** system with gettext `.po` files:

- **UI strings**: Marked with `@tr("text")` in `.slint` files
- **Translation files**: `.po` files in `lang/` directory
- **Compilation**: `.po` files are compiled to `.mo` files during build
- **Runtime**: Translations are embedded in the WASM/native binary

## Directory Structure

```
frontend/
├── lang/
│   ├── fr.po                           # French translations (source)
│   ├── de.po                           # German translations (source)
│   ├── rbibli.pot                      # Template file (generated)
│   └── fr/
│       └── LC_MESSAGES/
│           ├── frontend.po             # Compiled during build
│           └── frontend.mo             # Compiled during build
├── ui/
│   ├── app-window.slint               # Main UI file
│   └── pages/                         # Page components
├── build.rs                           # Build script (compiles translations)
└── update_translations.sh             # Script to extract and update translations
```

## Translation Workflow

### 1. Extract Strings & Update PO Files

The project uses `slint-tr-extractor` to extract strings from `.slint` files. A convenience script is provided:

```bash
cd frontend
./update_translations.sh
```

This script will:

1. Run `slint-tr-extractor` to generate `lang/rbibli.pot`
2. Run `msgmerge` (if installed) to update `fr.po` and `de.po` with new strings
3. Mark changed strings as "fuzzy" for review

### 2. Translate with Poedit

**Install Poedit:**

```bash
sudo apt install poedit  # Ubuntu/Debian
```

**Open and translate:**

```bash
poedit lang/fr.po
```

In Poedit:

1. **Filter for Fuzzy/Untranslated**: Look for entries marked in orange (fuzzy) or bold (untranslated).
2. **Review**: Confirm if the fuzzy matching is correct. Remove the "Fuzzy" flag (Ctrl+U) if it is.
3. **Translate**: Enter missing translations.
4. **Save**: (Ctrl+S) - automatically compiles to `.mo`

**Check translation status:**

```bash
msgfmt --statistics lang/fr.po
```

### 3. Build and Test

**For WASM (with trunk):**

```bash
cargo clean -p frontend
trunk build --release
trunk serve --release
```

**For native:**

```bash
cargo clean -p frontend
cargo build --release
./target/release/frontend
```

## Adding a New Language

### Example: Adding Spanish (es)

1. **Create translation file from template:**

```bash
msginit -i lang/rbibli.pot -o lang/es.po -l es
# OR copy an existing one:
cp lang/fr.po lang/es.po
```

2. **Translate strings in Poedit:**

```bash
poedit lang/es.po
```

3. **Add to build.rs:**
(See existing `build.rs` for example of how to add a new language block)

4. **Add language detection in main.rs:**
(Update the locale detection logic)

## Troubleshooting

### Translations not appearing

- Check `msgfmt --statistics lang/fr.po`.
- Ensure you have un-marked "fuzzy" translations. Fuzzy translations are NOT compiled into the final binary.

### Fuzzy translations

When the UI changes or context is added, `msgmerge` tries to guess the match and marks it "fuzzy".
You **MUST** review these and uncheck the "Needs work" (fuzzy) flag in Poedit for them to appear in the app.

## Maintenance

### When adding new UI strings

1. Add `@tr("New text")` in `.slint` file
2. Run `./update_translations.sh`
3. Update `.po` files in Poedit
4. Rebuild
