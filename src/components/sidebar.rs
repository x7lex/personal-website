use super::{card::Card, icon::Icon};
use crate::{assets, content::{EDUCATION, PROFILE}};
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <img class="avatar" src=assets::asset("images/pfp") alt="Hadi's profile picture"/>
            <h1 class="name">{PROFILE.name}</h1>
            <p class="meta"><Icon name="location"/>{PROFILE.location}</p>
            <p class="meta pronouns">{PROFILE.pronouns}</p>
            <hr/>
            <p class="tagline">{PROFILE.tagline}</p>
            <Card title="Education" class="education">
                {EDUCATION.iter().map(|e| view! {
                    <div class="school">
                        <Icon name="graduation"/>
                        <div>
                            <p class="school-name">{e.school}</p>
                            <p>{e.program}</p>
                            <p class="muted">{e.faculty}</p>
                            <p class="muted">{e.years}</p>
                        </div>
                    </div>
                }).collect_view()}
            </Card>
        </aside>
    }
}
