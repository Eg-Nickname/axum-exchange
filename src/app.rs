// use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);

    let user = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(cx),
    );

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        // sets the document title
        <Title text="Axum exchange"/>
        // content for this welcome page

        <Router fallback=|cx| {
            view! {cx, <h1>"Page not found :("</h1>}.into_view(cx)
        }>

        <header>
                <A href="/"><h1>"My Tasks"</h1></A>
                <Transition
                    fallback=move || view! {cx, <span>"Loading..."</span>}
                >
                {move || {
                    user.read(cx).map(|user| match user {
                        Err(e) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>{format!("Login error: {}", e)}</span>
                        }.into_view(cx),
                        Ok(None) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>"Logged out."</span>
                        }.into_view(cx),
                        Ok(Some(user)) => view! {cx,
                            <A href="/settings">"Settings"</A>", "
                            <span>{format!("Logged in as: {} ({})", user.username, user.id)}</span>
                        }.into_view(cx)
                    })
                }}
                </Transition>
            </header>

            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="signup" view=move |cx| view! {
                        cx,
                        <Signup action=signup/>
                    }/>
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <Login action=login />
                    }/>
                    <Route path="settings" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=logout />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}


#[component]
pub fn Login(
    cx: Scope,
    action: Action<Login, Result<(), ServerFnError>>,
) -> impl IntoView {

    view! {
        cx,
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember" class="auth-input" />
                "Remember me?"
            </label>
            <br/>
            <button type="submit" class="button">"Log In"</button>
        </ActionForm>
    }
}

#[component]
pub fn Signup(
    cx: Scope,
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me?"
                <input type="checkbox" name="remember" class="auth-input" />
            </label>

            <br/>
            <button type="submit" class="button">"Sign Up"</button>
        </ActionForm>
    }
}

#[component]
pub fn Logout(
    cx: Scope,
    action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}