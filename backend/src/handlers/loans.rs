use actix_web::{web, HttpResponse, Responder};
use crate::models::{
    Loan, LoanStatus, LoanDetail, CreateLoanRequest
};
use crate::AppState;
use log::{info, error};
use sqlx::Row;
use uuid::Uuid;
use chrono::{Utc, Duration};

/// GET /api/v1/loans - List all active loans with details
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

/// GET /api/v1/loans/overdue - List overdue loans
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

/// POST /api/v1/loans - Create a loan by barcode scanning
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

/// POST /api/v1/loans/{id}/return - Return a loaned volume
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

/// POST /api/v1/loans/{id}/extend - Extend a loan
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
