use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use leptos_image::Image;

use crate::server::items::get_items;

#[component]
pub fn ItemsListPage(cx: Scope) -> impl IntoView {
    // let query: Memo<ParamsMap> = use_query_map(cx);
    // let name = move || query().get("name").cloned().unwrap_or_default();
    // let number = move || query().get("number").cloned().unwrap_or_default();
    // let select = move || query().get("select").cloned().unwrap_or_default();


    view! {
        cx,
        <h2>"Items page"</h2>
        <div class="items-filter-wrapper">
            <ItemsFilter />
        </div>
        <div class="items-list-wrapper">
            <ItemsList />
        </div>

        
        <h2>"page change"</h2>
        <PageButtons />

    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemsQueryData{
    pub page: u32,
}

#[component]
pub fn ItemsList(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let page = move || params().get("page").cloned();
    
    let parsed_page_num = move || { page().unwrap_or_default().parse::<u32>().unwrap_or_default() };

    let query_data = create_memo(cx, move |_| {
        ItemsQueryData{
            page: parsed_page_num()
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
        <h2>"List of items"</h2>

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
                                                        {item.display_name_eng} ", Item Id:" {item.id}
                                                        {}
                                                        <Image
                                                            src={"/items_images/".to_string() + &item.filename}
                                                            width=128
                                                            height=128
                                                            quality=85
                                                            blur=false
                                                            class="test-image"
                                                        />
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

    let name = move || query().get("name").cloned().unwrap_or_default();
    let _number = move || query().get("groupby").cloned().unwrap_or_default();
    let select = move || query().get("sortorder").cloned().unwrap_or_default();
    let color = move || query().get("color").cloned().unwrap_or_default();

    log::info!("{}", color());

    view! {
        cx,
        <h2>"Filter"</h2>
        <Form method="GET" action="">

            <input type="text" name="name" value=name />
            <input type="color" name="color" value=color />
            <select name="sortorder">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A-Z">
                    "A-Z"
                </option>
                <option selected=move || select() == "Z-A">
                    "Z-A"
                </option>
            </select>
            <input type="submit" value="Filtruj"/>
        </Form>
    }
}

#[component]
pub fn PageButtons(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let query_str = move || query().to_query_string();

    let params = use_params_map(cx);
    let page = move || params().get("page").cloned();

    let parsed_page_num = move || { page().unwrap_or_default().parse::<u32>().unwrap_or_default() };

    let page_prefix = move || {
        match page() {
            Some(_) => "../",
            None => "./"
        }
    };

    let prev_page_url = move || {
        if parsed_page_num() > 0{
            page_prefix().to_string() + &(parsed_page_num() - 1).to_string() + &query_str()
        } else {
            "".to_string() + &query_str()
        }
    };

    let next_page_url = move || {
        page_prefix().to_string() + &(parsed_page_num() + 1).to_string() + &query_str()
    };

    view! {
        cx,

        <div>
            <A href={prev_page_url}>"<"</A>
            <p>{parsed_page_num}</p>
            <A href={next_page_url}>">"</A>
        </div>
    }
}