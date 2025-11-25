// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;

// Module declarations
mod models;
mod api_client;

use api_client::ApiClient;
use models::{
    CreateTitleRequest, UpdateTitleRequest, CreateLocationRequest, UpdateLocationRequest,
    CreateAuthorRequest, UpdateAuthorRequest, CreatePublisherRequest, UpdatePublisherRequest,
    CreateGenreRequest, UpdateGenreRequest, CreateSeriesRequest, UpdateSeriesRequest,
    CreateBorrowerGroupRequest, UpdateBorrowerGroupRequest, CreateBorrowerRequest,
    UpdateBorrowerRequest, CreateLoanRequest,
    LibraryStatistics as ModelsLibraryStatistics, GenreStatistic as ModelsGenreStatistic,
    LocationStatistic as ModelsLocationStatistic, LoanStatistic as ModelsLoanStatistic
};
use slint::{Model, ComponentHandle};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

/// Main entry point for the rbibli frontend application.
///
/// This function initializes and runs the Slint-based desktop application for the rbibli
/// (personal library management) system. It sets up the UI, creates an API client for
/// backend communication, and connects all UI callbacks to their respective handlers.
///
/// # Application Structure
///
/// The application performs the following initialization:
///
/// 1. **UI Creation**: Instantiates the main `AppWindow` from the Slint UI definition
///
/// 2. **API Client**: Creates a default `ApiClient` instance configured to communicate
///    with the backend server at `http://localhost:8000`
///
/// 3. **Data Loading**: Defines closures for loading data from the backend:
///    - Titles with volume counts
///    - Locations with hierarchical paths
///    - Authors with title counts
///    - Publishers with title counts
///    - Genres with title counts
///
/// 4. **Callback Connections**: Wires up UI callbacks for:
///    - Loading data from the backend
///    - Creating new entities (titles, locations, authors, publishers, genres)
///    - Updating existing entities (titles, publishers, genres)
///    - Deleting entities (locations, authors, publishers, genres)
///    - Finding genre indices for dropdown selection
///
/// 5. **Initial Data Load**: Fetches all initial data from the backend on startup
///
/// 6. **Event Loop**: Starts the Slint event loop to handle user interactions
///
/// # Returns
///
/// * `Ok(())` - Application ran successfully and was closed by the user
/// * `Err(Box<dyn Error>)` - Application failed to start or encountered a runtime error
///
/// # Errors
///
/// This function will return an error if:
/// - Failed to create the Slint UI window
/// - Failed to start the Slint event loop
///
/// Note: Backend communication errors are logged to stderr but do not cause the
/// application to crash. Users will see error messages in the console if the backend
/// is unavailable.
///
/// # Examples
///
/// Run the application:
/// ```bash
/// # Ensure the backend server is running on http://localhost:8000
/// cargo run
/// ```
///
/// # Panics
///
/// The function may panic if the Slint UI initialization fails in an unrecoverable way,
/// though this is typically returned as an error instead.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    run().await.map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
fn main() {}

async fn run() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Set application version from Cargo.toml
    ui.set_app_version(env!("CARGO_PKG_VERSION").into());

    // Create API client
    let api_client = Rc::new(ApiClient::default());

    // Function to load titles and populate the UI
    let load_titles = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Loading titles from backend...");

                match api_client.get_titles().await {
                    Ok(titles_data) => {
                        println!("Successfully fetched {} titles", titles_data.len());

                        // Convert backend TitleWithCount to Slint TitleData
                        let slint_titles: Vec<TitleData> = titles_data
                            .iter()
                            .map(|t| TitleData {
                                id: t.title.id.clone().into(),
                                title: t.title.title.clone().into(),
                                subtitle: t.title.subtitle.clone().unwrap_or_default().into(),
                                isbn: t.title.isbn.clone().unwrap_or_default().into(),
                                publisher: t.title.publisher.clone().unwrap_or_default().into(),
                                publisher_id: t.title.publisher_id.clone().unwrap_or_default().into(),
                                volume_count: t.volume_count as i32,
                                language: t.title.language.clone().into(),
                                publication_year: t.title.publication_year.map(|y| y.to_string()).unwrap_or_default().into(),
                                pages: t.title.pages.map(|p| p.to_string()).unwrap_or_default().into(),
                                genre: t.title.genre.clone().unwrap_or_default().into(),
                                genre_id: t.title.genre_id.clone().unwrap_or_default().into(),
                                series_name: t.title.series_name.clone().unwrap_or_default().into(),
                                series_id: t.title.series_id.clone().unwrap_or_default().into(),
                                series_number: t.title.series_number.clone().unwrap_or_default().into(),
                                dewey_code: t.title.dewey_code.clone().unwrap_or_default().into(),
                                summary: t.title.summary.clone().unwrap_or_default().into(),
                                cover_url: t.title.cover_url.clone().unwrap_or_default().into(),
                                // Duplicate detection fields (initially false/empty)
                                is_duplicate: false,
                                duplicate_pair_id: "".into(),
                                duplicate_similarity: 0.0,
                                duplicate_confidence: "".into(),
                                duplicate_match_reasons: "".into(),
                                is_duplicate_primary: false,
                            })
                            .collect();

                        // Update the UI with the titles
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_titles));
                            ui.set_titles(model.into());
                            println!("UI updated with titles");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch titles: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Function to load locations and populate the UI
    let load_locations = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Loading locations from backend...");

                match api_client.get_locations().await {
                    Ok(locations_data) => {
                        println!("Successfully fetched {} locations", locations_data.len());

                        // Convert backend LocationWithPath to Slint LocationData
                        let slint_locations: Vec<LocationData> = locations_data
                            .iter()
                            .map(|l| LocationData {
                                id: l.location.id.clone().into(),
                                name: l.location.name.clone().into(),
                                description: l.location.description.clone().unwrap_or_default().into(),
                                parent_id: l.location.parent_id.clone().unwrap_or_default().into(),
                                full_path: l.full_path.clone().into(),
                                level: l.level,
                                child_count: l.child_count,
                                volume_count: l.volume_count,
                            })
                            .collect();

                        // Extract location full paths for ComboBox model
                        // Include "(No location)" as first element
                        let mut location_names: Vec<slint::SharedString> = vec!["(No location)".into()];
                        location_names.extend(
                            locations_data
                                .iter()
                                .map(|l| l.full_path.clone().into())
                        );

                        // Update the UI with the locations
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_locations));
                            ui.set_locations(model.into());
                            let names_model = Rc::new(slint::VecModel::from(location_names));
                            ui.set_location_names(names_model.into());
                            println!("UI updated with locations");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch locations: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Function to load authors and populate the UI
    let load_authors = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Loading authors from backend...");

                match api_client.get_authors().await {
                    Ok(authors_data) => {
                        println!("Successfully fetched {} authors", authors_data.len());

                        // Convert backend AuthorWithTitleCount to Slint AuthorData
                        let slint_authors: Vec<AuthorData> = authors_data
                            .iter()
                            .map(|a| AuthorData {
                                id: a.author.id.clone().into(),
                                first_name: a.author.first_name.clone().into(),
                                last_name: a.author.last_name.clone().into(),
                                biography: a.author.biography.clone().unwrap_or_default().into(),
                                birth_date: a.author.birth_date.clone().unwrap_or_default().into(),
                                death_date: a.author.death_date.clone().unwrap_or_default().into(),
                                nationality: a.author.nationality.clone().unwrap_or_default().into(),
                                website_url: a.author.website_url.clone().unwrap_or_default().into(),
                                title_count: a.title_count as i32,
                            })
                            .collect();

                        // Update the UI with the authors
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_authors));
                            ui.set_authors(model.into());
                            println!("UI updated with authors");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch authors: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Function to load publishers and populate the UI
    let load_publishers = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Loading publishers from backend...");

                match api_client.get_publishers().await {
                    Ok(publishers_data) => {
                        println!("Successfully fetched {} publishers", publishers_data.len());

                        // Convert backend PublisherWithTitleCount to Slint PublisherData
                        let slint_publishers: Vec<PublisherData> = publishers_data
                            .iter()
                            .map(|p| PublisherData {
                                id: p.publisher.id.clone().into(),
                                name: p.publisher.name.clone().into(),
                                description: p.publisher.description.clone().unwrap_or_default().into(),
                                website_url: p.publisher.website_url.clone().unwrap_or_default().into(),
                                country: p.publisher.country.clone().unwrap_or_default().into(),
                                founded_year: p.publisher.founded_year.unwrap_or(0),
                                title_count: p.title_count as i32,
                            })
                            .collect();

                        // Convert to PublisherItem for dropdown usage in TitlesPage
                        let publisher_items: Vec<PublisherItem> = publishers_data
                            .iter()
                            .map(|p| PublisherItem {
                                id: p.publisher.id.clone().into(),
                                name: p.publisher.name.clone().into(),
                            })
                            .collect();

                        // Extract publisher names for ComboBox model
                        // Include "(No publisher)" as first element
                        let mut publisher_names: Vec<slint::SharedString> = vec!["(No publisher)".into()];
                        publisher_names.extend(
                            publishers_data
                                .iter()
                                .map(|p| p.publisher.name.clone().into())
                        );

                        // Update the UI with the publishers
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_publishers));
                            ui.set_publishers(model.into());
                            let items_model = Rc::new(slint::VecModel::from(publisher_items));
                            ui.set_publisher_items(items_model.into());
                            let names_model = Rc::new(slint::VecModel::from(publisher_names));
                            ui.set_publisher_names(names_model.into());
                            println!("UI updated with publishers");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch publishers: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Function to load genres and populate the UI
    let load_genres = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Loading genres from backend...");

                match api_client.get_genres().await {
                    Ok(genres_data) => {
                        println!("Successfully fetched {} genres", genres_data.len());

                        // Convert backend GenreWithTitleCount to Slint GenreData
                        let slint_genres: Vec<GenreData> = genres_data
                            .iter()
                            .map(|g| GenreData {
                                id: g.genre.id.clone().into(),
                                name: g.genre.name.clone().into(),
                                description: g.genre.description.clone().unwrap_or_default().into(),
                                title_count: g.title_count as i32,
                            })
                            .collect();

                        // Convert to GenreItem for dropdown usage in TitlesPage
                        let genre_items: Vec<GenreItem> = genres_data
                            .iter()
                            .map(|g| GenreItem {
                                id: g.genre.id.clone().into(),
                                name: g.genre.name.clone().into(),
                            })
                            .collect();

                        // Extract genre names for ComboBox model
                        let genre_names: Vec<slint::SharedString> = genres_data
                            .iter()
                            .map(|g| g.genre.name.clone().into())
                            .collect();

                        // Update the UI with the genres
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_genres));
                            ui.set_genres(model.into());
                            let items_model = Rc::new(slint::VecModel::from(genre_items));
                            ui.set_genre_items(items_model.into());
                            let names_model = Rc::new(slint::VecModel::from(genre_names));
                            ui.set_genre_names(names_model.into());
                            println!("UI updated with genres");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch genres: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Connect the load-titles callback
    {
        let load_titles = load_titles.clone();
        ui.on_load_titles(move || {
            load_titles();
        });
    }

    // Connect the search-titles callback
    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();
        ui.on_search_titles(move |search_text, title, subtitle, isbn, series_id, author_id, genre_id, publisher_id, year_from, year_to, language, dewey_code, has_volumes, wishlist_only, available_only, location_id| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            
            // Clone arguments to move into async block
            let search_text = search_text.clone();
            let title = title.clone();
            let subtitle = subtitle.clone();
            let isbn = isbn.clone();
            let series_id = series_id.clone();
            let author_id = author_id.clone();
            let genre_id = genre_id.clone();
            let publisher_id = publisher_id.clone();
            let year_from = year_from.clone();
            let year_to = year_to.clone();
            let language = language.clone();
            let dewey_code = dewey_code.clone();
            let location_id = location_id.clone();

            slint::spawn_local(async move {
                println!("Searching titles with filters:");
                println!("  search_text: {}", search_text);
                println!("  series_id: {}", series_id);
                println!("  has_volumes: {}, wishlist_only: {}", has_volumes, wishlist_only);

                // Convert has_volumes and wishlist_only to the appropriate filter value
                let has_volumes_filter = if wishlist_only {
                    Some(false)  // Wishlist = books without volumes
                } else if has_volumes {
                    Some(true)   // Owned = books with volumes
                } else {
                    None         // Show all
                };

                // Convert available_only bool to Option<bool>
                let available_filter = if available_only {
                    Some(true)
                } else {
                    None
                };

                match api_client.search_titles(
                    if search_text.is_empty() { None } else { Some(search_text.as_str()) },
                    if title.is_empty() { None } else { Some(title.as_str()) },
                    if subtitle.is_empty() { None } else { Some(subtitle.as_str()) },
                    if isbn.is_empty() { None } else { Some(isbn.as_str()) },
                    if series_id.is_empty() { None } else { Some(series_id.as_str()) },
                    if author_id.is_empty() { None } else { Some(author_id.as_str()) },
                    if genre_id.is_empty() { None } else { Some(genre_id.as_str()) },
                    if publisher_id.is_empty() { None } else { Some(publisher_id.as_str()) },
                    if year_from.is_empty() { None } else { Some(year_from.as_str()) },
                    if year_to.is_empty() { None } else { Some(year_to.as_str()) },
                    if language.is_empty() { None } else { Some(language.as_str()) },
                    if dewey_code.is_empty() { None } else { Some(dewey_code.as_str()) },
                    has_volumes_filter,
                    available_filter,
                    if location_id.is_empty() { None } else { Some(location_id.as_str()) },
                ).await {
                    Ok(titles) => {
                        println!("Search returned {} titles", titles.len());

                        // Convert backend titles to Slint TitleData format
                        let slint_titles: Vec<TitleData> = titles
                            .into_iter()
                            .map(|t| {
                                TitleData {
                                    id: t.title.id.to_string().into(),
                                    title: t.title.title.into(),
                                    subtitle: t.title.subtitle.unwrap_or_default().into(),
                                    isbn: t.title.isbn.unwrap_or_default().into(),
                                    publisher: t.title.publisher.unwrap_or_default().into(),
                                    publisher_id: t.title.publisher_id.unwrap_or_default().into(),
                                    volume_count: t.volume_count as i32,
                                    language: t.title.language.into(),
                                    publication_year: t.title.publication_year.map(|y| y.to_string()).unwrap_or_default().into(),
                                    pages: t.title.pages.map(|p| p.to_string()).unwrap_or_default().into(),
                                    genre: t.title.genre.unwrap_or_default().into(),
                                    genre_id: t.title.genre_id.unwrap_or_default().into(),
                                    series_name: t.title.series_name.unwrap_or_default().into(),
                                    series_id: t.title.series_id.unwrap_or_default().into(),
                                    series_number: t.title.series_number.unwrap_or_default().into(),
                                    dewey_code: t.title.dewey_code.unwrap_or_default().into(),
                                    summary: t.title.summary.unwrap_or_default().into(),
                                    cover_url: t.title.cover_url.unwrap_or_default().into(),
                                    // Duplicate detection fields (initially false/empty)
                                    is_duplicate: false,
                                    duplicate_pair_id: "".into(),
                                    duplicate_similarity: 0.0,
                                    duplicate_confidence: "".into(),
                                    duplicate_match_reasons: "".into(),
                                    is_duplicate_primary: false,
                                }
                            })
                            .collect();

                        if let Some(ui) = ui_weak.upgrade() {
                            let titles_model = std::rc::Rc::new(slint::VecModel::from(slint_titles));
                            ui.set_titles(titles_model.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to search titles: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the find-duplicates callback
    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();
        ui.on_find_duplicates(move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            slint::spawn_local(async move {
                println!("Finding duplicate titles...");

                match api_client.detect_duplicates(Some(50.0)).await {
                    Ok(result) => {
                        println!(
                            "Found {} duplicate pairs (High: {}, Medium: {}, Low: {})",
                            result.total_pairs,
                            result.high_confidence.len(),
                            result.medium_confidence.len(),
                            result.low_confidence.len()
                        );

                        if let Some(ui) = ui_weak.upgrade() {
                            if result.total_pairs > 0 {
                                // Get current titles from UI
                                let titles_model = ui.get_titles();
                                let mut titles_vec: Vec<TitleData> = Vec::new();
                                for i in 0..titles_model.row_count() {
                                    if let Some(title) = titles_model.row_data(i) {
                                        titles_vec.push(title);
                                    }
                                }

                                // Process all duplicate pairs and mark them in the titles
                                let mut all_pairs = Vec::new();
                                all_pairs.extend(result.high_confidence.iter().map(|p| (p, "High")));
                                all_pairs.extend(result.medium_confidence.iter().map(|p| (p, "Medium")));
                                all_pairs.extend(result.low_confidence.iter().map(|p| (p, "Low")));

                                for (pair, confidence) in all_pairs {
                                    let title1_id = &pair.title1.title.id;
                                    let title2_id = &pair.title2.title.id;

                                    // Mark both titles as duplicates
                                    for title in titles_vec.iter_mut() {
                                        if title.id.as_str() == title1_id {
                                            title.is_duplicate = true;
                                            title.duplicate_pair_id = title2_id.clone().into();
                                            title.duplicate_similarity = pair.similarity_score as f32;
                                            title.duplicate_confidence = confidence.to_string().into();
                                            title.duplicate_match_reasons = pair.match_reasons.join(", ").into();
                                            title.is_duplicate_primary = true;  // First one is primary
                                        } else if title.id.as_str() == title2_id {
                                            title.is_duplicate = true;
                                            title.duplicate_pair_id = title1_id.clone().into();
                                            title.duplicate_similarity = pair.similarity_score as f32;
                                            title.duplicate_confidence = confidence.to_string().into();
                                            title.duplicate_match_reasons = pair.match_reasons.join(", ").into();
                                            title.is_duplicate_primary = false;  // Second one is secondary
                                        }
                                    }
                                }

                                // Update the UI with marked titles
                                let titles_rc = std::rc::Rc::new(slint::VecModel::from(titles_vec));
                                ui.set_titles(titles_rc.into());
                                ui.set_duplicates_detected(true);
                                ui.set_duplicate_count(result.total_pairs as i32);

                                println!("✓ Duplicate titles are now highlighted in the list!");
                                println!("  Use the duplicate filters to show only duplicates.");
                            } else {
                                println!("No duplicates found!");
                                ui.set_duplicates_detected(false);
                                ui.set_duplicate_count(0);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to detect duplicates: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the merge-titles callback
    {
        let api_client = api_client.clone();
        let load_titles = load_titles.clone();
        let ui_weak = ui.as_weak();
        ui.on_merge_titles(move |primary_id, secondary_id| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            let load_titles = load_titles.clone();
            let primary_id = primary_id.clone();
            let secondary_id = secondary_id.clone();

            slint::spawn_local(async move {
                println!("Merging title {} into {}", secondary_id, primary_id);

                match api_client.merge_titles(primary_id.as_str(), secondary_id.as_str()).await {
                    Ok(response) => {
                        println!("✓ Merge successful: {}", response.message);
                        println!("  {} volumes moved", response.volumes_moved);

                        if let Some(ui) = ui_weak.upgrade() {
                            // Clear duplicate flags and reload
                            ui.set_duplicates_detected(false);
                            load_titles();
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to merge titles: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the dismiss-duplicate callback
    {
        let ui_weak = ui.as_weak();
        ui.on_dismiss_duplicate(move |title1_id, title2_id| {
            let ui = ui_weak.unwrap();
            println!("Dismissing duplicate pair: {} <-> {}", title1_id, title2_id);

            // Remove duplicate flags from both titles in the pair
            let titles_model = ui.get_titles();
            let mut titles_vec: Vec<TitleData> = Vec::new();
            for i in 0..titles_model.row_count() {
                if let Some(mut title) = titles_model.row_data(i) {
                    // Clear duplicate flags if this title is part of the dismissed pair
                    if (title.id.as_str() == title1_id.as_str() && title.duplicate_pair_id.as_str() == title2_id.as_str()) ||
                       (title.id.as_str() == title2_id.as_str() && title.duplicate_pair_id.as_str() == title1_id.as_str()) {
                        title.is_duplicate = false;
                        title.duplicate_pair_id = "".into();
                        title.duplicate_similarity = 0.0;
                        title.duplicate_confidence = "".into();
                        title.duplicate_match_reasons = "".into();
                        title.is_duplicate_primary = false;
                    }
                    titles_vec.push(title);
                }
            }

            // Check if there are any remaining duplicates
            let has_duplicates = titles_vec.iter().any(|t| t.is_duplicate);

            // Update UI
            let titles_rc = std::rc::Rc::new(slint::VecModel::from(titles_vec));
            ui.set_titles(titles_rc.into());

            if !has_duplicates {
                ui.set_duplicates_detected(false);
                ui.set_duplicate_count(0);
            }

            println!("✓ Pair dismissed. Titles are no longer marked as duplicates.");
        });
    }

    // Connect the clear-duplicates callback
    {
        let ui_weak = ui.as_weak();
        ui.on_clear_duplicates(move || {
            let ui = ui_weak.unwrap();
            println!("Clearing all duplicate detection results...");

            // Remove all duplicate flags from titles
            let titles_model = ui.get_titles();
            let mut titles_vec: Vec<TitleData> = Vec::new();
            for i in 0..titles_model.row_count() {
                if let Some(mut title) = titles_model.row_data(i) {
                    title.is_duplicate = false;
                    title.duplicate_pair_id = "".into();
                    title.duplicate_similarity = 0.0;
                    title.duplicate_confidence = "".into();
                    title.duplicate_match_reasons = "".into();
                    title.is_duplicate_primary = false;
                    titles_vec.push(title);
                }
            }

            // Update UI
            let titles_rc = std::rc::Rc::new(slint::VecModel::from(titles_vec));
            ui.set_titles(titles_rc.into());
            ui.set_duplicates_detected(false);
            ui.set_duplicate_count(0);

            println!("✓ All duplicate detection results cleared.");
        });
    }

    // Connect the load-locations callback
    {
        let load_locations = load_locations.clone();
        ui.on_load_locations(move || {
            load_locations();
        });
    }

    // Connect the create-location callback
    {
        let load_locations = load_locations.clone();
        let api_client = api_client.clone();
        ui.on_create_location(move |name, description, parent_id| {
            let load_locations = load_locations.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let description = description.clone();
            let parent_id = parent_id.clone();

            slint::spawn_local(async move {
                println!("Creating location: {}", name);

                let request = CreateLocationRequest {
                    name: name.to_string(),
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                    parent_id: if parent_id.is_empty() {
                        None
                    } else {
                        Some(parent_id.to_string())
                    },
                };

                match api_client.create_location(request).await {
                    Ok(id) => {
                        println!("Successfully created location with ID: {}", id);
                        load_locations();
                    }
                    Err(e) => {
                        eprintln!("Failed to create location: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-location callback
    {
        let load_locations = load_locations.clone();
        let api_client = api_client.clone();
        ui.on_update_location(move |id, name, description, parent_id| {
            let load_locations = load_locations.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let description = description.clone();
            let parent_id = parent_id.clone();

            slint::spawn_local(async move {
                println!("Updating location: {}", id);

                let request = UpdateLocationRequest {
                    name: if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    },
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                    parent_id: if parent_id.is_empty() {
                        None
                    } else {
                        Some(parent_id.to_string())
                    },
                };

                match api_client.update_location(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated location");
                        load_locations();
                    }
                    Err(e) => {
                        eprintln!("Failed to update location: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the delete-location callback
    {
        let load_locations = load_locations.clone();
        let api_client = api_client.clone();
        ui.on_delete_location(move |location_id| {
            let load_locations = load_locations.clone();
            let api_client = api_client.clone();
            let location_id = location_id.clone();

            slint::spawn_local(async move {
                println!("Deleting location: {}", location_id);

                match api_client.delete_location(&location_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted location");
                        load_locations();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete location: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the load-authors callback
    {
        let load_authors = load_authors.clone();
        ui.on_load_authors(move || {
            load_authors();
        });
    }

    // Connect the create-author callback
    {
        let load_authors = load_authors.clone();
        let api_client = api_client.clone();
        ui.on_create_author(move |first_name, last_name, biography, birth_date, death_date, nationality, website_url| {
            let load_authors = load_authors.clone();
            let api_client = api_client.clone();
            let first_name = first_name.clone();
            let last_name = last_name.clone();
            let biography = biography.clone();
            let birth_date = birth_date.clone();
            let death_date = death_date.clone();
            let nationality = nationality.clone();
            let website_url = website_url.clone();

            slint::spawn_local(async move {
                println!("Creating author: {} {}", first_name, last_name);

                let request = CreateAuthorRequest {
                    first_name: first_name.to_string(),
                    last_name: last_name.to_string(),
                    biography: if biography.is_empty() {
                        None
                    } else {
                        Some(biography.to_string())
                    },
                    birth_date: if birth_date.is_empty() {
                        None
                    } else {
                        Some(birth_date.to_string())
                    },
                    death_date: if death_date.is_empty() {
                        None
                    } else {
                        Some(death_date.to_string())
                    },
                    nationality: if nationality.is_empty() {
                        None
                    } else {
                        Some(nationality.to_string())
                    },
                    website_url: if website_url.is_empty() {
                        None
                    } else {
                        Some(website_url.to_string())
                    },
                };

                match api_client.create_author(request).await {
                    Ok(id) => {
                        println!("Successfully created author with ID: {}", id);
                        load_authors();
                    }
                    Err(e) => {
                        eprintln!("Failed to create author: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-author callback
    {
        let load_authors = load_authors.clone();
        let api_client = api_client.clone();
        ui.on_update_author(move |id, first_name, last_name, biography, birth_date, death_date, nationality, website_url| {
            let load_authors = load_authors.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let first_name = first_name.clone();
            let last_name = last_name.clone();
            let biography = biography.clone();
            let birth_date = birth_date.clone();
            let death_date = death_date.clone();
            let nationality = nationality.clone();
            let website_url = website_url.clone();

            slint::spawn_local(async move {
                println!("Updating author: {}", id);

                let request = UpdateAuthorRequest {
                    first_name: if first_name.is_empty() {
                        None
                    } else {
                        Some(first_name.to_string())
                    },
                    last_name: if last_name.is_empty() {
                        None
                    } else {
                        Some(last_name.to_string())
                    },
                    biography: if biography.is_empty() {
                        None
                    } else {
                        Some(biography.to_string())
                    },
                    birth_date: if birth_date.is_empty() {
                        None
                    } else {
                        Some(birth_date.to_string())
                    },
                    death_date: if death_date.is_empty() {
                        None
                    } else {
                        Some(death_date.to_string())
                    },
                    nationality: if nationality.is_empty() {
                        None
                    } else {
                        Some(nationality.to_string())
                    },
                    website_url: if website_url.is_empty() {
                        None
                    } else {
                        Some(website_url.to_string())
                    },
                };

                match api_client.update_author(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated author");
                        load_authors();
                    }
                    Err(e) => {
                        eprintln!("Failed to update author: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the delete-author callback
    {
        let load_authors = load_authors.clone();
        let api_client = api_client.clone();
        ui.on_delete_author(move |author_id| {
            let load_authors = load_authors.clone();
            let api_client = api_client.clone();
            let author_id = author_id.clone();

            slint::spawn_local(async move {
                println!("Deleting author: {}", author_id);

                match api_client.delete_author(&author_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted author");
                        load_authors();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete author: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the load-publishers callback
    {
        let load_publishers = load_publishers.clone();
        ui.on_load_publishers(move || {
            load_publishers();
        });
    }

    // Connect the create-publisher callback
    {
        let load_publishers = load_publishers.clone();
        let api_client = api_client.clone();
        ui.on_create_publisher(move |name, description, website_url, country, founded_year| {
            let load_publishers = load_publishers.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let description = description.clone();
            let website_url = website_url.clone();
            let country = country.clone();
            let founded_year = founded_year.clone();

            slint::spawn_local(async move {
                println!("Creating publisher: {}", name);

                let request = CreatePublisherRequest {
                    name: name.to_string(),
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                    website_url: if website_url.is_empty() {
                        None
                    } else {
                        Some(website_url.to_string())
                    },
                    country: if country.is_empty() {
                        None
                    } else {
                        Some(country.to_string())
                    },
                    founded_year: if founded_year.is_empty() {
                        None
                    } else {
                        founded_year.parse::<i32>().ok()
                    },
                };

                match api_client.create_publisher(request).await {
                    Ok(id) => {
                        println!("Successfully created publisher with ID: {}", id);
                        load_publishers();
                    }
                    Err(e) => {
                        eprintln!("Failed to create publisher: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-publisher callback
    {
        let load_publishers = load_publishers.clone();
        let api_client = api_client.clone();
        ui.on_update_publisher(move |id, name, description, website_url, country, founded_year| {
            let load_publishers = load_publishers.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let description = description.clone();
            let website_url = website_url.clone();
            let country = country.clone();
            let founded_year = founded_year.clone();

            slint::spawn_local(async move {
                println!("Updating publisher: {}", id);

                let request = UpdatePublisherRequest {
                    name: if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    },
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                    website_url: if website_url.is_empty() {
                        None
                    } else {
                        Some(website_url.to_string())
                    },
                    country: if country.is_empty() {
                        None
                    } else {
                        Some(country.to_string())
                    },
                    founded_year: if founded_year.is_empty() {
                        None
                    } else {
                        founded_year.parse::<i32>().ok()
                    },
                };

                match api_client.update_publisher(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated publisher");
                        load_publishers();
                    }
                    Err(e) => {
                        eprintln!("Failed to update publisher: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the delete-publisher callback
    {
        let load_publishers = load_publishers.clone();
        let api_client = api_client.clone();
        ui.on_delete_publisher(move |publisher_id| {
            let load_publishers = load_publishers.clone();
            let api_client = api_client.clone();
            let publisher_id = publisher_id.clone();

            slint::spawn_local(async move {
                println!("Deleting publisher: {}", publisher_id);

                match api_client.delete_publisher(&publisher_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted publisher");
                        load_publishers();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete publisher: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the load-genres callback
    {
        let load_genres = load_genres.clone();
        ui.on_load_genres(move || {
            load_genres();
        });
    }

    // Connect the create-genre callback
    {
        let load_genres = load_genres.clone();
        let api_client = api_client.clone();
        ui.on_create_genre(move |name, description| {
            let load_genres = load_genres.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Creating genre: {}", name);

                let request = CreateGenreRequest {
                    name: name.to_string(),
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                };

                match api_client.create_genre(request).await {
                    Ok(id) => {
                        println!("Successfully created genre with ID: {}", id);
                        load_genres();
                    }
                    Err(e) => {
                        eprintln!("Failed to create genre: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-genre callback
    {
        let load_genres = load_genres.clone();
        let api_client = api_client.clone();
        ui.on_update_genre(move |id, name, description| {
            let load_genres = load_genres.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Updating genre: {}", id);

                let request = UpdateGenreRequest {
                    name: if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    },
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                };

                match api_client.update_genre(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated genre");
                        load_genres();
                    }
                    Err(e) => {
                        eprintln!("Failed to update genre: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the delete-genre callback
    {
        let load_genres = load_genres.clone();
        let api_client = api_client.clone();
        ui.on_delete_genre(move |genre_id| {
            let load_genres = load_genres.clone();
            let api_client = api_client.clone();
            let genre_id = genre_id.clone();

            slint::spawn_local(async move {
                println!("Deleting genre: {}", genre_id);

                match api_client.delete_genre(&genre_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted genre");
                        load_genres();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete genre: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Function to load series and populate the UI
    let load_series = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();

            slint::spawn_local(async move {
                println!("Loading series from backend...");

                match api_client.get_series().await {
                    Ok(series_data) => {
                        println!("Successfully fetched {} series", series_data.len());

                        // Convert backend SeriesWithTitleCount to Slint SeriesData
                        let slint_series: Vec<SeriesData> = series_data
                            .iter()
                            .map(|s| SeriesData {
                                id: s.series.id.clone().into(),
                                name: s.series.name.clone().into(),
                                description: s.series.description.clone().unwrap_or_default().into(),
                                title_count: s.title_count as i32,
                            })
                            .collect();

                        // Convert to SeriesItem for dropdown usage in TitlesPage
                        let series_items: Vec<SeriesItem> = series_data
                            .iter()
                            .map(|s| SeriesItem {
                                id: s.series.id.clone().into(),
                                name: s.series.name.clone().into(),
                            })
                            .collect();

                        // Extract series names for ComboBox model
                        let series_names: Vec<slint::SharedString> = series_data
                            .iter()
                            .map(|s| s.series.name.clone().into())
                            .collect();

                        // Update the UI with the series
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_series));
                            ui.set_series(model.into());
                            let items_model = Rc::new(slint::VecModel::from(series_items));
                            ui.set_series_items(items_model.into());
                            let names_model = Rc::new(slint::VecModel::from(series_names));
                            ui.set_series_names(names_model.into());
                            println!("UI updated with series");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch series: {}", e);
                        eprintln!("Make sure the backend server is running on http://localhost:8000");
                    }
                }
            }).unwrap();
        }
    };

    // Connect the load-series callback
    {
        let load_series = load_series.clone();
        ui.on_load_series(move || {
            load_series();
        });
    }

    // Connect the create-series callback
    {
        let load_series = load_series.clone();
        let api_client = api_client.clone();
        ui.on_create_series(move |name, description| {
            let load_series = load_series.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Creating series: {}", name);

                let request = CreateSeriesRequest {
                    name: name.to_string(),
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                };

                match api_client.create_series(request).await {
                    Ok(id) => {
                        println!("Successfully created series with ID: {}", id);
                        load_series();
                    }
                    Err(e) => {
                        eprintln!("Failed to create series: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-series callback
    {
        let load_series = load_series.clone();
        let api_client = api_client.clone();
        ui.on_update_series(move |id, name, description| {
            let load_series = load_series.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Updating series: {}", id);

                let request = UpdateSeriesRequest {
                    name: if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    },
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description.to_string())
                    },
                };

                match api_client.update_series(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated series");
                        load_series();
                    }
                    Err(e) => {
                        eprintln!("Failed to update series: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the delete-series callback
    {
        let load_series = load_series.clone();
        let api_client = api_client.clone();
        ui.on_delete_series(move |series_id| {
            let load_series = load_series.clone();
            let api_client = api_client.clone();
            let series_id = series_id.clone();

            slint::spawn_local(async move {
                println!("Deleting series: {}", series_id);

                match api_client.delete_series(&series_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted series");
                        load_series();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete series: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the find-genre-index callback
    {
        let ui_weak = ui.as_weak();
        ui.on_find_genre_index(move |genre_id| {
            if let Some(ui) = ui_weak.upgrade() {
                let genre_items = ui.get_genre_items();
                for (i, item) in genre_items.iter().enumerate() {
                    if item.id == genre_id {
                        return i as i32;
                    }
                }
            }
            -1
        });
    }

    // Connect the find-publisher-index callback
    {
        let ui_weak = ui.as_weak();
        ui.on_find_publisher_index(move |publisher_id| {
            if let Some(ui) = ui_weak.upgrade() {
                let publisher_items = ui.get_publisher_items();
                for (i, item) in publisher_items.iter().enumerate() {
                    if item.id == publisher_id {
                        return i as i32;
                    }
                }
            }
            -1
        });
    }

    // Connect the find-location-index callback
    {
        let ui_weak = ui.as_weak();
        ui.on_find_location_index(move |location_id| {
            if let Some(ui) = ui_weak.upgrade() {
                let locations = ui.get_locations();
                // Index 0 is "(No location)", so we start from 1
                for (i, location) in locations.iter().enumerate() {
                    if location.id == location_id {
                        return (i + 1) as i32;  // +1 because index 0 is "(No location)"
                    }
                }
            }
            0  // Return 0 for "(No location)" if not found
        });
    }

    // Connect the find-series-index callback
    {
        let ui_weak = ui.as_weak();
        ui.on_find_series_index(move |series_id| {
            if let Some(ui) = ui_weak.upgrade() {
                let series_items = ui.get_series_items();
                for (i, item) in series_items.iter().enumerate() {
                    if item.id == series_id {
                        return i as i32;
                    }
                }
            }
            -1
        });
    }

    // Connect the create-title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_create_title(move |title, subtitle, isbn, publisher, publisher_id, publication_year, pages, language, genre_id, series_id, series_number, summary, cover_url, dewey_code| {
            let load_titles = load_titles.clone();
            let api_client = api_client.clone();
            let title = title.clone();
            let subtitle = subtitle.clone();
            let isbn = isbn.clone();
            let publisher = publisher.clone();
            let publisher_id = publisher_id.clone();
            let publication_year = publication_year.clone();
            let pages = pages.clone();
            let language = language.clone();
            let genre_id = genre_id.clone();
            let series_id = series_id.clone();
            let series_number = series_number.clone();
            let summary = summary.clone();
            let cover_url = cover_url.clone();
            let dewey_code = dewey_code.clone();

            slint::spawn_local(async move {
                println!("Creating title: {}", title);

                let request = CreateTitleRequest {
                    title: title.to_string(),
                    subtitle: if subtitle.is_empty() {
                        None
                    } else {
                        Some(subtitle.to_string())
                    },
                    isbn: if isbn.is_empty() {
                        None
                    } else {
                        Some(isbn.to_string())
                    },
                    publisher: if publisher.is_empty() {
                        None
                    } else {
                        Some(publisher.to_string())
                    },
                    publisher_id: if publisher_id.is_empty() {
                        None
                    } else {
                        Some(publisher_id.to_string())
                    },
                    publication_year: if publication_year.is_empty() {
                        None
                    } else {
                        publication_year.parse::<i32>().ok()
                    },
                    pages: if pages.is_empty() {
                        None
                    } else {
                        pages.parse::<i32>().ok()
                    },
                    language: language.to_string(),
                    dewey_code: if dewey_code.is_empty() {
                        None
                    } else {
                        Some(dewey_code.to_string())
                    },
                    genre_id: if genre_id.is_empty() {
                        None
                    } else {
                        Some(genre_id.to_string())
                    },
                    series_id: if series_id.is_empty() {
                        None
                    } else {
                        Some(series_id.to_string())
                    },
                    series_number: if series_number.is_empty() {
                        None
                    } else {
                        Some(series_number.to_string())
                    },
                    summary: if summary.is_empty() {
                        None
                    } else {
                        Some(summary.to_string())
                    },
                    cover_url: if cover_url.is_empty() {
                        None
                    } else {
                        Some(cover_url.to_string())
                    },
                };

                match api_client.create_title(request).await {
                    Ok(id) => {
                        println!("Successfully created title with ID: {}", id);
                        load_titles();
                    }
                    Err(e) => {
                        eprintln!("Failed to create title: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the update-title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_update_title(move |id, title, subtitle, isbn, publisher, publisher_id, publication_year, pages, language, genre_id, series_id, series_number, summary, cover_url, dewey_code| {
            let load_titles = load_titles.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let title = title.clone();
            let subtitle = subtitle.clone();
            let isbn = isbn.clone();
            let publisher = publisher.clone();
            let publisher_id = publisher_id.clone();
            let publication_year = publication_year.clone();
            let pages = pages.clone();
            let language = language.clone();
            let genre_id = genre_id.clone();
            let series_id = series_id.clone();
            let series_number = series_number.clone();
            let summary = summary.clone();
            let cover_url = cover_url.clone();
            let dewey_code = dewey_code.clone();

            slint::spawn_local(async move {
                println!("Updating title: {}", id);

                let request = UpdateTitleRequest {
                    title: if title.is_empty() {
                        None
                    } else {
                        Some(title.to_string())
                    },
                    subtitle: if subtitle.is_empty() {
                        None
                    } else {
                        Some(subtitle.to_string())
                    },
                    isbn: if isbn.is_empty() {
                        None
                    } else {
                        Some(isbn.to_string())
                    },
                    publisher: if publisher.is_empty() {
                        None
                    } else {
                        Some(publisher.to_string())
                    },
                    publisher_id: if publisher_id.is_empty() {
                        None
                    } else {
                        Some(publisher_id.to_string())
                    },
                    publication_year: if publication_year.is_empty() {
                        None
                    } else {
                        publication_year.parse::<i32>().ok()
                    },
                    pages: if pages.is_empty() {
                        None
                    } else {
                        pages.parse::<i32>().ok()
                    },
                    language: if language.is_empty() {
                        None
                    } else {
                        Some(language.to_string())
                    },
                    dewey_code: if dewey_code.is_empty() {
                        None
                    } else {
                        Some(dewey_code.to_string())
                    },
                    genre_id: if genre_id.is_empty() {
                        None
                    } else {
                        Some(genre_id.to_string())
                    },
                    series_id: if series_id.is_empty() {
                        None
                    } else {
                        Some(series_id.to_string())
                    },
                    series_number: if series_number.is_empty() {
                        None
                    } else {
                        Some(series_number.to_string())
                    },
                    summary: if summary.is_empty() {
                        None
                    } else {
                        Some(summary.to_string())
                    },
                    cover_url: if cover_url.is_empty() {
                        None
                    } else {
                        Some(cover_url.to_string())
                    },
                };

                match api_client.update_title(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated title");
                        load_titles();
                    }
                    Err(e) => {
                        eprintln!("Failed to update title: {}", e);
                    }
                }
            }).unwrap();
        });
    }



    // Handle delete title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_delete_title(move |id| {
            let load_titles = load_titles.clone();
            let api_client = api_client.clone();
            let id = id.clone();

            slint::spawn_local(async move {
                println!("Deleting title: {}", id);

                match api_client.delete_title(&id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted title");
                        load_titles();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete title: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // ==================================================================
    // Volume Callback Handlers
    // ==================================================================

    // Handle load volumes callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_load_volumes(move |title_id| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            let title_id = title_id.clone();

            slint::spawn_local(async move {
                println!("Loading volumes for title: {}", title_id);

                match api_client.get_volumes_for_title(&title_id.to_string()).await {
                    Ok(volumes) => {
                        println!("Successfully fetched {} volumes", volumes.len());

                        // Get locations to lookup location names
                        let locations = match api_client.get_locations().await {
                            Ok(locs) => locs,
                            Err(e) => {
                                eprintln!("Failed to fetch locations: {}", e);
                                Vec::new()
                            }
                        };

                        // Convert volumes to Slint format
                        let slint_volumes: Vec<VolumeData> = volumes
                            .into_iter()
                            .map(|v| {
                                // Lookup location name from ID
                                let location_name = if let Some(ref loc_id) = v.location_id {
                                    locations
                                        .iter()
                                        .find(|loc| &loc.location.id == loc_id)
                                        .map(|loc| loc.full_path.clone())
                                        .unwrap_or_default()
                                } else {
                                    String::new()
                                };

                                VolumeData {
                                    id: v.id.into(),
                                    title_id: v.title_id.into(),
                                    copy_number: v.copy_number,
                                    barcode: v.barcode.into(),
                                    condition: v.condition.to_string().into(),
                                    location_id: v.location_id.unwrap_or_default().into(),
                                    location_name: location_name.into(),
                                    loan_status: v.loan_status.to_string().into(),
                                    individual_notes: v.individual_notes.unwrap_or_default().into(),
                                }
                            })
                            .collect();

                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_volumes));
                            ui.set_volumes(model.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch volumes: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle create volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_create_volume(move |title_id, barcode, condition, location_id, notes| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            let title_id = title_id.clone();
            let barcode = barcode.clone();
            let condition = condition.clone();
            let location_id = location_id.clone();
            let notes = notes.clone();

            slint::spawn_local(async move {
                println!("Creating volume for title: {}", title_id);

                let condition_enum = match condition.as_str() {
                    "Excellent" => models::VolumeCondition::Excellent,
                    "Good" => models::VolumeCondition::Good,
                    "Fair" => models::VolumeCondition::Fair,
                    "Poor" => models::VolumeCondition::Poor,
                    "Damaged" => models::VolumeCondition::Damaged,
                    _ => models::VolumeCondition::Good,
                };

                let request = models::CreateVolumeRequest {
                    title_id: title_id.to_string(),
                    barcode: barcode.to_string(),
                    condition: condition_enum,
                    location_id: if location_id.is_empty() { None } else { Some(location_id.to_string()) },
                    individual_notes: if notes.is_empty() { None } else { Some(notes.to_string()) },
                };

                match api_client.create_volume(request).await {
                    Ok(_) => {
                        println!("Successfully created volume");
                        // Reload volumes for this title
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.invoke_load_volumes(title_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create volume: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle update volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_update_volume(move |id, barcode, condition, location_id, notes| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let barcode = barcode.clone();
            let condition = condition.clone();
            let location_id = location_id.clone();
            let notes = notes.clone();

            slint::spawn_local(async move {
                println!("Updating volume: {}", id);

                let condition_enum = if !condition.is_empty() {
                    Some(match condition.as_str() {
                        "Excellent" => models::VolumeCondition::Excellent,
                        "Good" => models::VolumeCondition::Good,
                        "Fair" => models::VolumeCondition::Fair,
                        "Poor" => models::VolumeCondition::Poor,
                        "Damaged" => models::VolumeCondition::Damaged,
                        _ => models::VolumeCondition::Good,
                    })
                } else {
                    None
                };

                let request = models::UpdateVolumeRequest {
                    barcode: if barcode.is_empty() { None } else { Some(barcode.to_string()) },
                    condition: condition_enum,
                    location_id: if location_id.is_empty() { None } else { Some(location_id.to_string()) },
                    loan_status: None,
                    individual_notes: if notes.is_empty() { None } else { Some(notes.to_string()) },
                };

                match api_client.update_volume(&id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully updated volume");
                        // Reload volumes for the current expanded title
                        if let Some(ui) = ui_weak.upgrade() {
                            let expanded_title_id = ui.get_expanded_title_id();
                            if !expanded_title_id.is_empty() {
                                ui.invoke_load_volumes(expanded_title_id);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to update volume: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle delete volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_delete_volume(move |id| {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();
            let id = id.clone();

            slint::spawn_local(async move {
                println!("Deleting volume: {}", id);

                match api_client.delete_volume(&id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted volume");
                        // Reload volumes for the current expanded title
                        if let Some(ui) = ui_weak.upgrade() {
                            let expanded_title_id = ui.get_expanded_title_id();
                            if !expanded_title_id.is_empty() {
                                ui.invoke_load_volumes(expanded_title_id);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to delete volume: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect author management callbacks
    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();

        ui.on_load_title_authors(move |title_id| {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();
            let title_id = title_id.clone();

            slint::spawn_local(async move {
                println!("Loading authors for title: {}", title_id);

                match api_client.get_title_authors(&title_id.to_string()).await {
                    Ok(authors) => {
                        use slint::Model;
                        use slint::VecModel;

                        if let Some(ui) = ui_weak.upgrade() {
                            let slint_authors: Vec<AuthorWithRoleData> = authors
                                .iter()
                                .map(|a| AuthorWithRoleData {
                                    author_id: a.author.id.clone().into(),
                                    first_name: a.author.first_name.clone().into(),
                                    last_name: a.author.last_name.clone().into(),
                                    role: match a.role {
                                        crate::models::AuthorRole::MainAuthor => "main_author".into(),
                                        crate::models::AuthorRole::CoAuthor => "co_author".into(),
                                        crate::models::AuthorRole::Translator => "translator".into(),
                                        crate::models::AuthorRole::Illustrator => "illustrator".into(),
                                        crate::models::AuthorRole::Editor => "editor".into(),
                                    },
                                    display_order: a.display_order,
                                })
                                .collect();

                            let model = std::rc::Rc::new(VecModel::from(slint_authors));
                            ui.set_title_authors(model.into());
                            println!("Successfully loaded {} authors", authors.len());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to load title authors: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();

        ui.on_load_all_authors(move || {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();

            slint::spawn_local(async move {
                println!("Loading all authors for dropdown");

                match api_client.get_authors().await {
                    Ok(authors) => {
                        use slint::Model;
                        use slint::VecModel;

                        if let Some(ui) = ui_weak.upgrade() {
                            // Create AuthorItem list
                            let author_items: Vec<AuthorItem> = authors
                                .iter()
                                .map(|a| AuthorItem {
                                    id: a.author.id.clone().into(),
                                    name: format!("{} {}", a.author.first_name, a.author.last_name).into(),
                                })
                                .collect();

                            // Create author names list for ComboBox
                            let author_names: Vec<slint::SharedString> = authors
                                .iter()
                                .map(|a| format!("{} {}", a.author.first_name, a.author.last_name).into())
                                .collect();

                            ui.set_all_authors(std::rc::Rc::new(VecModel::from(author_items)).into());
                            ui.set_author_names(std::rc::Rc::new(VecModel::from(author_names)).into());
                            println!("Successfully loaded {} authors", authors.len());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to load authors: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();

        ui.on_add_author_to_title(move |title_id, author_id, role| {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();
            let title_id = title_id.clone();
            let author_id = author_id.clone();
            let role = role.clone();

            slint::spawn_local(async move {
                println!("Adding author {} to title {} with role {}", author_id, title_id, role);

                let role_enum = match role.as_str() {
                    "main_author" => crate::models::AuthorRole::MainAuthor,
                    "co_author" => crate::models::AuthorRole::CoAuthor,
                    "translator" => crate::models::AuthorRole::Translator,
                    "illustrator" => crate::models::AuthorRole::Illustrator,
                    "editor" => crate::models::AuthorRole::Editor,
                    _ => crate::models::AuthorRole::MainAuthor,
                };

                let request = crate::models::AddAuthorToTitleRequest {
                    author_id: author_id.to_string(),
                    role: role_enum,
                    display_order: None,
                };

                match api_client.add_author_to_title(&title_id.to_string(), request).await {
                    Ok(_) => {
                        println!("Successfully added author to title");
                        // Reload the authors list
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.invoke_load_title_authors(title_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to add author to title: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();

        ui.on_remove_author_from_title(move |title_id, author_id| {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();
            let title_id = title_id.clone();
            let author_id = author_id.clone();

            slint::spawn_local(async move {
                println!("Removing author {} from title {}", author_id, title_id);

                match api_client.remove_author_from_title(&title_id.to_string(), &author_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully removed author from title");
                        // Reload the authors list
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.invoke_load_title_authors(title_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to remove author from title: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the upload-image callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_upload_image(move |title_id| {
            let load_titles = load_titles.clone();
            let api_client = api_client.clone();
            let title_id = title_id.clone();

            slint::spawn_local(async move {
                println!("Uploading image for title: {}", title_id);

                // Open file dialog
                let file = rfd::AsyncFileDialog::new()
                    .add_filter("Images", &["jpg", "jpeg", "png", "gif", "webp"])
                    .set_title("Select Cover Image")
                    .pick_file()
                    .await;

                if let Some(file) = file {
                    let filename = file.file_name();
                    let file_data = file.read().await;
                    println!("Selected file: {}", filename);

                    // Upload the image
                    match api_client.upload_cover_image(title_id.to_string(), file_data, filename).await {
                        Ok(_) => {
                            println!("Image uploaded successfully");
                            // Reload titles to show the updated data
                            load_titles();
                        }
                        Err(e) => {
                            eprintln!("Failed to upload image: {}", e);
                        }
                    }
                }
            }).unwrap();
        });
    }

    // Connect the fetch-from-isbn callback (for create dialog)
    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();
        ui.on_fetch_from_isbn(move |isbn| {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();
            let isbn = isbn.clone();

            slint::spawn_local(async move {
                println!("Fetching book data from ISBN (create mode): {}", isbn);

                match api_client.lookup_isbn(isbn.to_string()).await {
                    Ok(book_data) => {
                        println!("Successfully fetched book data: {}", book_data.title);

                        if let Some(ui) = ui_weak.upgrade() {
                            // Populate the "new" form fields
                            ui.set_new_title(book_data.title.into());
                            ui.set_new_subtitle(book_data.subtitle.unwrap_or_default().into());
                            ui.set_new_isbn(book_data.isbn.into());
                            ui.set_new_publisher(book_data.publisher.clone().unwrap_or_default().into());

                            if let Some(year) = book_data.publication_year {
                                ui.set_new_publication_year(year.to_string().into());
                            } else {
                                ui.set_new_publication_year("".into());
                            }

                            if let Some(pages) = book_data.pages {
                                ui.set_new_pages(pages.to_string().into());
                            } else {
                                ui.set_new_pages("".into());
                            }

                            if let Some(language) = &book_data.language {
                                ui.set_new_language(language.clone().into());
                            }

                            ui.set_new_summary(book_data.summary.unwrap_or_default().into());

                            if let Some(cover_data) = &book_data.cover_image_data {
                                println!("Cover image data available ({} bytes base64)", cover_data.len());
                                // TODO: Store this for upload after title creation
                            }

                            println!("Create form fields populated with book data");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch book data: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Connect the fetch-from-isbn-edit callback (for edit dialog)
    {
        let api_client = api_client.clone();
        let ui_weak = ui.as_weak();
        ui.on_fetch_from_isbn_edit(move |isbn| {
            let api_client = api_client.clone();
            let ui_weak = ui_weak.clone();
            let isbn = isbn.clone();

            slint::spawn_local(async move {
                println!("Fetching book data from ISBN (edit mode): {}", isbn);

                match api_client.lookup_isbn(isbn.to_string()).await {
                    Ok(book_data) => {
                        println!("Successfully fetched book data: {}", book_data.title);

                        if let Some(ui) = ui_weak.upgrade() {
                            // Populate the "edit" form fields
                            ui.set_edit_title(book_data.title.into());
                            ui.set_edit_subtitle(book_data.subtitle.unwrap_or_default().into());
                            ui.set_edit_isbn(book_data.isbn.into());
                            ui.set_edit_publisher(book_data.publisher.clone().unwrap_or_default().into());

                            if let Some(year) = book_data.publication_year {
                                ui.set_edit_publication_year(year.to_string().into());
                            } else {
                                ui.set_edit_publication_year("".into());
                            }

                            if let Some(pages) = book_data.pages {
                                ui.set_edit_pages(pages.to_string().into());
                            } else {
                                ui.set_edit_pages("".into());
                            }

                            if let Some(language) = &book_data.language {
                                ui.set_edit_language(language.clone().into());
                            }

                            ui.set_edit_summary(book_data.summary.unwrap_or_default().into());

                            if let Some(cover_data) = &book_data.cover_image_data {
                                println!("Cover image data available ({} bytes base64)", cover_data.len());
                                // TODO: Store this for upload after title update
                            }

                            println!("Edit form fields populated with book data");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch book data: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Load titles on startup
    load_titles();

    // Load locations on startup
    load_locations();

    // Load authors on startup
    load_authors();

    // Load publishers on startup
    load_publishers();

    // Load genres on startup
    load_genres();

    // ========================================================================
    // LOAN MANAGEMENT: Borrower Groups
    // ========================================================================

    // Function to load borrower groups and populate the UI
    let load_borrower_groups = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();

            slint::spawn_local(async move {
                println!("Loading borrower groups from backend...");

                match api_client.get_borrower_groups().await {
                    Ok(groups_data) => {
                        println!("Successfully fetched {} borrower groups", groups_data.len());
                        for (i, g) in groups_data.iter().enumerate() {
                            println!("  Group {}: {} (ID: {})", i, g.name, g.id);
                        }

                        // Convert backend BorrowerGroup to Slint BorrowerGroupData
                        let slint_groups: Vec<BorrowerGroupData> = groups_data
                            .iter()
                            .map(|g| BorrowerGroupData {
                                id: g.id.clone().into(),
                                name: g.name.clone().into(),
                                loan_duration_days: g.loan_duration_days,
                                description: g.description.clone().unwrap_or_default().into(),
                            })
                            .collect();
                        
                        println!("Converted to {} Slint BorrowerGroupData items", slint_groups.len());

                        // Create group names array for ComboBox
                        let group_names: Vec<slint::SharedString> = groups_data
                            .iter()
                            .map(|g| format!("{} ({} days)", g.name, g.loan_duration_days).into())
                            .collect();

                        // Update the UI
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_groups));
                            ui.set_borrower_groups(model.into());

                            let names_model = Rc::new(slint::VecModel::from(group_names));
                            ui.set_group_names(names_model.into());

                            println!("UI updated with borrower groups via set_borrower_groups");
                            
                            // Verify the update by reading back if possible (not directly possible with set_, but we can log)
                        } else {
                            println!("ERROR: UI weak reference could not be upgraded!");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch borrower groups: {}", e);
                    }
                }
            }).unwrap();
        }
    };

    // Connect load-borrower-groups callback
    {
        let load_borrower_groups = load_borrower_groups.clone();
        ui.on_load_borrower_groups(move || {
            load_borrower_groups();
        });
    }

    // Handle create borrower group callback
    {
        let load_borrower_groups = load_borrower_groups.clone();
        let api_client = api_client.clone();
        ui.on_create_borrower_group(move |name, loan_duration_days, description| {
            let load_borrower_groups = load_borrower_groups.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Creating borrower group: {}", name);

                let request = CreateBorrowerGroupRequest {
                    name: name.to_string(),
                    loan_duration_days: loan_duration_days as i32,
                    description: if description.is_empty() { None } else { Some(description.to_string()) },
                };

                match api_client.create_borrower_group(&request).await {
                    Ok(id) => {
                        println!("Successfully created borrower group with ID: {}", id);
                        load_borrower_groups();
                    }
                    Err(e) => {
                        eprintln!("Failed to create borrower group: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle update borrower group callback
    {
        let load_borrower_groups = load_borrower_groups.clone();
        let api_client = api_client.clone();
        ui.on_update_borrower_group(move |id, name, loan_duration_days, description| {
            let load_borrower_groups = load_borrower_groups.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let description = description.clone();

            slint::spawn_local(async move {
                println!("Updating borrower group: {}", id);

                let request = UpdateBorrowerGroupRequest {
                    name: if name.is_empty() { None } else { Some(name.to_string()) },
                    loan_duration_days: Some(loan_duration_days as i32),
                    description: if description.is_empty() { None } else { Some(description.to_string()) },
                };

                match api_client.update_borrower_group(&id.to_string(), &request).await {
                    Ok(_) => {
                        println!("Successfully updated borrower group");
                        load_borrower_groups();
                    }
                    Err(e) => {
                        eprintln!("Failed to update borrower group: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle delete borrower group callback
    {
        let load_borrower_groups = load_borrower_groups.clone();
        let api_client = api_client.clone();
        ui.on_delete_borrower_group(move |id| {
            let load_borrower_groups = load_borrower_groups.clone();
            let api_client = api_client.clone();
            let id = id.clone();

            slint::spawn_local(async move {
                println!("Deleting borrower group: {}", id);

                match api_client.delete_borrower_group(&id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted borrower group");
                        load_borrower_groups();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete borrower group: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // ========================================================================
    // LOAN MANAGEMENT: Borrowers
    // ========================================================================

    // Function to load borrowers and populate the UI
    let load_borrowers = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();

            slint::spawn_local(async move {
                println!("Loading borrowers from backend...");

                match api_client.get_borrowers().await {
                    Ok(borrowers_data) => {
                        println!("Successfully fetched {} borrowers", borrowers_data.len());

                        // Convert backend BorrowerWithGroup to Slint BorrowerData
                        let slint_borrowers: Vec<BorrowerData> = borrowers_data
                            .iter()
                            .map(|b| BorrowerData {
                                id: b.borrower.id.clone().into(),
                                name: b.borrower.name.clone().into(),
                                email: b.borrower.email.clone().unwrap_or_default().into(),
                                phone: b.borrower.phone.clone().unwrap_or_default().into(),
                                address: b.borrower.address.clone().unwrap_or_default().into(),
                                city: b.borrower.city.clone().unwrap_or_default().into(),
                                zip: b.borrower.zip.clone().unwrap_or_default().into(),
                                group_id: b.borrower.group_id.clone().unwrap_or_default().into(),
                                group_name: b.group_name.clone().unwrap_or_default().into(),
                                loan_duration_days: b.loan_duration_days.unwrap_or(21),
                                active_loan_count: b.active_loan_count,
                            })
                            .collect();

                        // Create borrower names array for ComboBox
                        let borrower_names: Vec<slint::SharedString> = borrowers_data
                            .iter()
                            .map(|b| {
                                let group_name = b.group_name.clone().unwrap_or_else(|| "No Group".to_string());
                                format!("{} ({})", b.borrower.name, group_name).into()
                            })
                            .collect();

                        // Update the UI
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_borrowers));
                            ui.set_borrowers(model.into());

                            let names_model = Rc::new(slint::VecModel::from(borrower_names));
                            ui.set_borrower_names(names_model.into());

                            println!("UI updated with borrowers");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch borrowers: {}", e);
                    }
                }
            }).unwrap();
        }
    };

    // Connect load-borrowers callback
    {
        let load_borrowers = load_borrowers.clone();
        ui.on_load_borrowers(move || {
            load_borrowers();
        });
    }

    // Handle create borrower callback
    {
        let load_borrowers = load_borrowers.clone();
        let api_client = api_client.clone();
        ui.on_create_borrower(move |name, email, phone, address, city, zip, group_id| {
            let load_borrowers = load_borrowers.clone();
            let api_client = api_client.clone();
            let name = name.clone();
            let email = email.clone();
            let phone = phone.clone();
            let address = address.clone();
            let city = city.clone();
            let zip = zip.clone();
            let group_id = group_id.clone();

            slint::spawn_local(async move {
                println!("Creating borrower: {}", name);

                let request = CreateBorrowerRequest {
                    name: name.to_string(),
                    email: if email.is_empty() { None } else { Some(email.to_string()) },
                    phone: if phone.is_empty() { None } else { Some(phone.to_string()) },
                    address: if address.is_empty() { None } else { Some(address.to_string()) },
                    city: if city.is_empty() { None } else { Some(city.to_string()) },
                    zip: if zip.is_empty() { None } else { Some(zip.to_string()) },
                    group_id: if group_id.is_empty() { None } else { Some(group_id.to_string()) },
                };

                match api_client.create_borrower(&request).await {
                    Ok(id) => {
                        println!("Successfully created borrower with ID: {}", id);
                        load_borrowers();
                    }
                    Err(e) => {
                        eprintln!("Failed to create borrower: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle update borrower callback
    {
        let load_borrowers = load_borrowers.clone();
        let api_client = api_client.clone();
        ui.on_update_borrower(move |id, name, email, phone, address, city, zip, group_id| {
            let load_borrowers = load_borrowers.clone();
            let api_client = api_client.clone();
            let id = id.clone();
            let name = name.clone();
            let email = email.clone();
            let phone = phone.clone();
            let address = address.clone();
            let city = city.clone();
            let zip = zip.clone();
            let group_id = group_id.clone();

            slint::spawn_local(async move {
                println!("Updating borrower: {}", id);

                let request = UpdateBorrowerRequest {
                    name: if name.is_empty() { None } else { Some(name.to_string()) },
                    email: if email.is_empty() { None } else { Some(email.to_string()) },
                    phone: if phone.is_empty() { None } else { Some(phone.to_string()) },
                    address: if address.is_empty() { None } else { Some(address.to_string()) },
                    city: if city.is_empty() { None } else { Some(city.to_string()) },
                    zip: if zip.is_empty() { None } else { Some(zip.to_string()) },
                    group_id: if group_id.is_empty() { None } else { Some(group_id.to_string()) },
                };

                match api_client.update_borrower(&id.to_string(), &request).await {
                    Ok(_) => {
                        println!("Successfully updated borrower");
                        load_borrowers();
                    }
                    Err(e) => {
                        eprintln!("Failed to update borrower: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle delete borrower callback
    {
        let load_borrowers = load_borrowers.clone();
        let api_client = api_client.clone();
        ui.on_delete_borrower(move |id| {
            let load_borrowers = load_borrowers.clone();
            let api_client = api_client.clone();
            let id = id.clone();

            slint::spawn_local(async move {
                println!("Deleting borrower: {}", id);

                match api_client.delete_borrower(&id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully deleted borrower");
                        load_borrowers();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete borrower: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // ========================================================================
    // LOAN MANAGEMENT: Loans
    // ========================================================================

    // Function to load active loans and populate the UI
    let load_active_loans = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            let ui_weak = ui_weak.clone();
            let api_client = api_client.clone();

            slint::spawn_local(async move {
                println!("Loading active loans from backend...");

                match api_client.get_active_loans().await {
                    Ok(loans_data) => {
                        println!("Successfully fetched {} active loans", loans_data.len());

                        // Convert backend LoanDetail to Slint LoanData
                        let slint_loans: Vec<LoanData> = loans_data
                            .iter()
                            .map(|l| LoanData {
                                id: l.loan.id.to_string().into(),
                                title: l.title.clone().into(),
                                barcode: l.barcode.clone().into(),
                                borrower_name: l.borrower_name.clone().into(),
                                borrower_email: l.borrower_email.clone().unwrap_or_default().into(),
                                loan_date: l.loan.loan_date.format("%Y-%m-%d").to_string().into(),
                                due_date: l.loan.due_date.format("%Y-%m-%d").to_string().into(),
                                extension_count: l.loan.extension_count,
                                is_overdue: l.is_overdue,
                            })
                            .collect();

                        // Update the UI
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = Rc::new(slint::VecModel::from(slint_loans));
                            ui.set_active_loans(model.into());
                            println!("UI updated with active loans");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch active loans: {}", e);
                    }
                }
            }).unwrap();
        }
    };

    // Connect load-active-loans callback
    {
        let load_active_loans = load_active_loans.clone();
        ui.on_load_active_loans(move || {
            load_active_loans();
        });
    }

    // Handle create loan callback
    {
        let load_active_loans = load_active_loans.clone();
        let api_client = api_client.clone();
        ui.on_create_loan(move |borrower_id, barcode| {
            let load_active_loans = load_active_loans.clone();
            let api_client = api_client.clone();
            let borrower_id = borrower_id.clone();
            let barcode = barcode.clone();

            slint::spawn_local(async move {
                println!("Creating loan - Borrower: {}, Barcode: {}", borrower_id, barcode);

                let request = CreateLoanRequest {
                    borrower_id: borrower_id.to_string(),
                    barcode: barcode.to_string(),
                };

                match api_client.create_loan_by_barcode(&request).await {
                    Ok(response) => {
                        let due_date = chrono::DateTime::from_timestamp(response.due_date, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                        println!("Successfully created loan with ID: {}", response.id);
                        println!("Due date: {}, Duration: {} days", due_date, response.loan_duration_days);
                        load_active_loans();
                    }
                    Err(e) => {
                        eprintln!("Failed to create loan: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle return loan callback
    {
        let load_active_loans = load_active_loans.clone();
        let api_client = api_client.clone();
        ui.on_return_loan(move |loan_id| {
            let load_active_loans = load_active_loans.clone();
            let api_client = api_client.clone();
            let loan_id = loan_id.clone();

            slint::spawn_local(async move {
                println!("Returning loan: {}", loan_id);

                match api_client.return_loan(&loan_id.to_string()).await {
                    Ok(_) => {
                        println!("Successfully returned loan");
                        load_active_loans();
                    }
                    Err(e) => {
                        eprintln!("Failed to return loan: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // Handle extend loan callback
    {
        let load_active_loans = load_active_loans.clone();
        let api_client = api_client.clone();
        ui.on_extend_loan(move |loan_id| {
            let load_active_loans = load_active_loans.clone();
            let api_client = api_client.clone();
            let loan_id = loan_id.clone();

            slint::spawn_local(async move {
                println!("Extending loan: {}", loan_id);

                match api_client.extend_loan(&loan_id.to_string()).await {
                    Ok(updated_loan) => {
                        println!("Successfully extended loan. New due date: {}",
                                 updated_loan.loan.due_date.format("%Y-%m-%d"));
                        load_active_loans();
                    }
                    Err(e) => {
                        eprintln!("Failed to extend loan: {}", e);
                    }
                }
            }).unwrap();
        });
    }

    // ========================================================================
    // Statistics callbacks
    // ========================================================================

    // Create load_statistics closure
    let load_statistics = {
        let ui_handle = ui.as_weak();
        let api_client = api_client.clone();
        move || {
            let ui_handle = ui_handle.clone();
            let api_client = api_client.clone();

            slint::spawn_local(async move {
                println!("Loading statistics from backend...");

                // Load library statistics
                match api_client.get_library_statistics().await {
                    Ok(stats) => {
                        println!("Successfully fetched library statistics");

                        if let Some(ui) = ui_handle.upgrade() {
                            ui.set_library_statistics(LibraryStatistics {
                                total_titles: stats.total_titles as i32,
                                total_volumes: stats.total_volumes as i32,
                                total_authors: stats.total_authors as i32,
                                total_publishers: stats.total_publishers as i32,
                                total_genres: stats.total_genres as i32,
                                total_locations: stats.total_locations as i32,
                                total_borrowers: stats.total_borrowers as i32,
                                active_loans: stats.active_loans as i32,
                                overdue_loans: stats.overdue_loans as i32,
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch library statistics: {}", e);
                    }
                }

                // Load genre statistics
                match api_client.get_genre_statistics().await {
                    Ok(stats_data) => {
                        println!("Successfully fetched {} genre statistics", stats_data.len());

                        let slint_stats: Vec<GenreStatistic> = stats_data
                            .iter()
                            .map(|s| GenreStatistic {
                                genre_id: s.genre_id.clone().unwrap_or_default().into(),
                                genre_name: s.genre_name.clone().into(),
                                volume_count: s.volume_count as i32,
                                title_count: s.title_count as i32,
                            })
                            .collect();

                        if let Some(ui) = ui_handle.upgrade() {
                            use slint::Model;
                            let model = std::rc::Rc::new(slint::VecModel::from(slint_stats));
                            ui.set_genre_statistics(model.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch genre statistics: {}", e);
                    }
                }

                // Load location statistics
                match api_client.get_location_statistics().await {
                    Ok(stats_data) => {
                        println!("Successfully fetched {} location statistics", stats_data.len());

                        let slint_stats: Vec<LocationStatistic> = stats_data
                            .iter()
                            .map(|s| LocationStatistic {
                                location_id: s.location_id.clone().unwrap_or_default().into(),
                                location_name: s.location_name.clone().into(),
                                location_path: s.location_path.clone().into(),
                                volume_count: s.volume_count as i32,
                            })
                            .collect();

                        if let Some(ui) = ui_handle.upgrade() {
                            let model = std::rc::Rc::new(slint::VecModel::from(slint_stats));
                            ui.set_location_statistics(model.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch location statistics: {}", e);
                    }
                }

                // Load loan statistics
                match api_client.get_loan_statistics().await {
                    Ok(stats_data) => {
                        println!("Successfully fetched {} loan statistics", stats_data.len());

                        let slint_stats: Vec<LoanStatistic> = stats_data
                            .iter()
                            .map(|s| LoanStatistic {
                                status: s.status.clone().into(),
                                count: s.count as i32,
                            })
                            .collect();

                        if let Some(ui) = ui_handle.upgrade() {
                            let model = std::rc::Rc::new(slint::VecModel::from(slint_stats));
                            ui.set_loan_statistics(model.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch loan statistics: {}", e);
                    }
                }
            }).unwrap();
        }
    };

    // Connect load-statistics callback
    {
        let load_statistics = load_statistics.clone();
        ui.on_load_statistics(move || {
            load_statistics();
        });
    }

    // Load loan management data on startup
    load_borrower_groups();
    load_borrowers();
    load_active_loans();

    ui.run()?;

    Ok(())
}
