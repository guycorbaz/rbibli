// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;

// Module declarations
mod models;
mod api_client;

use api_client::ApiClient;
use models::CreateLocationRequest;

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

    // Load titles on startup
    load_titles();

    // Load locations on startup
    load_locations();

    ui.run()?;

    Ok(())
}
