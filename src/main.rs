mod assets;
mod components;
mod content;

use components::{contact::Contact, readme::Readme, sidebar::Sidebar};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="page">
            <main class="panel">
                <Sidebar/>
                <div class="content">
                    <Readme/>
                    <Contact/>
                </div>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
