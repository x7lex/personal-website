mod assets;
mod components;
mod content;
mod languages;

use components::{contact::Contact, readme::Readme, sidebar::Sidebar};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let background = assets::asset("images/background")
        .map(|url| format!("background-image: url('{url}')"));

    view! {
        <div class="backdrop" style=background aria-hidden="true"></div>
        <div class="page">
            <main class="panel">
                <Sidebar/>
                <div class="content">
                    <Readme/>
                    <Contact/>
                    <pre class="cats" aria-hidden="true">{include_str!("art/sleeping-cats.txt")}</pre>
                </div>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
