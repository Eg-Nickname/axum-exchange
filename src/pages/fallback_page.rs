use leptos::*;


#[component]
pub fn FallbackPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <p>"Page Not Found"</p>
    }
}