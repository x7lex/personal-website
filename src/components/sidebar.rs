use super::{card::Card, education::Education};
use crate::content::{PROFILE, PROJECTS};
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h1 class="name">{PROFILE.name}</h1>
            <p class="meta">{PROFILE.location}</p>
            <p class="meta">{PROFILE.characteristics}</p>
            <Card title="Projects" class="projects">
                {PROJECTS.iter().map(|p| view! {
                    <div class="school">
                        <div class="project-head">
                            <p class="school-program">{p.name}</p>
                            <span class="tag">{p.tag}</span>
                        </div>
                        {p.details.iter().map(|d| view! {
                            <p class="school-detail">"↳ "{*d}</p>
                        }).collect_view()}
                    </div>
                }).collect_view()}
            </Card>
            <Education/>
        </aside>
    }
}
