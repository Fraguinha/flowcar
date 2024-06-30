use leptos::*;

use crate::components::error::Error;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Error
            title="404".to_string()
            subtitle="Not Found".to_string()
            href="/".to_string()
            text="Back to Home Page".to_string()
        />
    }
}
