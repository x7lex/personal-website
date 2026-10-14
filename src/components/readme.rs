use super::{card::Card, icon::Icon};
use crate::{assets, content::{PROFILE, STACK}};
use leptos::prelude::*;

#[component]
pub fn Readme() -> impl IntoView {
    view! {
        <Card title=PROFILE.readme_title mono=true class="readme">
            <div class="readme-body">
                <img class="square-avatar" src=assets::asset("images/pfp") alt=""/>
                <div class="readme-text">
                    <div>
                        <p class="greeting">{PROFILE.greeting}</p>
                        <p class="tagline">{PROFILE.tagline}</p>
                    </div>
                    <ul class="stack" aria-label="Tech stack">
                        {STACK.iter().map(|t| view! {
                            <li><Icon name=t.icon label=t.name/></li>
                        }).collect_view()}
                    </ul>
                    <a class="button primary github" href=PROFILE.github target="_blank" rel="noopener">
                        <Icon name="github"/>"GitHub"
                    </a>
                </div>
            </div>
        </Card>
    }
}
