use leptos::*;
use leptos_router::*;

#[component]
pub fn ResourcesListingsPage(cx: Scope) -> impl IntoView {
    let query: Memo<ParamsMap> = use_query_map(cx);
    // let name = move || query().get("name").cloned().unwrap_or_default();
    // let number = move || query().get("number").cloned().unwrap_or_default();
    // let select = move || query().get("select").cloned().unwrap_or_default();


    view! {
        cx,
        <h2>"Resources page"</h2>

        // <table>
        //     <tr>
        //         <td><code>"name"</code></td>
        //         <td>{name}</td>
        //     </tr>
        //     <tr>
        //         <td><code>"number"</code></td>
        //         <td>{number}</td>
        //     </tr>
        //     <tr>
        //         <td><code>"select"</code></td>
        //         <td>{select}</td>
        //     </tr>
        //         <tr>
        //         <td><code>"page"</code></td>
        //     </tr>
        // </table>

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
    let number = move || query().get("groupby").cloned().unwrap_or_default();
    let select = move || query().get("sortorder").cloned().unwrap_or_default();

    view! {
        cx,
        <h2>"Filter"</h2>
        // Item Name; Item ID; Item meta; Display Name Eng; Display Name Pl; Filename
        <Form method="GET" action="">

            <input type="text" name="name" value=name/>
            <select name="sortorder">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A-Z">
                    "A-Z"
                </option>
                <option selected=move || select() == "Z-A">
                    "Z-A"
                </option>
            </select>
            // <input type="radio" name="groupby" value="Modyfikacja" />
            // <input type="radio" name="groupby" value="Kolor" />
            <input type="submit" value="Filtruj"/>
        </Form>
    }
}

#[component]
pub fn PageButtons(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
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