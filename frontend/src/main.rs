// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;

// Module declarations
mod models;
mod api_client;

use api_client::ApiClient;

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

    // Connect the load-titles callback
    {
        let load_titles = load_titles.clone();
        ui.on_load_titles(move || {
            load_titles();
        });
    }

    // Load titles on startup
    load_titles();

    ui.run()?;

    Ok(())
}
