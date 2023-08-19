// use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_image::*;
use crate::auth::*;
use crate::components::navbar::NavBar;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::resources_listings_page::ResourcesListingsPage;
use crate::pages::items_list_page::ItemsListPage;
// use crate::pages::fallback_page::FallbackPage;

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
    
    // Provide user resource to components
    provide_context(cx, user);
    provide_image_context(cx);
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/axum-exchange.css"/>
        <Title text="Axum exchange"/>
        <Router fallback=|cx| {
            view! {cx, <h1>"Page not found :("</h1>}.into_view(cx)
        }>
        

            <main>
                <NavBar />
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
                    // <Route path="resources/:page" view=|cx| view! { cx, <ResourcesListingsPage /> } />
                    <Route path="/resources" view=|cx| view! { cx, <Outlet />}>
                        <Route path="items/:page?" view=|cx| view! { cx, <ItemsListPage /> } />
                        <Route path="selloffers/:page?" view=|cx| view! { cx, <ResourcesListingsPage /> } />
                        <Route path="offer/:offer_id" view=|cx| view! { cx, <ResourcesListingsPage /> } />
                    </Route>

                </Routes>
            </main>
        </Router>
    }
}
