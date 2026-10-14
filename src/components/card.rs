use leptos::prelude::*;

/// A titled panel. `mono` renders the title like a file name.
#[component]
pub fn Card(
    title: &'static str,
    #[prop(optional)] mono: bool,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <section class=format!("card {class}")>
            <h2 class="card-title" class:mono=mono>{title}</h2>
            {children()}
        </section>
    }
}
