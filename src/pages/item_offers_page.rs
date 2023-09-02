use leptos::*;
use leptos_router::*;
// use leptos_image::Image;

use crate::components::page_buttons::PageButtons;
#[component]
pub fn ItemOffersPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    // Getting item id from url
    let item_id = move || params().get("item_id").cloned().unwrap_or("1".to_string()); 
    // Getting page number from url
    let page = move || params().get("page").cloned();
    let parsed_page_num = move || { page().unwrap_or_default().parse::<u32>().unwrap_or_default() };

    view! {
        cx,
        <div class="resources-items-item-offers-page">
            <div class="offers-filter-wrapper">
                <h1>"Item offers filter"</h1>
            </div>
            <div class="offer-item-wrapper">
                <h1>"Item"</h1>
            </div>
            <div class="offers-wrapper">
                <h1>"Item offers"</h1>
            </div>

            // 
            <PageButtons page_url={"/resources/offers/".to_string() + item_id().as_str() + "/" } />
        </div>
    }
}