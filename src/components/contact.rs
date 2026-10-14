use super::{booking::BookingModal, card::Card, icon::Icon};
use crate::content::CONTACT;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    let booking = RwSignal::new(None::<&'static str>);
    let modal_open = RwSignal::new(false);

    view! {
        <Card title="Get in touch." class="contact">
            <div class="buttons">
                {CONTACT.iter().map(|c| {
                    let external = c.href.starts_with("http");
                    let gated = c.gated;
                    let href = c.href;
                    view! {
                        <a
                            class="button"
                            class:wide=c.wide
                            href=c.href
                            target=(external && !gated).then_some("_blank")
                            rel=external.then_some("noopener")
                            on:click=move |e| {
                                if gated {
                                    e.prevent_default();
                                    booking.set(Some(href));
                                    modal_open.set(true);
                                }
                            }
                        >
                            <Icon name=c.icon/>{c.label}
                        </a>
                    }
                }).collect_view()}
            </div>
        </Card>
        <Show when=move || modal_open.get()>
            {move || booking.get().map(|href| view! { <BookingModal open=modal_open href/> })}
        </Show>
    }
}
