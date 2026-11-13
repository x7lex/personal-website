use super::{card::Card, icon::Icon};
use crate::content::CONTACT;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Card title="💬 Feel free to reach out!" class="contact">
            <div class="buttons">
                {CONTACT.iter().map(|c| {
                    let external = c.href.starts_with("http");
                    view! {
                        <a
                            class="button"
                            class:wide=c.wide
                            href=c.href
                            target=external.then_some("_blank")
                            rel=external.then_some("noopener")
                        >
                            <Icon name=c.icon/>
                            <span>{c.label}</span>
                            <span class="go" aria-hidden="true">"↗"</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </Card>
    }
}
