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
        <div class="resources-item-offers-page">
            <div class="item-offers-filter-wrapper">
                <ItemOffersFilter />
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

#[component]
pub fn ItemOffersFilter(cx: Scope) -> impl IntoView {

    let query = use_query_map(cx);

    // Name filter
    let item_name = move || query().get("item_name").cloned().unwrap_or_default();
    // Select display language
    let language = move || query().get("language").cloned().unwrap_or_default();
    // Select sorting by Name eng / Name Pl / Minecraft Id / Id 
    let sort_by = move || query().get("sort_by").cloned().unwrap_or_default();
    // Select sort order ASC / DESC
    let sort_order = move || query().get("sort_order").cloned().unwrap_or_default();
    // Advanced color search
    let color_search = move || query().get("use_color_search").cloned().is_some();
    // Color
    let color = move || query().get("color").cloned().unwrap_or("#000000".to_string());
    // Color distance
    let color_distance = move || query().get("color_distance").cloned().unwrap_or_default();

    // Rendering
    let (color_search, set_color_search) = create_signal(cx, color_search());

    view! {
        cx,
        // {move || color_search()}
        <Form method="GET" action="/resources/items">

            // Name filter
            <label for="item_name">"Wyszukaj po nazwie"</label>
            <input type="text" name="item_name" value=item_name />

            <div class="spacer"></div>
            
            // Select display language
            <label class="lang-label" for="language">"Język nazw przedmiotów"</label>
            {move || {
                if language() == "pl" {
                    view! {
                        cx,
                        <label class="radio-container"><input type="radio" name="language" value="eng"  /> "Angielski" <span class="checkmark"></span></label>
                        <label class="radio-container"><input type="radio" name="language" value="pl" checked /> "Polski"<span class="checkmark"></span></label>
                    }
                }else{
                    view! {
                        cx,
                        <label class="radio-container"><input type="radio" name="language" value="eng" checked /> "Angielski" <span class="checkmark"></span></label>
                        <label class="radio-container"><input type="radio" name="language" value="pl" /> "Polski"<span class="checkmark"></span></label>
                    }
                }
            }}

            <div class="spacer"></div>

            // Select sorting by Name eng / Name Pl / Minecraft Id / Id 
            <label for="sort_by">"Sposób sortowania"</label>
            <select name="sort_by">
                <option selected=move || sort_by() == "eng-name" value="eng-name">
                    "Nazwa Przedmiotu Angielski"
                </option>
                <option selected=move || sort_by() == "pl-name" value="pl-name">
                    "Nazwa Przedmiotu Polski"
                </option>
                <option selected=move || sort_by() == "mc-id" value="mc-id">
                    "Minecraft Id"
                </option>
                {move || {
                    if color_search() {
                        view! { cx,
                        <option selected=move || sort_by() == "color-distance" value="color-distance">
                            "Podobieństwo Koloru"
                        </option>
                    }}else{
                        view! { cx, 
                        <option disabled selected=move || sort_by() == "color-distance" value="color-distance">
                            "Podobieństwo Koloru"
                        </option>
                        }
                    }
                }}
                <option selected=move || (sort_by() == "default") |  (sort_by() == "") value="default">
                    "Domyślne"
                </option>
            </select>

            <div class="spacer"></div>

            // Select sort order ASC / DESC
            <label for="sort_by">"Kolejność sortowania"</label>
            <select name="sort_order">
                <option selected=move || (sort_order() == "A-Z") |  (sort_order() == "") value="A-Z">
                    "A-Z | Rosnąco"
                </option>
                <option selected=move || sort_order() == "Z-A" value="Z-A">
                    "Z-A | Malejąco"
                </option>
            </select>

            <div class="spacer"></div>

            // Use Color search
            <div class="color-search-box">
                <label for="use_color_search">"Filtrowania po kolorze"</label>
                <label class="switch">
                    {move || {
                        if color_search() {
                            view! { cx, <input class="color-search-checkbox" type="checkbox" name="use_color_search" checked on:click=move |_| { set_color_search.update(|n| *n = !*n ); } /> }
                        }else{
                            view! { cx, <input class="color-search-checkbox" type="checkbox" name="use_color_search" on:click=move |_| { set_color_search.update(|n| *n = !*n ); } /> }
                        }
                    }}
                    <span class="slider round"></span>
                </label>
            </div>

            <div class="spacer"></div>

            <div class=
            {move || 
                if color_search(){
                    "color-inputs-box show"}
                else{
                    "color-inputs-box hide"
            }}>
                // Color
                <div class="item-color-box">
                    <label for="color">"Kolor przedmiotu"</label>
                    <input type="color" name="color" value=color />
                </div>

                <div class="spacer"></div>

                // Max color distance
                <label for="color_distance">"Maksymalna odległość wybranego koloru do koloru przedmiotu"</label>
                <input class="form_slider" type="range" name="color_distance" value=color_distance min="0" max="127" />

                <div class="spacer"></div>
            </div>    
            // Submit Reset
            <div class="submit-reset">
                <input type="submit" value="Filtruj"/>
                <input type="reset" value="Resetuj"/>
            </div>
        </Form>
    }
}