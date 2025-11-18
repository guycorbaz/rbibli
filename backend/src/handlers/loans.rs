//! Loan management API handlers.
//!
//! This module provides HTTP handlers for managing book loans in the library system.
//! It includes endpoints for creating loans, returning volumes, extending loan periods,
//! and querying active and overdue loans.
//!
//! # Endpoints
//!
//! - `GET /api/v1/loans` - List all active loans with details
//! - `GET /api/v1/loans/overdue` - List overdue loans
//! - `POST /api/v1/loans` - Create a new loan by barcode scanning
//! - `POST /api/v1/loans/{id}/return` - Return a loaned volume
//! - `POST /api/v1/loans/{id}/extend` - Extend a loan period
//!
//! # Loan Workflow
//!
//! 1. **Create Loan**: Scan volume barcode, select borrower → System validates and creates loan
//! 2. **Active Loan**: Volume is marked as loaned, due date set based on borrower group
//! 3. **Extension**: Loan can be extended once (adds same duration as original)
//! 4. **Return**: Volume is returned, loan marked complete, volume becomes available
//!
//! # Business Rules
//!
//! - Loans can be extended maximum once (extension_count ≤ 1)
//! - Extension adds same duration as original loan (e.g., 21-day loan → +21 days)
//! - Overdue loans CAN be extended (trust-based system)
//! - Volume must be loanable and available
//! - Loan duration determined by borrower group (default: 21 days)

use actix_web::{web, HttpResponse, Responder};
use crate::models::{
    Loan, LoanStatus, LoanDetail, CreateLoanRequest
};
use crate::AppState;
use log::{info, error};
use sqlx::Row;
use uuid::Uuid;
use chrono::{Utc, Duration};

/// Lists all active loans with complete details including borrower and title information.
///
/// # Endpoint
///
/// `GET /api/v1/loans`
///
/// # Description
///
/// Retrieves all currently active loans (status = 'active' or 'overdue') with joined
/// information from titles, volumes, and borrowers tables. The response includes
/// calculated fields like overdue status and extension count.
///
/// # Query Details
///
/// Performs a JOIN across four tables:
/// - `loans` (l) - Core loan records
/// - `titles` (t) - Book title information
/// - `volumes` (v) - Physical volume details (barcode)
/// - `borrowers` (b) - Borrower contact information
///
/// Filters to only active/overdue loans and orders by loan_date descending (newest first).
///
/// # Returns
///
/// * `200 OK` - Array of LoanDetail objects
/// * `500 Internal Server Error` - Database query failed
///
/// # Success Response
///
/// ```json
/// [
///   {
///     "id": "loan-uuid",
///     "title_id": "title-uuid",
///     "volume_id": "volume-uuid",
///     "borrower_id": "borrower-uuid",
///     "loan_date": 1699564800,
///     "due_date": 1701374400,
///     "extension_count": 0,
///     "return_date": null,
///     "status": "active",
///     "created_at": 1699564800,
///     "updated_at": 1699564800,
///     "title": "The Hobbit",
///     "barcode": "VOL-000123",
///     "borrower_name": "John Doe",
///     "borrower_email": "john@example.com",
///     "is_overdue": false
///   }
/// ]
/// ```
///
/// # Overdue Detection
///
/// The `is_overdue` field is calculated in the SQL query:
/// - `TRUE` if current time > due_date AND status = 'active'
/// - `FALSE` otherwise
///
/// # Performance
///
/// Uses indexed foreign keys for efficient joins. Typical response time < 100ms
/// for collections under 10,000 loans.
pub async fn list_active_loans(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/loans - Fetching all active loans");

    let query = "
        SELECT
            l.id, l.title_id, l.volume_id, l.borrower_id,
            l.loan_date, l.due_date, l.extension_count, l.return_date, l.status,
            l.created_at, l.updated_at,
            t.title,
            v.barcode,
            b.name as borrower_name,
            b.email as borrower_email,
            CASE WHEN l.due_date < NOW() AND l.status = 'active' THEN TRUE ELSE FALSE END as is_overdue
        FROM loans l
        INNER JOIN titles t ON l.title_id = t.id
        INNER JOIN volumes v ON l.volume_id = v.id
        INNER JOIN borrowers b ON l.borrower_id = b.id
        WHERE l.status IN ('active', 'overdue')
        ORDER BY l.loan_date DESC
    ";

    match sqlx::query(query).fetch_all(&data.db_pool).await {
        Ok(rows) => {
            let loans: Vec<LoanDetail> = rows
                .iter()
                .map(|row| {
                    let id: String = row.get("id");
                    let loan_date: chrono::NaiveDateTime = row.get("loan_date");
                    let due_date: chrono::NaiveDateTime = row.get("due_date");
                    let return_date: Option<chrono::NaiveDateTime> = row.get("return_date");
                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");
                    let status_str: String = row.get("status");

                    LoanDetail {
                        loan: Loan {
                            id: Uuid::parse_str(&id).unwrap(),
                            title_id: row.get("title_id"),
                            volume_id: row.get("volume_id"),
                            borrower_id: row.get("borrower_id"),
                            loan_date: chrono::DateTime::from_naive_utc_and_offset(loan_date, Utc),
                            due_date: chrono::DateTime::from_naive_utc_and_offset(due_date, Utc),
                            extension_count: row.get("extension_count"),
                            return_date: return_date.map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                            status: match status_str.as_str() {
                                "active" => LoanStatus::Active,
                                "returned" => LoanStatus::Returned,
                                "overdue" => LoanStatus::Overdue,
                                _ => LoanStatus::Active,
                            },
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, Utc),
                        },
                        title: row.get("title"),
                        barcode: row.get("barcode"),
                        borrower_name: row.get("borrower_name"),
                        borrower_email: row.get("borrower_email"),
                        is_overdue: row.get("is_overdue"),
                    }
                })
                .collect();

            info!("Successfully fetched {} active loans", loans.len());
            HttpResponse::Ok().json(loans)
        }
        Err(e) => {
            error!("Failed to fetch active loans: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch active loans"
            }))
        }
    }
}

/// Lists all overdue loans sorted by due date (most overdue first).
///
/// # Endpoint
///
/// `GET /api/v1/loans/overdue`
///
/// # Description
///
/// Retrieves loans that are past their due date and still active (not returned).
/// Useful for generating reminder lists or overdue reports.
///
/// # Query Details
///
/// Filters loans where:
/// - `due_date < NOW()` - Past the due date
/// - `status = 'active'` - Not yet returned
///
/// Results are ordered by `due_date ASC` (most overdue first), allowing
/// prioritization of follow-up actions.
///
/// # Returns
///
/// * `200 OK` - Array of LoanDetail objects with is_overdue always true
/// * `500 Internal Server Error` - Database query failed
///
/// # Use Cases
///
/// - Generate overdue reminder emails
/// - Create "books to follow up on" reports
/// - Dashboard warnings for library management
/// - Statistical analysis of loan durations
///
/// # Performance
///
/// Uses composite index on (due_date, status) for efficient filtering.
pub async fn list_overdue_loans(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/loans/overdue - Fetching overdue loans");

    let query = "
        SELECT
            l.id, l.title_id, l.volume_id, l.borrower_id,
            l.loan_date, l.due_date, l.extension_count, l.return_date, l.status,
            l.created_at, l.updated_at,
            t.title,
            v.barcode,
            b.name as borrower_name,
            b.email as borrower_email,
            TRUE as is_overdue
        FROM loans l
        INNER JOIN titles t ON l.title_id = t.id
        INNER JOIN volumes v ON l.volume_id = v.id
        INNER JOIN borrowers b ON l.borrower_id = b.id
        WHERE l.due_date < NOW() AND l.status = 'active'
        ORDER BY l.due_date ASC
    ";

    match sqlx::query(query).fetch_all(&data.db_pool).await {
        Ok(rows) => {
            let loans: Vec<LoanDetail> = rows
                .iter()
                .map(|row| {
                    let id: String = row.get("id");
                    let loan_date: chrono::NaiveDateTime = row.get("loan_date");
                    let due_date: chrono::NaiveDateTime = row.get("due_date");
                    let return_date: Option<chrono::NaiveDateTime> = row.get("return_date");
                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    LoanDetail {
                        loan: Loan {
                            id: Uuid::parse_str(&id).unwrap(),
                            title_id: row.get("title_id"),
                            volume_id: row.get("volume_id"),
                            borrower_id: row.get("borrower_id"),
                            loan_date: chrono::DateTime::from_naive_utc_and_offset(loan_date, Utc),
                            due_date: chrono::DateTime::from_naive_utc_and_offset(due_date, Utc),
                            extension_count: row.get("extension_count"),
                            return_date: return_date.map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                            status: LoanStatus::Overdue,
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, Utc),
                        },
                        title: row.get("title"),
                        barcode: row.get("barcode"),
                        borrower_name: row.get("borrower_name"),
                        borrower_email: row.get("borrower_email"),
                        is_overdue: true,
                    }
                })
                .collect();

            info!("Successfully fetched {} overdue loans", loans.len());
            HttpResponse::Ok().json(loans)
        }
        Err(e) => {
            error!("Failed to fetch overdue loans: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch overdue loans"
            }))
        }
    }
}

/// Creates a new loan by scanning a volume barcode.
///
/// # Endpoint
///
/// `POST /api/v1/loans`
///
/// # Request Body
///
/// ```json
/// {
///   "borrower_id": "borrower-uuid",
///   "barcode": "VOL-000123"
/// }
/// ```
///
/// # Workflow
///
/// 1. **Lookup Volume**: Find volume by barcode
/// 2. **Validate Loanability**: Check volume is loanable and available
/// 3. **Get Loan Duration**: Fetch borrower's group loan duration (default: 21 days)
/// 4. **Calculate Dates**: loan_date = now, due_date = now + duration
/// 5. **Create Loan**: Insert loan record with status 'active', extension_count 0
/// 6. **Update Volume**: Set volume.loan_status = 'loaned'
///
/// All operations are performed in a database transaction for atomicity.
///
/// # Validation Rules
///
/// - Borrower must exist in database
/// - Volume must exist with matching barcode
/// - Volume must be loanable (loanable = true)
/// - Volume must be available (loan_status = 'available')
///
/// # Success Response
///
/// **Status**: 201 Created
///
/// ```json
/// {
///   "id": "new-loan-uuid",
///   "due_date": 1701374400,
///   "loan_duration_days": 21
/// }
/// ```
///
/// # Error Responses
///
/// **404 Not Found** - Volume barcode not found
/// ```json
/// {
///   "error": "Volume not found with this barcode"
/// }
/// ```
///
/// **400 Bad Request** - Volume not loanable
/// ```json
/// {
///   "error": "This volume is not loanable (damaged or restricted)"
/// }
/// ```
///
/// **400 Bad Request** - Volume already loaned
/// ```json
/// {
///   "error": "This volume is already loaned"
/// }
/// ```
///
/// **404 Not Found** - Borrower not found
/// ```json
/// {
///   "error": "Borrower not found"
/// }
/// ```
///
/// **500 Internal Server Error** - Database error during transaction
///
/// # Transaction Safety
///
/// Uses database transaction to ensure atomic operation:
/// - If loan creation fails, volume status is not updated
/// - If volume update fails, loan creation is rolled back
/// - Guarantees data consistency
pub async fn create_loan_by_barcode(
    request: web::Json<CreateLoanRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/loans - Creating loan for borrower: {}, barcode: {}",
          request.borrower_id, request.barcode);

    // 1. Find volume by barcode
    let volume_query = "SELECT id, title_id, loanable, loan_status FROM volumes WHERE barcode = ?";
    let volume_row = match sqlx::query(volume_query)
        .bind(&request.barcode)
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            error!("Volume not found with barcode: {}", request.barcode);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Volume not found with this barcode"
            }));
        }
        Err(e) => {
            error!("Failed to fetch volume: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch volume"
            }));
        }
    };

    let volume_id: String = volume_row.get("id");
    let title_id: String = volume_row.get("title_id");
    let loanable: bool = volume_row.get("loanable");
    let loan_status: String = volume_row.get("loan_status");

    // 2. Check if volume is loanable
    if !loanable {
        error!("Volume {} is not loanable", request.barcode);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "This volume is not loanable (damaged or restricted)"
        }));
    }

    // 3. Check if volume is already loaned
    if loan_status != "available" {
        error!("Volume {} is already loaned", request.barcode);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "This volume is already loaned"
        }));
    }

    // 4. Get borrower's group loan duration
    let borrower_query = "
        SELECT b.id, b.name, COALESCE(g.loan_duration_days, 21) as loan_duration_days
        FROM borrowers b
        LEFT JOIN borrower_groups g ON b.group_id = g.id
        WHERE b.id = ?
    ";

    let borrower_row = match sqlx::query(borrower_query)
        .bind(&request.borrower_id)
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            error!("Borrower not found: {}", request.borrower_id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Borrower not found"
            }));
        }
        Err(e) => {
            error!("Failed to fetch borrower: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch borrower"
            }));
        }
    };

    let loan_duration_days: i32 = borrower_row.get("loan_duration_days");
    let borrower_name: String = borrower_row.get("name");

    // 5. Calculate dates
    let loan_date = Utc::now();
    let due_date = loan_date + Duration::days(loan_duration_days as i64);
    let loan_id = Uuid::new_v4().to_string();

    // 6. Create loan transaction
    let mut tx = match data.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create loan"
            }));
        }
    };

    // Insert loan
    let insert_loan = "
        INSERT INTO loans (id, title_id, volume_id, borrower_id, loan_date, due_date, status)
        VALUES (?, ?, ?, ?, ?, ?, 'active')
    ";

    if let Err(e) = sqlx::query(insert_loan)
        .bind(&loan_id)
        .bind(&title_id)
        .bind(&volume_id)
        .bind(&request.borrower_id)
        .bind(loan_date.naive_utc())
        .bind(due_date.naive_utc())
        .execute(&mut *tx)
        .await
    {
        error!("Failed to insert loan: {}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to create loan"
        }));
    }

    // Update volume status
    let update_volume = "UPDATE volumes SET loan_status = 'loaned' WHERE id = ?";
    if let Err(e) = sqlx::query(update_volume)
        .bind(&volume_id)
        .execute(&mut *tx)
        .await
    {
        error!("Failed to update volume status: {}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to update volume status"
        }));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        error!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to complete loan creation"
        }));
    }

    info!("Successfully created loan: {} for borrower: {}, volume: {}",
          loan_id, borrower_name, request.barcode);

    HttpResponse::Created().json(serde_json::json!({
        "id": loan_id,
        "due_date": due_date.timestamp(),
        "loan_duration_days": loan_duration_days
    }))
}

/// Marks a loan as returned and makes the volume available again.
///
/// # Endpoint
///
/// `POST /api/v1/loans/{id}/return`
///
/// # Path Parameters
///
/// * `id` - UUID of the loan to return
///
/// # Workflow
///
/// 1. **Lookup Loan**: Verify loan exists and get volume_id
/// 2. **Validate Status**: Ensure loan is not already returned
/// 3. **Update Loan**: Set return_date = now, status = 'returned'
/// 4. **Update Volume**: Set volume.loan_status = 'available'
///
/// All operations are performed in a database transaction for atomicity.
///
/// # Returns
///
/// * `200 OK` - Loan successfully returned
/// * `404 Not Found` - Loan doesn't exist
/// * `400 Bad Request` - Loan already returned
/// * `500 Internal Server Error` - Database error
///
/// # Success Response
///
/// **Status**: 200 OK
///
/// ```json
/// {
///   "message": "Loan returned successfully",
///   "return_date": 1699650000
/// }
/// ```
///
/// # Error Responses
///
/// **404 Not Found** - Loan doesn't exist
/// ```json
/// {
///   "error": "Loan not found"
/// }
/// ```
///
/// **400 Bad Request** - Already returned
/// ```json
/// {
///   "error": "This loan has already been returned"
/// }
/// ```
///
/// # Transaction Safety
///
/// Uses database transaction to ensure:
/// - Loan and volume status are updated atomically
/// - If either update fails, both are rolled back
/// - No partial state changes
///
/// # Side Effects
///
/// - Volume becomes available for new loans
/// - Loan appears in historical records with return_date
/// - Statistics are updated (active loan count decreases)
pub async fn return_loan(
    loan_id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/loans/{}/return - Returning loan", loan_id);

    // 1. Get loan details
    let loan_query = "SELECT id, volume_id, status FROM loans WHERE id = ?";
    let loan_row = match sqlx::query(loan_query)
        .bind(loan_id.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            error!("Loan not found: {}", loan_id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Loan not found"
            }));
        }
        Err(e) => {
            error!("Failed to fetch loan: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch loan"
            }));
        }
    };

    let volume_id: String = loan_row.get("volume_id");
    let status: String = loan_row.get("status");

    // 2. Check if loan is already returned
    if status == "returned" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "This loan has already been returned"
        }));
    }

    let return_date = Utc::now();

    // 3. Update loan and volume in transaction
    let mut tx = match data.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to process return"
            }));
        }
    };

    // Update loan
    let update_loan = "
        UPDATE loans
        SET return_date = ?, status = 'returned', updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
    ";

    if let Err(e) = sqlx::query(update_loan)
        .bind(return_date.naive_utc())
        .bind(loan_id.as_str())
        .execute(&mut *tx)
        .await
    {
        error!("Failed to update loan: {}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to process return"
        }));
    }

    // Update volume status
    let update_volume = "UPDATE volumes SET loan_status = 'available' WHERE id = ?";
    if let Err(e) = sqlx::query(update_volume)
        .bind(&volume_id)
        .execute(&mut *tx)
        .await
    {
        error!("Failed to update volume status: {}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to update volume status"
        }));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        error!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to complete return"
        }));
    }

    info!("Successfully returned loan: {}", loan_id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Loan returned successfully",
        "return_date": return_date.timestamp()
    }))
}

/// Extends an active loan by adding the same duration as the original loan period.
///
/// # Endpoint
///
/// `POST /api/v1/loans/{id}/extend`
///
/// # Path Parameters
///
/// * `id` - UUID of the loan to extend
///
/// # Business Logic
///
/// 1. Validates loan exists and is active (not returned)
/// 2. Checks extension limit (max 1 extension per loan)
/// 3. Calculates original loan duration: `due_date - loan_date`
/// 4. Adds that duration to current due date: `new_due_date = due_date + original_duration`
/// 5. Increments `extension_count` and updates `due_date`
///
/// # Extension Policy
///
/// - **Maximum extensions**: 1 per loan
/// - **Extension duration**: Same as original loan period
/// - **Example**: A 21-day loan gets 21 more days (total: 42 days from original loan date)
/// - **Overdue loans**: CAN be extended (trust-based system)
///
/// # Success Response
///
/// **Status**: 200 OK
///
/// ```json
/// {
///   "message": "Loan extended successfully",
///   "new_due_date": 1700000000,
///   "extension_count": 1,
///   "original_duration_days": 21
/// }
/// ```
///
/// # Error Responses
///
/// **404 Not Found** - Loan doesn't exist
/// ```json
/// {
///   "error": "Loan not found"
/// }
/// ```
///
/// **400 Bad Request** - Loan already returned
/// ```json
/// {
///   "error": "Cannot extend a returned loan"
/// }
/// ```
///
/// **409 Conflict** - Extension limit reached
/// ```json
/// {
///   "error": "Maximum extensions reached (1/1)",
///   "extension_count": 1
/// }
/// ```
///
/// **500 Internal Server Error** - Database error
///
/// # Example
///
/// Loan created on 2024-01-01 with 21-day duration (due 2024-01-22):
/// - First extension on 2024-01-15 → new due date: 2024-02-12 (21 more days)
/// - Second extension attempt → 409 Conflict (limit reached)
pub async fn extend_loan(
    loan_id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/loans/{}/extend - Extending loan", loan_id);

    // 1. Get loan details
    let loan_query = "
        SELECT id, loan_date, due_date, extension_count, status
        FROM loans
        WHERE id = ?
    ";

    let loan_row = match sqlx::query(loan_query)
        .bind(loan_id.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            error!("Loan not found: {}", loan_id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Loan not found"
            }));
        }
        Err(e) => {
            error!("Failed to fetch loan: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch loan"
            }));
        }
    };

    let status: String = loan_row.get("status");
    let extension_count: i32 = loan_row.get("extension_count");
    let loan_date: chrono::NaiveDateTime = loan_row.get("loan_date");
    let due_date: chrono::NaiveDateTime = loan_row.get("due_date");

    // 2. Check if loan is active (not returned)
    if status == "returned" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Cannot extend a returned loan"
        }));
    }

    // 3. Check if extension limit reached (max 1 extension)
    if extension_count >= 1 {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Maximum extensions reached (1/1)",
            "extension_count": extension_count
        }));
    }

    // 4. Calculate original loan duration and new due date
    let loan_date_utc: chrono::DateTime<Utc> = chrono::DateTime::from_naive_utc_and_offset(loan_date, Utc);
    let due_date_utc: chrono::DateTime<Utc> = chrono::DateTime::from_naive_utc_and_offset(due_date, Utc);

    let original_duration = due_date_utc.signed_duration_since(loan_date_utc);
    let new_due_date = due_date_utc + original_duration;

    // 5. Update loan with new due date and increment extension_count
    let update_query = "
        UPDATE loans
        SET due_date = ?, extension_count = extension_count + 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
    ";

    if let Err(e) = sqlx::query(update_query)
        .bind(new_due_date.naive_utc())
        .bind(loan_id.as_str())
        .execute(&data.db_pool)
        .await
    {
        error!("Failed to extend loan: {}", e);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to extend loan"
        }));
    }

    info!("Successfully extended loan: {} - New due date: {}", loan_id, new_due_date);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Loan extended successfully",
        "new_due_date": new_due_date.timestamp(),
        "extension_count": extension_count + 1,
        "original_duration_days": original_duration.num_days()
    }))
}
