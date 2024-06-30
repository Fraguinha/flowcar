use leptos::*;

#[component]
pub fn Error(title: String, subtitle: String, href: String, text: String) -> impl IntoView {
    view! {
        <div class="fullscreen d-flex flex-row align-items-center">
            <div class="container">
                <div class="row justify-content-center">
                    <div class="col-12 text-center">
                        <span class="display-1 d-block">{title}</span>
                        <div class="mb-4 lead">{subtitle}</div>
                        <a href=href class="link-style-light">
                            {text}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
