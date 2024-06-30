use leptos::*;

use crate::components::footer::Footer;
use crate::components::nav::NavBar;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="fullscreen">
            <NavBar/>
        </div>
        <Footer/>
    }
}
