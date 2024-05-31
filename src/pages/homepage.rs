use leptos::*;
// use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! { c
        <h1>"Kupuj Sprzedawaj "<span class="color-effect">"Zarabiaj"</span></h1>

        <h2>"Chcesz sprzedać swoje przedmioty w wygodny sposób. Porównać oferty różnych sprzedawców. Zakupić działki budynki lub mieszkania. Wszystko to w jednym miejscu w łatwy i wygodny sposób. "</h2>
        // <span class="color-effect">"Testowy smieszny kolorek"</span>
        // <button on:click=on_click>"Click Me: " {count}</button>
    }
}