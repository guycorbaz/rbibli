# Locations Feature Documentation

## Overview

The locations feature has been fully implemented to manage physical storage locations for your library volumes. Locations support **hierarchical organization**, meaning locations can be nested inside other locations (e.g., "Shelf 1" inside "Room A" inside "House").

## Database Schema

### Locations Table

```sql
CREATE TABLE locations (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    parent_id CHAR(36),  -- References another location (parent)
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES locations(id) ON DELETE SET NULL
);
```

### Volumes Table Update

The `volumes` table has been updated to reference locations:

```sql
ALTER TABLE volumes ADD COLUMN location_id CHAR(36);
ALTER TABLE volumes ADD CONSTRAINT fk_volumes_location
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL;

-- Old text field removed
ALTER TABLE volumes DROP COLUMN location;
```

## Hierarchical Structure

Locations can be organized in a tree structure:

```
House
├── Living Room
│   ├── Bookshelf A
│   └── Bookshelf B
└── Bedroom
    └── Bedside Table
```

When listing locations, the API returns the full hierarchical path:
- "House"
- "House > Living Room"
- "House > Living Room > Bookshelf A"
- "House > Living Room > Bookshelf B"
- "House > Bedroom"
- "House > Bedroom > Bedside Table"

## API Endpoints

All endpoints are under `/api/v1/locations`

### 1. List All Locations

```
GET /api/v1/locations
```

Returns all locations with their full hierarchical paths.

**Response:**
```json
[
  {
    "location": {
      "id": "uuid",
      "name": "House",
      "description": "Main house",
      "parent_id": null,
      "created_at": 1699564800,
      "updated_at": 1699564800
    },
    "full_path": "House",
    "level": 0
  },
  {
    "location": {
      "id": "uuid",
      "name": "Living Room",
      "description": null,
      "parent_id": "parent-uuid",
      "created_at": 1699564800,
      "updated_at": 1699564800
    },
    "full_path": "House > Living Room",
    "level": 1
  }
]
```

### 2. Get Single Location

```
GET /api/v1/locations/{id}
```

Get details of a specific location by UUID.

**Response:**
```json
{
  "id": "uuid",
  "name": "Bookshelf A",
  "description": "White bookshelf in living room",
  "parent_id": "parent-uuid",
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

### 3. Create Location

```
POST /api/v1/locations
Content-Type: application/json

{
  "name": "Bookshelf A",
  "description": "White bookshelf in living room",
  "parent_id": "parent-location-uuid"  // Optional
}
```

**Response:**
```json
{
  "id": "new-uuid",
  "message": "Location created successfully"
}
```

### 4. Update Location

```
PUT /api/v1/locations/{id}
Content-Type: application/json

{
  "name": "Bookshelf A - Updated",       // Optional
  "description": "Updated description",  // Optional
  "parent_id": "new-parent-uuid"        // Optional (null to remove parent)
}
```

**Response:**
```json
{
  "message": "Location updated successfully"
}
```

### 5. Delete Location

```
DELETE /api/v1/locations/{id}
```

Deletes a location. If the location has child locations, their `parent_id` will be set to NULL (they become root locations).

**Response:**
```json
{
  "message": "Location deleted successfully"
}
```

## Running the Migrations

To apply the new database schema:

```bash
cd backend

# Check if sqlx-cli is installed
sqlx --version

# If not installed, install it
cargo install sqlx-cli --features mysql

# Run migrations
sqlx migrate run
```

This will:
1. Create the `locations` table
2. Update the `volumes` table to use `location_id`
3. Remove the old `location` text field

## Example Usage

### Creating a Hierarchical Structure

```bash
# 1. Create root location (House)
curl -X POST http://localhost:8000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{"name": "House", "description": "Main house"}'
# Returns: {"id": "house-uuid", "message": "..."}

# 2. Create child location (Living Room)
curl -X POST http://localhost:8000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Living Room",
    "parent_id": "house-uuid"
  }'
# Returns: {"id": "livingroom-uuid", "message": "..."}

# 3. Create grandchild location (Bookshelf A)
curl -X POST http://localhost:8000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Bookshelf A",
    "description": "White bookshelf",
    "parent_id": "livingroom-uuid"
  }'
```

### Listing Locations

```bash
curl http://localhost:8000/api/v1/locations
```

Will return locations sorted by path:
```json
[
  {"full_path": "House", "level": 0, ...},
  {"full_path": "House > Living Room", "level": 1, ...},
  {"full_path": "House > Living Room > Bookshelf A", "level": 2, ...}
]
```

### Moving a Location

To move "Bookshelf A" from "Living Room" to "Bedroom":

```bash
curl -X PUT http://localhost:8000/api/v1/locations/bookshelf-a-uuid \
  -H "Content-Type: application/json" \
  -d '{"parent_id": "bedroom-uuid"}'
```

## Integration with Volumes

When creating or updating a volume, you can now reference a location:

```sql
-- Example: Assign a volume to a location
UPDATE volumes
SET location_id = 'location-uuid'
WHERE id = 'volume-uuid';
```

In the future API for volumes management, this will be:

```json
POST /api/v1/volumes
{
  "title_id": "...",
  "barcode": "VOL-000001",
  "condition": "excellent",
  "location_id": "bookshelf-a-uuid"
}
```

## Error Responses

All endpoints return standard error responses:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable message",
    "details": {}
  }
}
```

Common error codes:
- `INVALID_UUID`: Malformed location ID
- `INVALID_PARENT_UUID`: Malformed parent location ID
- `NOT_FOUND`: Location doesn't exist
- `DATABASE_ERROR`: Database operation failed
- `NO_UPDATES`: No fields provided for update

## Logging

All location operations are logged:

- **INFO**: API calls, successful operations
- **WARN**: Invalid UUIDs, not found errors
- **ERROR**: Database failures
- **DEBUG**: Query execution, row processing

Enable debug logging to see detailed information:
```bash
RUST_LOG=debug cargo run
```

## Benefits of Hierarchical Locations

1. **Organization**: Group related locations together
2. **Flexibility**: Move entire subtrees by changing parent
3. **Queries**: Find all volumes in "House" (including all sublocations)
4. **UI**: Display as tree view or breadcrumb navigation
5. **Reporting**: Generate location-based statistics

## Next Steps

Future enhancements:
1. **Frontend UI**: Location management page in Slint
2. **Volume Integration**: Update volumes API to use locations
3. **Search**: Find volumes by location (including children)
4. **Statistics**: Count volumes per location
5. **Validation**: Prevent circular references (A → B → A)
6. **Bulk Operations**: Move multiple volumes between locations

## Testing

Test the API with curl or any HTTP client:

```bash
# Health check
curl http://localhost:8000/health

# List locations
curl http://localhost:8000/api/v1/locations

# Create location
curl -X POST http://localhost:8000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Location"}'

# Get location
curl http://localhost:8000/api/v1/locations/{uuid}

# Update location
curl -X PUT http://localhost:8000/api/v1/locations/{uuid} \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name"}'

# Delete location
curl -X DELETE http://localhost:8000/api/v1/locations/{uuid}
```

## Database Rollback

If you need to revert the changes:

```bash
# Rollback the volumes update
sqlx migrate revert

# Rollback the locations table creation
sqlx migrate revert
```

This will restore the old `location` VARCHAR field in the volumes table and remove the locations table.
