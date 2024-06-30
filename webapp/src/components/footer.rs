use chrono::Datelike;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="position-relative bottom text-center text-bg-light">
            <section class="container">
                <div class="row justify-content-center py-5">
                    <div class="col-auto">
                        <a href="https://www.facebook.com/p/Flow-Car-Studio-100084819075644/">
                            <i class="bi bi-facebook"></i>
                        </a>
                    </div>
                    <div class="col-auto">
                        <a href="https://www.instagram.com/flowcarstudio/">
                            <i class="bi bi-instagram"></i>
                        </a>
                    </div>
                </div>
            </section>
            <div class="container">
                <div class="row justify-content-center">
                    <div class="col-auto p-3 text-bg-light">
                        {format!("© {} Flow Car Studio, created by: ", chrono::Utc::now().year())}
                        <a href="https://fraguinha.com/">
                            {"Fraguinha"}
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
