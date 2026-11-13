use super::card::Card;
use crate::{assets, content::{LANGUAGES, LANGUAGES_CAPTION, PROFILE}};
use leptos::prelude::*;

#[component]
pub fn Readme() -> impl IntoView {
    let summary = LANGUAGES
        .iter()
        .map(|l| format!("{} {:.1}%", l.name, l.share))
        .collect::<Vec<_>>()
        .join(", ");

    view! {
        <Card title=PROFILE.readme_title mono=true class="readme">
            <div class="readme-body">
                <img class="square-avatar" src=assets::asset("images/pfp") alt=""/>
                <div class="readme-text">
                    <div>
                        <p class="greeting">{PROFILE.greeting}</p>
                        <p class="tagline">{PROFILE.tagline}</p>
                    </div>
                    <div class="langs">
                        <p class="langs-caption">{LANGUAGES_CAPTION}</p>
                        <div class="langs-bar" role="img" aria-label=format!("Languages: {summary}")>
                            {LANGUAGES.iter().map(|l| view! {
                                <span
                                    style=format!("--swatch: {}; flex-grow: {}", l.color, l.share)
                                    title=format!("{} {:.1}%", l.name, l.share)
                                ></span>
                            }).collect_view()}
                        </div>
                        <ul class="langs-legend" aria-hidden="true">
                            {LANGUAGES.iter().map(|l| view! {
                                <li style=format!("--swatch: {}", l.color)>
                                    {l.name}
                                    <span class="share">{format!("{:.1}%", l.share)}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>
                </div>
            </div>
        </Card>
    }
}
