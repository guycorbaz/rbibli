# Dewey Decimal Classification Implementation Proposal

## Current Status

**Database Schema:** ✅ Already implemented
- `titles.dewey_code VARCHAR(20)` - Numeric classification code
- `titles.dewey_category VARCHAR(200)` - Human-readable category name
- Index on `dewey_code` for efficient queries

**Backend Models:** ✅ Fields exist in Title struct
- `dewey_code: Option<String>`
- `dewey_category: Option<String>`

**What's Missing:** UI implementation and helper functionality

---

## Solution 1: Simple Text Input (MVP - Recommended First)

**Complexity:** 🟢 Low | **Time:** 2-3 hours

### Description
Add two optional text fields to the title create/edit forms with basic validation.

### Implementation

#### Frontend (Slint UI):
```slint
// In titles_page.slint - Add to create/edit dialogs

HorizontalBox {
    Text { text: @tr("Dewey Code:"); min-width: 120px; }
    LineEdit {
        text <=> root.new-dewey-code;
        placeholder-text: @tr("e.g., 515.35");
        // Optional: input-type: number; (but decimals need special handling)
    }
}

HorizontalBox {
    Text { text: @tr("Dewey Category:"); min-width: 120px; }
    LineEdit {
        text <=> root.new-dewey-category;
        placeholder-text: @tr("e.g., Mathematics - Analysis");
    }
}
```

#### Backend Validation:
```rust
// Add to title handlers

fn validate_dewey_code(code: &str) -> Result<(), String> {
    if code.is_empty() {
        return Ok(()); // Optional field
    }

    // Basic format: XXX or XXX.XX or XXX.XXXX
    let parts: Vec<&str> = code.split('.').collect();

    if parts.is_empty() || parts.len() > 2 {
        return Err("Invalid Dewey format. Use XXX or XXX.XX".to_string());
    }

    // Main class (000-999)
    if parts[0].len() != 3 || !parts[0].chars().all(|c| c.is_numeric()) {
        return Err("Dewey code must start with 3 digits (000-999)".to_string());
    }

    let main_class: i32 = parts[0].parse().unwrap();
    if main_class > 999 {
        return Err("Dewey code must be between 000-999".to_string());
    }

    // Optional decimal part
    if parts.len() == 2 && !parts[1].chars().all(|c| c.is_numeric()) {
        return Err("Dewey decimal part must be numeric".to_string());
    }

    Ok(())
}
```

### Pros & Cons

**Pros:**
- ✅ Quick to implement
- ✅ No additional database changes
- ✅ Flexible for advanced users
- ✅ Works immediately

**Cons:**
- ❌ No user assistance
- ❌ Requires Dewey knowledge
- ❌ Manual entry prone to errors

---

## Solution 2: Main Categories Dropdown + Subcategory Input

**Complexity:** 🟡 Medium | **Time:** 1 day

### Description
Provide a dropdown with the 10 main Dewey classes, then allow manual entry of subcategories.

### Implementation

#### Create Reference Data:
```rust
// backend/src/models/dewey.rs

pub struct DeweyMainClass {
    pub code: String,      // "000", "100", ..., "900"
    pub name: String,      // "Computer Science & Information"
    pub description: String,
}

pub fn get_main_classes() -> Vec<DeweyMainClass> {
    vec![
        DeweyMainClass {
            code: "000".to_string(),
            name: "Computer Science, Information & General Works".to_string(),
            description: "Includes encyclopedias, libraries, journalism".to_string(),
        },
        DeweyMainClass {
            code: "100".to_string(),
            name: "Philosophy & Psychology".to_string(),
            description: "Includes ethics, paranormal, psychology".to_string(),
        },
        DeweyMainClass {
            code: "200".to_string(),
            name: "Religion".to_string(),
            description: "Includes theology, Bible, mythology".to_string(),
        },
        DeweyMainClass {
            code: "300".to_string(),
            name: "Social Sciences".to_string(),
            description: "Includes sociology, politics, economics, law".to_string(),
        },
        DeweyMainClass {
            code: "400".to_string(),
            name: "Language".to_string(),
            description: "Includes linguistics, dictionaries".to_string(),
        },
        DeweyMainClass {
            code: "500".to_string(),
            name: "Science".to_string(),
            description: "Includes mathematics, astronomy, physics, chemistry, biology".to_string(),
        },
        DeweyMainClass {
            code: "600".to_string(),
            name: "Technology".to_string(),
            description: "Includes medicine, engineering, agriculture, home economics".to_string(),
        },
        DeweyMainClass {
            code: "700".to_string(),
            name: "Arts & Recreation".to_string(),
            description: "Includes architecture, painting, music, sports".to_string(),
        },
        DeweyMainClass {
            code: "800".to_string(),
            name: "Literature".to_string(),
            description: "Includes poetry, drama, novels by language".to_string(),
        },
        DeweyMainClass {
            code: "900".to_string(),
            name: "History & Geography".to_string(),
            description: "Includes world history, biography, geography".to_string(),
        },
    ]
}
```

#### Frontend UI:
```slint
// Two-step selection

VerticalBox {
    spacing: 10px;

    // Step 1: Main class dropdown
    HorizontalBox {
        Text { text: @tr("Dewey Main Class:"); min-width: 120px; }
        ComboBox {
            model: [
                "(None)",
                "000 - Computer Science & Information",
                "100 - Philosophy & Psychology",
                "200 - Religion",
                "300 - Social Sciences",
                "400 - Language",
                "500 - Science",
                "600 - Technology",
                "700 - Arts & Recreation",
                "800 - Literature",
                "900 - History & Geography"
            ];
            current-index <=> root.dewey-main-class-index;
        }
    }

    // Step 2: Subcategory manual entry
    if root.dewey-main-class-index > 0: HorizontalBox {
        Text { text: @tr("Subcategory:"); min-width: 120px; }
        LineEdit {
            text <=> root.dewey-subcategory;
            placeholder-text: @tr("e.g., .35 for specific topic");
        }
        Text {
            text: "→ " + root.computed-dewey-code;
            color: #0088cc;
        }
    }
}
```

### Pros & Cons

**Pros:**
- ✅ Guides users to correct main categories
- ✅ Reduces errors for main classification
- ✅ Still flexible for subcategories
- ✅ Good balance of guidance and flexibility

**Cons:**
- ❌ Still requires knowledge for subcategories
- ❌ Main class list is static (but that's fine)

---

## Solution 3: Hierarchical Three-Tier Dropdown

**Complexity:** 🟡 Medium | **Time:** 2-3 days

### Description
Create a comprehensive dropdown system with:
1. Main class (000-900)
2. Division (010-990)
3. Section (010.1-999.9)

### Implementation

#### Database Approach:
Create a reference table for Dewey classifications (one-time data load).

```sql
-- Migration: Create Dewey reference table
CREATE TABLE dewey_classifications (
    id CHAR(36) PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,
    level INT NOT NULL, -- 1=main, 2=division, 3=section
    parent_code VARCHAR(20),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    INDEX idx_code (code),
    INDEX idx_parent (parent_code),
    INDEX idx_level (level)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Example data
INSERT INTO dewey_classifications (id, code, level, parent_code, name, description) VALUES
(UUID(), '500', 1, NULL, 'Science', 'Natural sciences and mathematics'),
(UUID(), '510', 2, '500', 'Mathematics', 'Pure mathematics'),
(UUID(), '515', 3, '510', 'Analysis', 'Mathematical analysis'),
(UUID(), '515.3', 3, '515', 'Differential equations', 'Equations involving derivatives');
```

#### Backend API:
```rust
// GET /api/v1/dewey?parent_code=510
// Returns children of the specified parent

pub async fn get_dewey_classifications(
    query: web::Query<DeweyQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let parent_code = query.parent_code.as_deref().unwrap_or("");

    let classifications = if parent_code.is_empty() {
        // Get main classes (level 1)
        sqlx::query_as::<_, DeweyClassification>(
            "SELECT * FROM dewey_classifications WHERE level = 1 ORDER BY code"
        )
        .fetch_all(&data.db_pool)
        .await
    } else {
        // Get children of parent
        sqlx::query_as::<_, DeweyClassification>(
            "SELECT * FROM dewey_classifications WHERE parent_code = ? ORDER BY code"
        )
        .bind(parent_code)
        .fetch_all(&data.db_pool)
        .await
    };

    // Return results...
}
```

#### Frontend UI (Cascading Dropdowns):
```slint
VerticalBox {
    spacing: 10px;

    // Level 1: Main class
    HorizontalBox {
        Text { text: @tr("Main Class:"); min-width: 120px; }
        ComboBox {
            model: root.dewey-main-classes;
            current-index <=> root.dewey-main-index;
            // On change, load divisions
        }
    }

    // Level 2: Division (shown when main class selected)
    if root.dewey-main-index >= 0: HorizontalBox {
        Text { text: @tr("Division:"); min-width: 120px; }
        ComboBox {
            model: root.dewey-divisions;
            current-index <=> root.dewey-division-index;
            // On change, load sections
        }
    }

    // Level 3: Section (shown when division selected)
    if root.dewey-division-index >= 0: HorizontalBox {
        Text { text: @tr("Section:"); min-width: 120px; }
        ComboBox {
            model: root.dewey-sections;
            current-index <=> root.dewey-section-index;
        }
    }

    // Show final result
    Text {
        text: "Classification: " + root.selected-dewey-code + " - " + root.selected-dewey-name;
        font-weight: 700;
        color: #0088cc;
    }
}
```

### Data Loading Script:
You'd need to populate the table with Dewey data. This can be done via:
- Manual SQL script with common categories
- Import from CSV file
- Web scraping from public Dewey databases (check licensing)

### Pros & Cons

**Pros:**
- ✅ Comprehensive guidance
- ✅ No Dewey knowledge required
- ✅ Standardized classifications
- ✅ Browse-able classification tree
- ✅ Consistent data quality

**Cons:**
- ❌ Requires reference data loading
- ❌ More complex implementation
- ❌ Database size increase (~10-20MB for full Dewey)
- ❌ Limited to classifications in database

---

## Solution 4: Autocomplete Search with Fuzzy Matching

**Complexity:** 🔴 High | **Time:** 3-5 days

### Description
Provide a search box where users can type keywords and get matching Dewey classifications.

### Implementation

#### Database:
Same as Solution 3, but add full-text search index:

```sql
-- Add full-text search
ALTER TABLE dewey_classifications
ADD FULLTEXT INDEX ft_name_desc (name, description);
```

#### Backend Search API:
```rust
// GET /api/v1/dewey/search?q=mathematics
pub async fn search_dewey(
    query: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let search_term = &query.q;

    // Full-text search
    let results = sqlx::query_as::<_, DeweyClassification>(
        "SELECT *,
         MATCH(name, description) AGAINST(? IN NATURAL LANGUAGE MODE) as relevance
         FROM dewey_classifications
         WHERE MATCH(name, description) AGAINST(? IN NATURAL LANGUAGE MODE)
         ORDER BY relevance DESC
         LIMIT 20"
    )
    .bind(search_term)
    .bind(search_term)
    .fetch_all(&data.db_pool)
    .await;

    // Return JSON...
}
```

#### Frontend Autocomplete:
```slint
VerticalBox {
    spacing: 10px;

    HorizontalBox {
        Text { text: @tr("Search Dewey:"); min-width: 120px; }
        LineEdit {
            text <=> root.dewey-search-query;
            placeholder-text: @tr("Type keyword (e.g., 'calculus', 'poetry')");
            edited => {
                // Trigger search after 300ms delay
                root.trigger-dewey-search();
            }
        }
    }

    // Search results dropdown
    if root.dewey-search-results.length > 0: ScrollView {
        height: 200px;
        VerticalBox {
            for result in root.dewey-search-results: Rectangle {
                height: 40px;
                background: touch-area.has-hover ? #e8f4f8 : transparent;

                HorizontalBox {
                    padding: 10px;
                    Text {
                        text: result.code + " - " + result.name;
                        font-size: 14px;
                    }
                }

                TouchArea {
                    clicked => {
                        root.select-dewey-result(result.code, result.name);
                    }
                }
            }
        }
    }

    // Selected classification
    if root.selected-dewey-code != "": HorizontalBox {
        Text {
            text: "Selected: " + root.selected-dewey-code + " - " + root.selected-dewey-name;
            font-weight: 700;
            color: #00aa00;
        }
        Button {
            text: "Clear";
            clicked => {
                root.clear-dewey-selection();
            }
        }
    }
}
```

### Pros & Cons

**Pros:**
- ✅ Best user experience
- ✅ No Dewey knowledge needed
- ✅ Fast and intuitive
- ✅ Discovers related categories
- ✅ Modern UI pattern

**Cons:**
- ❌ Most complex implementation
- ❌ Requires full-text search setup
- ❌ Larger reference database needed
- ❌ Requires JavaScript/async for autocomplete
- ❌ More backend API calls

---

## Recommended Implementation Path

### Phase 1: **Solution 1** (MVP - Weeks 1-2)
Start with simple text input for users who know Dewey. This unblocks the feature immediately.

**Benefits:**
- Quick win
- Optional feature doesn't block other work
- Validates user need for Dewey classification

### Phase 2: **Solution 2** (Enhanced - Month 2)
Add main class dropdown for better UX once you confirm users are using Dewey.

**Benefits:**
- Improves usability
- Still lightweight
- Good enough for most personal libraries

### Phase 3: **Solution 3 or 4** (Advanced - Month 3+)
Only if users request more sophisticated classification tools.

---

## Data Source Recommendations

### For Solutions 3 & 4: Where to get Dewey data?

1. **OCLC Dewey Services** (Official but paid)
   - https://www.oclc.org/en/dewey.html
   - Most accurate and complete

2. **LibraryThing Dewey Browser** (Free, partial)
   - https://www.librarything.com/mds/
   - Good for main categories

3. **Wikipedia Dewey List** (Free, basic)
   - Main and division levels only
   - Good starting point

4. **Create Minimal Set Manually**
   - Just include ~50-100 most common categories
   - Covers 90% of personal library needs
   - Examples:
     - 004 Computer programming
     - 005 Computer software
     - 510 Mathematics
     - 610 Medicine
     - 641 Cooking
     - 793 Games & sports
     - 813 American fiction
     - 823 English fiction
     - 843 French fiction

### Recommended Minimal Set for Personal Libraries

```sql
-- Just the essentials (~30 categories that cover most home libraries)
INSERT INTO dewey_classifications (code, level, name) VALUES
-- Fiction
('808', 2, 'Literature - Collections'),
('813', 3, 'American fiction'),
('823', 3, 'English fiction'),
('833', 3, 'German fiction'),
('843', 3, 'French fiction'),
('853', 3, 'Italian fiction'),
('863', 3, 'Spanish fiction'),
-- Non-fiction popular
('004', 3, 'Data processing & Computer science'),
('100', 1, 'Philosophy & Psychology'),
('150', 2, 'Psychology'),
('200', 1, 'Religion'),
('300', 1, 'Social Sciences'),
('330', 2, 'Economics'),
('500', 1, 'Science'),
('510', 2, 'Mathematics'),
('520', 2, 'Astronomy'),
('530', 2, 'Physics'),
('540', 2, 'Chemistry'),
('550', 2, 'Earth sciences'),
('570', 2, 'Life sciences & Biology'),
('600', 1, 'Technology'),
('610', 2, 'Medicine & health'),
('620', 2, 'Engineering'),
('640', 2, 'Home & family management'),
('641', 3, 'Food & cooking'),
('700', 1, 'Arts'),
('780', 2, 'Music'),
('790', 2, 'Sports & recreation'),
('900', 1, 'History & Geography'),
('920', 2, 'Biography');
```

---

## Integration with Google Books API

Your system already fetches book data from Google Books. The API sometimes includes Dewey classifications!

```rust
// In api_client.rs - when fetching ISBN data

#[derive(Debug, Deserialize)]
struct GoogleBookClassification {
    #[serde(rename = "classificationSystem")]
    classification_system: String, // "Dewey" or "LCC"
    value: String, // "515.35" or similar
}

// Parse response and extract Dewey if available
if let Some(classifications) = book_data.volume_info.classifications {
    for classification in classifications {
        if classification.classification_system == "Dewey" {
            title.dewey_code = Some(classification.value);
            // Optionally lookup category name from your reference table
        }
    }
}
```

---

## Final Recommendation

**For rbibli (personal library), I recommend:**

1. **Start with Solution 1** (simple text input)
   - Takes 2-3 hours
   - Optional field, no pressure
   - Works for power users

2. **Later add Solution 2** (main class dropdown)
   - Takes 1 day
   - Much better UX
   - Covers 80% of use cases

3. **Skip Solutions 3 & 4** unless users specifically request them
   - Personal libraries rarely need full Dewey precision
   - Genres (already implemented) cover most categorization needs
   - Dewey is more useful for large institutional libraries

**Alternative: Consider whether Dewey is needed at all**
- Your Genre system already provides categorization
- Dewey might be overkill for a personal library
- Physical location (shelves) might be more useful than Dewey for finding books at home
- Could make Dewey completely optional and de-emphasize it in UI

Would you like me to implement Solution 1 or 2 right now?
