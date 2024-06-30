use leptos::*;

#[component]
pub fn Pagination(
    current_page: i32,
    total_pages: i32,
    category: Option<String>,
    fuel: Option<String>,
) -> impl IntoView {
    let pages = (1..=total_pages).collect::<Vec<_>>();
    let query = match (category, fuel) {
        (Some(category), _) => format!("?category={}", category),
        (_, Some(fuel)) => format!("?fuel={}", fuel),
        (_, _) => "".to_string(),
    };

    if total_pages == 0 {
        view! { <>{}</> }
    } else {
        view! {
            <>
                <nav aria-label="Pagination">
                    <ul class="pagination">
                        {if current_page > 1 {
                            view! {
                                <li class="page-item">
                                    <a class="page-link" href=format!("/search/{}{}", current_page - 1, query) aria-label="Previous">
                                        <span aria-hidden="true">{"«"}</span>
                                    </a>
                                </li>
                            }
                        } else {
                            view! {
                                <li class="page-item disabled">
                                    <a class="page-link" href="#" aria-label="Previous">
                                        <span aria-hidden="true">{"«"}</span>
                                    </a>
                                </li>
                            }
                        }}
                        {pages.into_iter().map(|page| {
                            if page == current_page {
                                view! {
                                    <li class="page-item active">
                                        <a class="page-link" href=format!("/search/{}{}", page, query)>{page}</a>
                                    </li>
                                }
                            } else {
                                view! {
                                    <li class="page-item">
                                        <a class="page-link" href=format!("/search/{}{}", page, query)>{page}</a>
                                    </li>
                                }
                            }
                        }).collect::<Vec<_>>()}
                        {if current_page < total_pages {
                            view! {
                                <li class="page-item">
                                    <a class="page-link" href=format!("/search/{}{}", current_page + 1, query) aria-label="Next">
                                        <span aria-hidden="true">{"»"}</span>
                                    </a>
                                </li>
                            }
                        } else {
                            view! {
                                <li class="page-item disabled">
                                    <a class="page-link" href="#" aria-label="Next">
                                        <span aria-hidden="true">{"»"}</span>
                                    </a>
                                </li>
                            }
                        }}
                    </ul>
                </nav>
            </>
        }
    }
}
