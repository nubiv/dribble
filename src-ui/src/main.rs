mod app;
mod landing_page;
mod meeting;
mod rtc;
mod tauri_api;

use app::App;
use leptos::{mount_to_body, view};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
