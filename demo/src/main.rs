mod components;

use crate::components::App;
use leptos::mount::mount_to_body;
use leptos::view;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! { <App /> }
    })
}
