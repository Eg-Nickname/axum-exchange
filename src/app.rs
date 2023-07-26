// use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;
use crate::components::navbar::NavBar;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);

    let user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>> = create_resource(
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
    log::warn!("Logout url: {:?}", logout.url());
    // Provide user resource to components
    provide_context(cx, user);

    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/axum-exchange.css"/>
        <Title text="Axum exchange"/>
        <Router fallback=|cx| {
            view! {cx, <h1>"Page not found :("</h1>}.into_view(cx)
        }>
        
            <NavBar />

            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="signup" view=move |cx| view! {
                        cx,
                        <SignupPage action=signup/>
                    }/>
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <LoginPage action=login />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}