// use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

use crate::auth::User;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>(cx).expect("User resource shoud have been provided.");
    view! {
        cx,
        <nav>
            <ul>
                <li><A href="/" class="nav-link">"Strona główna"</A></li>
                <li><A href="/" class="nav-link">"Surowce"</A></li>
                <li><A href="/" class="nav-link">"Nieruchomości"</A></li>
                <li><A href="/" class="nav-link">"Armouer's"</A></li>

                <Transition
                    fallback=move || view! {cx, <span>"Loading..."</span>}
                >
                    {move || {
                        user.read(cx).map(|user| match user {
                            // User error navbar view
                            Err(_e) => view! {cx,
                                <li>
                                    <A href="/login" class="login-link">"Zaloguj się"</A>
                                    // <span>" / "</span>
                                    <A href="/signup" class="signup-link">"Zarejestruj się"</A>
                                </li>
                                // <span>{format!("Login error: {}", e)}</span>
                            }.into_view(cx),

                            // User not logged in view
                            Ok(None) => view! {cx,
                                <li>
                                    <A href="/login" class="login-link">"Zaloguj się"</A>
                                    // <span>" / "</span>
                                    <A href="/signup" class="signup-link">"Zarejestruj się"</A>
                                </li>
                            }.into_view(cx),

                            // User logged in view
                            Ok(Some(user)) => view! {cx,
                                <A href="/settings">"Ustawienia"</A>
                                <span>{format!("Zalogowano jako: {} ({})", user.username, user.id)}</span>
                            }.into_view(cx)
                        })
                    }}
                </Transition>
            </ul>
        </nav>
    }
}