use leptos::*;

use crate::models::category::Category;
use crate::models::fuel::Fuel;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="navbar bg-light">
            <div class="container-fluid">
                <div
                    class="nav-item"
                    type="button"
                    data-bs-toggle="offcanvas"
                    data-bs-target="#offcanvasNavbar"
                    aria-controls="offcanvasNavbar"
                >
                    <i class="bi bi-list"></i>
                </div>
                <a class="navbar-brand" href="/">
                    {"Flow Car Studio"}
                </a>
                <a class="nav-item" href="/profile">
                    <i class="bi bi-person-fill"></i>
                </a>
                <div
                    class="offcanvas offcanvas-start"
                    tabindex="-1"
                    id="offcanvasNavbar"
                    aria-labelledby="offcanvasNavbarLabel"
                >
                    <div class="offcanvas-header">
                        <h5 class="offcanvas-title" id="offcanvasNavbarLabel">
                            {"Flow Car Studio"}
                        </h5>
                        <div
                            class="btn-close"
                            type="button"
                            data-bs-dismiss="offcanvas"
                            aria-label="Close"
                        ></div>
                    </div>
                    <div class="offcanvas-body">
                        <ul class="navbar-nav justify-content-end flex-grow-1 pe-3">
                            <li class="nav-item dropdown">
                                <div
                                    class="nav-link"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    {"Models"}
                                    <i class="bi bi-chevron-compact-right"></i>
                                </div>
                                <ul class="dropdown-menu">
                                    {Fuel::values()
                                        .into_iter()
                                        .map(|fuel| {
                                            view! {
                                                <li data-bs-dismiss="offcanvas">
                                                    <a
                                                        class="dropdown-item"
                                                        href=format!("/search?{}={}", "fuel", fuel.to_string())
                                                    >
                                                        {fuel.to_string()}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                    <hr class="dropdown-divider"/>
                                    {Category::values()
                                        .into_iter()
                                        .map(|category| {
                                            view! {
                                                <li data-bs-dismiss="offcanvas">
                                                    <a
                                                        class="dropdown-item"
                                                        href=format!("/search?{}={}", "category", category.to_string())
                                                    >
                                                        {category.to_string()}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                    <hr class="dropdown-divider"/>
                                    <li data-bs-dismiss="offcanvas">
                                        <a class="dropdown-item" href="/search">
                                            {"All"}
                                        </a>
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </nav>
    }
}
