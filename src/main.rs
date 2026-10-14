mod assets;
mod components;
mod content;

use components::{contact::Contact, readme::Readme, sidebar::Sidebar};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let background = assets::asset("images/background")
        .map(|url| format!("background-image: url('{url}')"));

    view! {
        <div class="backdrop" style=background aria-hidden="true"></div>
        <main class="layout">
            <Sidebar/>
            <div class="content">
                <Readme/>
                <Contact/>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
