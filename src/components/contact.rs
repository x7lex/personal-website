use super::{card::Card, icon::Icon};
use crate::content::CONTACT;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Card title="Where to contact me." class="contact">
            <div class="buttons">
                {CONTACT.iter().map(|c| {
                    let external = c.href.starts_with("http");
                    view! {
                        <a
                            class="button"
                            href=c.href
                            target=external.then_some("_blank")
                            rel=external.then_some("noopener")
                        >
                            <Icon name=c.icon/>{c.label}
                        </a>
                    }
                }).collect_view()}
            </div>
        </Card>
    }
}
