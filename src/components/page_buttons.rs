use leptos::*;
use leptos_router::*;

/// Component allowing dynamic page generations
/// 
/// You need to provide url with defined parameter named `pages` in router
/// 
/// ```rust <Route path="items/:page?" ... />```
/// 
/// Comonent will generate buttons with links to next and previous page if page >=0 for eg.: `items/0`. `items/2` for page 1 and `items/0`, `items/1` for page 0
#[component]
pub fn PageButtons(
    /// Url of current location on the page eg.: '/resources/items/'
    /// buttons will generate links to '/resources/items/*page number*'
    page_url: String
) -> impl IntoView {
    let query = use_query_map();
    let query_str = move || query().to_query_string().replace("#", "%23");

    let params = use_params_map();
    let page = move || params().get("page").cloned();

    let parsed_page_num = move || { page().unwrap_or_default().parse::<u32>().unwrap_or_default() };

    let (get_page_url, _) = create_signal(page_url);

    let prev_page_url = move || {
        if parsed_page_num() > 0{
            get_page_url() + &(parsed_page_num() - 1).to_string() + &query_str()
        } else {
            get_page_url() + &query_str()
        }
    };

    let next_page_url = move || {
        get_page_url() + &(parsed_page_num() + 1).to_string() + &query_str()
    };

    view! {
        <div class="page-buttons-div">
            <A href={prev_page_url}>"<"</A>
            <p>{parsed_page_num}</p>
            <A href={next_page_url}>">"</A>
        </div>
    }
}