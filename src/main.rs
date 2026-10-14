mod assets;
mod components;
mod content;

use components::{card::Card, contact::Contact, readme::Readme, sidebar::Sidebar};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="layout">
            <Sidebar/>
            <div class="content">
                <Readme/>
                <Contact/>
                <Card title=content::EXTRA_PANEL_TITLE class="extra">
                    <p class="muted">"More to come."</p>
                </Card>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
