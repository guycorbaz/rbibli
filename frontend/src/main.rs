

mod app;
mod components;
mod pages;
mod services;
mod utils;
use crate::app::App;
use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    mount_to_body(App)
}
