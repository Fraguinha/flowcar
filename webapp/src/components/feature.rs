use leptos::*;

#[component]
pub fn Feature(icon: String, text: String, vertical: bool) -> impl IntoView {
    view! {
        {if vertical { view! { <><br/></> } } else { view! { <>{}</> } }}
        <i class=icon></i>
        <span class="mx-2">{text}</span>
    }
}
