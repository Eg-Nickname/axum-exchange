use leptos::*;
use leptos_router::*;

#[component]
pub fn ResourcesListingsPage(cx: Scope) -> impl IntoView {
    let query: Memo<ParamsMap> = use_query_map(cx);
    let name = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();


    view! {
        cx,
        <h2>"Resources page"</h2>

        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
                <tr>
                <td><code>"page"</code></td>
            </tr>
        </table>

        <div class="filter-wrapper">
            <ResourcesFilter />
        </div>
        
        <h2>"page change"</h2>
        <PageButtons />

    }
}
#[component]
pub fn ResourcesFilter(cx: Scope) -> impl IntoView {

    let query = use_query_map(cx);
    let name = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();

    view! {
        cx,
        <h2>"Filter"</h2>

        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit" value="Filtruj"/>
        </Form>
    }
}

#[component]
pub fn PageButtons(cx: Scope) -> impl IntoView {
    let query: Memo<ParamsMap> = use_query_map(cx);
    let querry_str = move || query().to_query_string();

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
            page_prefix().to_string() + &(parsed_page_num() - 1).to_string() + &querry_str()
        } else {
            "".to_string() + &querry_str()
        }
    };

    let next_page_url = move || {
        page_prefix().to_string() + &(parsed_page_num() + 1).to_string() + &querry_str()
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