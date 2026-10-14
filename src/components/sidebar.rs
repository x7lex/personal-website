use super::card::Card;
use crate::{content::{EDUCATION, PROFILE}};
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h1 class="name">{PROFILE.name}</h1>
            <p class="meta">{PROFILE.location}</p>
            <p class="meta">{PROFILE.pronouns}</p>
            <hr/>
            <p class="tagline">{PROFILE.tagline}</p>
            <Card title="Education" class="education">
                {EDUCATION.iter().map(|e| view! {
                    <div class="school">
                        <div class="school-head">
                            <span>{e.school}</span>
                            <span>{e.years}</span>
                        </div>
                        <p class="school-program">{e.program}</p>
                        <p class="school-faculty">"↳ "{e.faculty}</p>
                    </div>
                }).collect_view()}
            </Card>
        </aside>
    }
}
