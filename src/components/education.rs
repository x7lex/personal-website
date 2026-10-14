use super::card::Card;
use crate::content::EDUCATION;
use leptos::prelude::*;

#[component]
pub fn Education() -> impl IntoView {
    view! {
        <Card title="Education" class="education">
            {EDUCATION.iter().map(|e| view! {
                <div class="school">
                    <div class="school-head">
                        <span>{e.school}</span>
                        <span>{e.years}</span>
                    </div>
                    <p class="school-program">{e.program}</p>
                    {e.details.iter().map(|d| view! {
                        <p class="school-detail">"↳ "{*d}</p>
                    }).collect_view()}
                </div>
            }).collect_view()}
        </Card>
    }
}
