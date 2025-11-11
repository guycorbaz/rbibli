// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;

// Module declarations
mod models;
mod api_client;

use api_client::ApiClient;
use models::{CreateLocationRequest, CreateAuthorRequest, CreatePublisherRequest, UpdatePublisherRequest};

slint::include_modules!();

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
                            title: t.title.title.clone().into(),
                            subtitle: t.title.subtitle.clone().unwrap_or_default().into(),
                            isbn: t.title.isbn.clone().unwrap_or_default().into(),
                            publisher: t.title.publisher.clone().unwrap_or_default().into(),
                            volume_count: t.volume_count as i32,
                            language: t.title.language.clone().into(),
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

                    // Update the UI with the locations
                    if let Some(ui) = ui_weak.upgrade() {
                        let model = Rc::new(slint::VecModel::from(slint_locations));
                        ui.set_locations(model.into());
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

    // Load titles on startup
    load_titles();

    // Load locations on startup
    load_locations();

    // Load authors on startup
    load_authors();

    // Load publishers on startup
    load_publishers();

    ui.run()?;

    Ok(())
}
