# Dewey Decimal Classification Implementation

**Status**: Implemented (Simplified)
**Date**: 2025-11-26

## Overview

The project originally considered a complex Dewey Decimal Classification system with a full database of codes, search functionality, and hierarchical browsing. However, for the scope of a personal library, this was deemed over-engineered.

**Decision**: The implementation has been simplified to **manual text input** for Dewey Codes and Categories.

## Current Implementation

### Database Schema
- `titles` table contains two columns:
  - `dewey_code` (VARCHAR): The numeric code (e.g., "005.133")
  - `dewey_category` (VARCHAR): The human-readable category (e.g., "Computer programming")
- No separate `dewey_classifications` table (it was removed to simplify the architecture).

### Backend API
- `Title` entity includes `dewey_code` and `dewey_category` fields.
- No dedicated Dewey API endpoints (search/browse endpoints were removed).

### Frontend UI
- **Titles Page**:
  - Create/Edit dialogs include text input fields for "Dewey Code" and "Dewey Category".
  - These fields are optional.
  - Users can manually enter codes found on physical books or via external search.

### Google Books Integration
- When looking up an ISBN, if the Google Books API returns Dewey classification data, it is automatically populated into the `dewey_code` field.

---

## Historical Proposals (Rejected/Simplified)

*The following proposals were considered but not fully implemented or were removed to reduce complexity.*

### Proposal 1: Full Database & Browsing
- **Idea**: Import the full DDC hierarchy (000-999) into a database table.
- **Features**: Hierarchical browsing, autocomplete search.
- **Reason for Rejection**: Too complex for personal use; requires maintaining a large reference dataset; "Genre" system already covers most categorization needs.

### Proposal 2: Strict Validation
- **Idea**: Validate entered codes against a known list of valid Dewey codes.
- **Reason for Rejection**: Prevents users from using custom or modified codes; adds unnecessary friction.

## Future Possibilities

If the need arises, we can revisit the more complex implementation, but for now, the manual text field provides maximum flexibility with minimum maintenance overhead.
