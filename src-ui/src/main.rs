mod app;
mod components;
mod pages;
mod utils;

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
