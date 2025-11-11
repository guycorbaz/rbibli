# Frontend Location Management

## Overview

The frontend now includes a complete location management interface with support for hierarchical locations. Users can view, create, and delete locations directly from the UI.

## Features

### ✅ What's Implemented

1. **View Locations** - List all locations with hierarchical paths
2. **Create Locations** - Add new locations with optional parent
3. **Delete Locations** - Remove locations
4. **Hierarchical Display** - Visual indentation shows location hierarchy
5. **Add Child Locations** - Quick button to add sub-locations
6. **Real-time Updates** - UI refreshes after create/delete operations

## Navigation

The Locations page is accessible from the second tab in the sidebar:

```
Sidebar Menu:
├── Titles
├── Locations ← New!
├── Loans
├── Statistics
└── About
```

## User Interface

### Main View

The locations page shows:
- **Header** with "Storage Locations" title
- **Refresh button** to reload locations
- **"+ New Location" button** to create locations
- **List of locations** with:
  - Visual indentation based on hierarchy level
  - Location name (bold)
  - Full path (e.g., "House > Living Room > Bookshelf A")
  - Description (if provided)
  - Action buttons (Add Child, Delete)

### Create Location Dialog

When clicking "+ New Location" or "Add Child", a dialog appears with:
- **Name field** (required)
- **Description field** (optional, multi-line)
- **Cancel/Create buttons**

The parent location is automatically set when clicking "Add Child" on an existing location.

### Hierarchical Display

Locations are displayed with visual hierarchy:

```
House
  → Living Room
    → Bookshelf A
    → Bookshelf B
  → Bedroom
    → Bedside Table
```

- Each level is indented 20px
- Arrow (→) indicates child locations
- Full path shown below location name
- Alternating row colors for readability

## Code Structure

### Frontend Components

```
frontend/
├── src/
│   ├── models.rs           # Location, LocationWithPath, CreateLocationRequest
│   ├── api_client.rs       # get_locations(), create_location(), delete_location()
│   └── main.rs             # Callbacks and UI wiring
├── ui/
│   ├── pages/
│   │   └── locations_page.slint  # LocationsPage UI component
│   └── app-window.slint    # Main window with navigation
```

### Models (frontend/src/models.rs)

```rust
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct LocationWithPath {
    pub location: Location,
    pub full_path: String,  // e.g., "House > Living Room > Shelf A"
    pub level: i32,         // Hierarchy level (0 = root)
}

pub struct CreateLocationRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}
```

### API Client Methods (frontend/src/api_client.rs)

```rust
// Fetch all locations with hierarchical paths
pub fn get_locations(&self) -> Result<Vec<LocationWithPath>, Box<dyn Error>>

// Create a new location
pub fn create_location(&self, request: CreateLocationRequest) -> Result<String, Box<dyn Error>>

// Delete a location by ID
pub fn delete_location(&self, location_id: &str) -> Result<(), Box<dyn Error>>
```

### Slint UI Component (frontend/ui/pages/locations_page.slint)

```slint
export struct LocationData {
    id: string,
    name: string,
    description: string,
    parent-id: string,
    full-path: string,
    level: int,
}

export component LocationsPage inherits Page {
    in-out property <[LocationData]> locations: [];
    callback load-locations();
    callback create-location(string, string, string);
    callback delete-location(string);

    // UI with list, create dialog, etc.
}
```

## User Workflows

### Creating a Root Location

1. Click "Locations" tab in sidebar
2. Click "+ New Location" button
3. Enter name: "House"
4. (Optional) Enter description
5. Click "Create"
6. Location appears in list

### Creating a Child Location

**Method 1: Using "Add Child" button**
1. Find parent location in list
2. Click "Add Child" button
3. Enter child name: "Living Room"
4. Click "Create"
5. Child appears indented under parent

**Method 2: Manual parent selection**
1. Click "+ New Location"
2. Enter name
3. Note the parent's UUID from the list
4. (Currently no dropdown - future enhancement)

### Deleting a Location

1. Find location in list
2. Click "Delete" button
3. Location is removed
4. Child locations become root locations (parent set to null)

## Data Flow

### Loading Locations

```
User opens Locations tab
  → load_locations() in main.rs
    → api_client.get_locations()
      → GET /api/v1/locations
      → Backend returns JSON with paths
    → Convert to Slint LocationData
    → ui.set_locations(model)
  → UI displays hierarchical list
```

### Creating a Location

```
User fills form and clicks Create
  → create-location callback
    → api_client.create_location(request)
      → POST /api/v1/locations
      → Backend returns new ID
    → load_locations() to refresh
  → UI updates with new location
```

### Deleting a Location

```
User clicks Delete button
  → delete-location callback
    → api_client.delete_location(id)
      → DELETE /api/v1/locations/{id}
    → load_locations() to refresh
  → UI updates without deleted location
```

## Example Usage

### Scenario: Setting Up a Home Library

1. **Create main location**
   - Click "+ New Location"
   - Name: "House"
   - Create

2. **Create rooms**
   - Select "House", click "Add Child"
   - Name: "Living Room"
   - Create

   - Select "House", click "Add Child"
   - Name: "Bedroom"
   - Create

3. **Create storage units**
   - Select "Living Room", click "Add Child"
   - Name: "Bookshelf A"
   - Description: "White bookshelf, 5 shelves"
   - Create

   - Select "Living Room", click "Add Child"
   - Name: "Bookshelf B"
   - Create

4. **Result**
   ```
   House
     → Living Room
       → Bookshelf A (White bookshelf, 5 shelves)
       → Bookshelf B
     → Bedroom
   ```

## Testing

### Manual Testing

1. **Start backend:**
   ```bash
   cd backend
   cargo run
   ```

2. **Start frontend:**
   ```bash
   cd frontend
   cargo run
   ```

3. **Test workflows:**
   - Click "Locations" tab
   - Create a few locations
   - Create child locations
   - Delete locations
   - Refresh to verify persistence

### Console Output

The frontend logs all operations:
```
Loading locations from backend...
Fetching locations from: http://localhost:8000/api/v1/locations
Successfully fetched 5 locations
UI updated with locations

Creating location: Bookshelf A
Successfully created location with ID: c6c55727-fb19-422b-a783-15c1745518d5

Deleting location: c6c55727-fb19-422b-a783-15c1745518d5
Successfully deleted location
```

## Limitations & Future Enhancements

### Current Limitations

1. **No dropdown for parent selection** - Must use "Add Child" button
2. **No edit functionality** - Can only create and delete
3. **No confirmation dialog** - Delete is immediate
4. **No search/filter** - All locations always shown
5. **No validation** - Can create duplicate names

### Future Enhancements

1. **Edit locations** - Update name, description, parent
2. **Parent selector** - Dropdown to choose parent when creating
3. **Confirmation dialogs** - "Are you sure?" for delete
4. **Search/filter** - Find locations by name
5. **Drag & drop** - Move locations in hierarchy
6. **Statistics** - Show volume count per location
7. **Validation** - Prevent duplicate names, circular references
8. **Batch operations** - Select multiple locations
9. **Export/import** - CSV or JSON format
10. **Location icons** - Visual indicators (house, room, shelf)

## Troubleshooting

### Locations Don't Load

**Check:**
1. Backend is running: `curl http://localhost:8000/health`
2. Locations exist: `curl http://localhost:8000/api/v1/locations`
3. Console for errors
4. Network tab in browser (if WASM version)

**Fix:**
- Ensure backend is running on port 8000
- Check `.env` file has correct DATABASE_URL
- Verify migrations were applied: `sqlx migrate info`

### Create Button Disabled

**Reason:** Name field is empty (required)

**Fix:** Enter a location name

### Location Not Deleted

**Check console for error:**
- Location might be referenced by volumes
- Database foreign key constraint

**Fix:**
- Remove volumes from location first
- Or update backend to allow deleting with volumes

### Hierarchy Not Showing

**Check:**
- `parent_id` field in database
- Recursive query in backend working
- `level` field being returned by API

**Fix:**
- Check backend logs
- Verify database foreign keys
- Test API directly: `curl http://localhost:8000/api/v1/locations`

## Integration with Volumes

In the future, when creating/editing volumes, users will be able to:
1. Select a location from the hierarchy
2. See which volumes are in each location
3. Move volumes between locations
4. Filter volumes by location

This will provide complete location-based organization for your library!

## Summary

✅ **Full CRUD** (except Update)
✅ **Hierarchical display** with visual indentation
✅ **Real-time updates** after operations
✅ **Clean UI** with dialogs and buttons
✅ **Backend integration** with REST API
✅ **Error handling** with console logging

The location management system is now fully functional and ready to use!
