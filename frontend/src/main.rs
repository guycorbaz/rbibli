// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;

// Module declarations
mod models;
mod api_client;

use api_client::ApiClient;
use models::{CreateTitleRequest, UpdateTitleRequest, CreateLocationRequest, CreateAuthorRequest, CreatePublisherRequest, UpdatePublisherRequest, CreateGenreRequest, UpdateGenreRequest};
use slint::Model;

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
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Create API client
    let api_client = Rc::new(ApiClient::default());

    // Function to load titles and populate the UI
    let load_titles = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            println!("Loading titles from backend...");

            match api_client.get_titles() {
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
                            volume_count: t.volume_count as i32,
                            language: t.title.language.clone().into(),
                            publication_year: t.title.publication_year.map(|y| y.to_string()).unwrap_or_default().into(),
                            pages: t.title.pages.map(|p| p.to_string()).unwrap_or_default().into(),
                            genre: t.title.genre.clone().unwrap_or_default().into(),
                            genre_id: t.title.genre_id.clone().unwrap_or_default().into(),
                            summary: t.title.summary.clone().unwrap_or_default().into(),
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
        }
    };

    // Function to load locations and populate the UI
    let load_locations = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            println!("Loading locations from backend...");

            match api_client.get_locations() {
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
        }
    };

    // Function to load authors and populate the UI
    let load_authors = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            println!("Loading authors from backend...");

            match api_client.get_authors() {
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
        }
    };

    // Function to load publishers and populate the UI
    let load_publishers = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            println!("Loading publishers from backend...");

            match api_client.get_publishers() {
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

                    // Update the UI with the publishers
                    if let Some(ui) = ui_weak.upgrade() {
                        let model = Rc::new(slint::VecModel::from(slint_publishers));
                        ui.set_publishers(model.into());
                        println!("UI updated with publishers");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch publishers: {}", e);
                    eprintln!("Make sure the backend server is running on http://localhost:8000");
                }
            }
        }
    };

    // Function to load genres and populate the UI
    let load_genres = {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();

        move || {
            println!("Loading genres from backend...");

            match api_client.get_genres() {
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
        }
    };

    // Connect the load-titles callback
    {
        let load_titles = load_titles.clone();
        ui.on_load_titles(move || {
            load_titles();
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

            match api_client.create_location(request) {
                Ok(id) => {
                    println!("Successfully created location with ID: {}", id);
                    load_locations();
                }
                Err(e) => {
                    eprintln!("Failed to create location: {}", e);
                }
            }
        });
    }

    // Connect the delete-location callback
    {
        let load_locations = load_locations.clone();
        let api_client = api_client.clone();
        ui.on_delete_location(move |location_id| {
            println!("Deleting location: {}", location_id);

            match api_client.delete_location(&location_id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted location");
                    load_locations();
                }
                Err(e) => {
                    eprintln!("Failed to delete location: {}", e);
                }
            }
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

            match api_client.create_author(request) {
                Ok(id) => {
                    println!("Successfully created author with ID: {}", id);
                    load_authors();
                }
                Err(e) => {
                    eprintln!("Failed to create author: {}", e);
                }
            }
        });
    }

    // Connect the delete-author callback
    {
        let load_authors = load_authors.clone();
        let api_client = api_client.clone();
        ui.on_delete_author(move |author_id| {
            println!("Deleting author: {}", author_id);

            match api_client.delete_author(&author_id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted author");
                    load_authors();
                }
                Err(e) => {
                    eprintln!("Failed to delete author: {}", e);
                }
            }
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

            match api_client.create_publisher(request) {
                Ok(id) => {
                    println!("Successfully created publisher with ID: {}", id);
                    load_publishers();
                }
                Err(e) => {
                    eprintln!("Failed to create publisher: {}", e);
                }
            }
        });
    }

    // Connect the update-publisher callback
    {
        let load_publishers = load_publishers.clone();
        let api_client = api_client.clone();
        ui.on_update_publisher(move |id, name, description, website_url, country, founded_year| {
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

            match api_client.update_publisher(&id.to_string(), request) {
                Ok(_) => {
                    println!("Successfully updated publisher");
                    load_publishers();
                }
                Err(e) => {
                    eprintln!("Failed to update publisher: {}", e);
                }
            }
        });
    }

    // Connect the delete-publisher callback
    {
        let load_publishers = load_publishers.clone();
        let api_client = api_client.clone();
        ui.on_delete_publisher(move |publisher_id| {
            println!("Deleting publisher: {}", publisher_id);

            match api_client.delete_publisher(&publisher_id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted publisher");
                    load_publishers();
                }
                Err(e) => {
                    eprintln!("Failed to delete publisher: {}", e);
                }
            }
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
            println!("Creating genre: {}", name);

            let request = CreateGenreRequest {
                name: name.to_string(),
                description: if description.is_empty() {
                    None
                } else {
                    Some(description.to_string())
                },
            };

            match api_client.create_genre(request) {
                Ok(id) => {
                    println!("Successfully created genre with ID: {}", id);
                    load_genres();
                }
                Err(e) => {
                    eprintln!("Failed to create genre: {}", e);
                }
            }
        });
    }

    // Connect the update-genre callback
    {
        let load_genres = load_genres.clone();
        let api_client = api_client.clone();
        ui.on_update_genre(move |id, name, description| {
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

            match api_client.update_genre(&id.to_string(), request) {
                Ok(_) => {
                    println!("Successfully updated genre");
                    load_genres();
                }
                Err(e) => {
                    eprintln!("Failed to update genre: {}", e);
                }
            }
        });
    }

    // Connect the delete-genre callback
    {
        let load_genres = load_genres.clone();
        let api_client = api_client.clone();
        ui.on_delete_genre(move |genre_id| {
            println!("Deleting genre: {}", genre_id);

            match api_client.delete_genre(&genre_id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted genre");
                    load_genres();
                }
                Err(e) => {
                    eprintln!("Failed to delete genre: {}", e);
                }
            }
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

    // Connect the create-title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_create_title(move |title, subtitle, isbn, publisher, publication_year, pages, language, genre_id, summary| {
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
                dewey_code: None,
                dewey_category: None,
                genre_id: if genre_id.is_empty() {
                    None
                } else {
                    Some(genre_id.to_string())
                },
                summary: if summary.is_empty() {
                    None
                } else {
                    Some(summary.to_string())
                },
                cover_url: None,
            };

            match api_client.create_title(request) {
                Ok(id) => {
                    println!("Successfully created title with ID: {}", id);
                    load_titles();
                }
                Err(e) => {
                    eprintln!("Failed to create title: {}", e);
                }
            }
        });
    }

    // Connect the update-title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_update_title(move |id, title, subtitle, isbn, publisher, publication_year, pages, language, genre_id, summary| {
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
                dewey_code: None,
                dewey_category: None,
                genre_id: if genre_id.is_empty() {
                    None
                } else {
                    Some(genre_id.to_string())
                },
                summary: if summary.is_empty() {
                    None
                } else {
                    Some(summary.to_string())
                },
                cover_url: None,
            };

            match api_client.update_title(&id.to_string(), request) {
                Ok(_) => {
                    println!("Successfully updated title");
                    load_titles();
                }
                Err(e) => {
                    eprintln!("Failed to update title: {}", e);
                }
            }
        });
    }

    // Handle delete title callback
    {
        let load_titles = load_titles.clone();
        let api_client = api_client.clone();
        ui.on_delete_title(move |id| {
            println!("Deleting title: {}", id);

            match api_client.delete_title(&id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted title");
                    load_titles();
                }
                Err(e) => {
                    eprintln!("Failed to delete title: {}", e);
                }
            }
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
            println!("Loading volumes for title: {}", title_id);

            match api_client.get_volumes_for_title(&title_id.to_string()) {
                Ok(volumes) => {
                    println!("Successfully fetched {} volumes", volumes.len());

                    // Get locations to lookup location names
                    let locations = match api_client.get_locations() {
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

                    let ui = ui_weak.unwrap();
                    let model = Rc::new(slint::VecModel::from(slint_volumes));
                    ui.set_volumes(model.into());
                }
                Err(e) => {
                    eprintln!("Failed to fetch volumes: {}", e);
                }
            }
        });
    }

    // Handle create volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_create_volume(move |title_id, barcode, condition, location_id, notes| {
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

            match api_client.create_volume(request) {
                Ok(_) => {
                    println!("Successfully created volume");
                    // Reload volumes for this title
                    let ui = ui_weak.unwrap();
                    ui.invoke_load_volumes(title_id);
                }
                Err(e) => {
                    eprintln!("Failed to create volume: {}", e);
                }
            }
        });
    }

    // Handle update volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_update_volume(move |id, barcode, condition, location_id, notes| {
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

            match api_client.update_volume(&id.to_string(), request) {
                Ok(_) => {
                    println!("Successfully updated volume");
                    // Reload volumes for the current expanded title
                    let ui = ui_weak.unwrap();
                    let expanded_title_id = ui.get_expanded_title_id();
                    if !expanded_title_id.is_empty() {
                        ui.invoke_load_volumes(expanded_title_id);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to update volume: {}", e);
                }
            }
        });
    }

    // Handle delete volume callback
    {
        let ui_weak = ui.as_weak();
        let api_client = api_client.clone();
        ui.on_delete_volume(move |id| {
            println!("Deleting volume: {}", id);

            match api_client.delete_volume(&id.to_string()) {
                Ok(_) => {
                    println!("Successfully deleted volume");
                    // Reload volumes for the current expanded title
                    let ui = ui_weak.unwrap();
                    let expanded_title_id = ui.get_expanded_title_id();
                    if !expanded_title_id.is_empty() {
                        ui.invoke_load_volumes(expanded_title_id);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to delete volume: {}", e);
                }
            }
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

    ui.run()?;

    Ok(())
}
