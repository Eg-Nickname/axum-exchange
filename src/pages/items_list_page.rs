use leptos::*;
use leptos_router::*;
use leptos_image::Image;


use crate::server::items::ItemsQueryData;
use crate::components::page_buttons::PageButtons;
#[component]
pub fn ItemsListPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="resources-items-page">
            <div class="items-filter-wrapper">
                <ItemsFilter />
            </div>
            <div class="items-list-wrapper">
                <ItemsList />
            </div>

            <PageButtons page_url="/resources/items/".to_string() />
        </div>
    }
}

#[component]
pub fn ItemsList(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    // Getting page number from url
    let page = move || params().get("page").cloned();
    let parsed_page_num = move || { page().unwrap_or_default().parse::<u32>().unwrap_or_default() };

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
    let color = move || query().get("color").cloned().unwrap_or_default();
    // Color distance
    let color_distance = move || query().get("color_distance").cloned().unwrap_or_default();


    let query_data = create_memo(cx, move |_| {
        ItemsQueryData{
            page: parsed_page_num(),
            item_name: item_name(),
            language: language(),
            sort_by: sort_by(),
            sort_order: sort_order(),
            color_search: color_search(), 
            color: color(),
            color_distance: color_distance(),
        }
    });

    let items = create_resource(
        cx, 
        query_data, 
        move |query_data| {
            use crate::server::items::get_items;
            get_items(cx, query_data)
        });

    view! {
        cx,
        <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
            {move || {
                let existing_items = {
                    move || {
                        items.read(cx)
                            .map(move |items| match items {
                                Err(e) => {
                                    view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                                }
                                Ok(items) => {
                                    if items.is_empty() {
                                        view! { cx, <p>"No items were found."</p> }.into_view(cx)
                                    } else {
                                        items
                                            .into_iter()
                                            .map(move |item| {
                                                view! {
                                                    cx,
                                                    <li>
                                                        <div class="item">
                                                            <div class="bg"></div>
                                                            <div class="image">
                                                                <Image
                                                                    src={"/items_images/".to_string() + &item.filename}
                                                                    width=128
                                                                    height=128
                                                                    quality=85
                                                                    blur=false
                                                                    class="item-image"
                                                                />
                                                            </div>
                                                            <div class="text">
                                                                <h2>{item.display_name_eng}</h2>
                                                                <h3>{item.display_name_pl}</h3>
                                                                <p>
                                                                    <a>"Minecraft Id:" {item.minecraft_item_id}":"{item.item_meta}</a>
                                                                    <a>"Has NBT: " {item.has_nbt}</a>
                                                                    <a>"Database Id:" {item.id}</a>
                                                                    <A class="link-button-secondary" href={"/resources/offers/".to_string() + &item.id.to_string()}>"Zobacz oferty"</A>
                                                                </p>
                                                            </div>
                                                        </div>
                                                    </li>
                                                }
                                            })
                                            .collect_view(cx)
                                    }
                                }
                            })
                            .unwrap_or_default()
                    }
                };

                view! {
                    cx,
                    <ul>
                        {existing_items}
                    </ul>
                }
            }
        }
        </Transition>
    }
}

#[component]
pub fn ItemsFilter(cx: Scope) -> impl IntoView {

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