use crate::assets;
use leptos::prelude::*;

/// An icon from `assets/icons/`, tinted with the current text colour via a CSS mask
#[component]
pub fn Icon(name: &'static str, #[prop(optional)] label: &'static str) -> impl IntoView {
    let style = assets::icon(name).map(|url| format!("--icon: url('{url}')"));
    let hidden = label.is_empty();
    view! {
        <span
            class="icon"
            style=style
            role=(!hidden).then_some("img")
            aria-label=(!hidden).then_some(label)
            aria-hidden=hidden.then_some("true")
            title=(!hidden).then_some(label)
        ></span>
    }
}
