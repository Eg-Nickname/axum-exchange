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
use crate::pages::item_offers_page::ItemOffersPage;
// use crate::pages::fallback_page::FallbackPage;

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>> = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );
    
    // Provide user resource to components
    provide_context(user);
    provide_image_context();
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/axum-exchange.css"/>
        <Title text="Axum exchange"/>
        <Router fallback=|| {
            view! { <h1>"Page not found :("</h1>}.into_view()
        }>
        

            <main>
                <NavBar />
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                    <Route path="signup" view=move || view! {
                        <SignupPage action=signup/>
                    }/>
                    <Route path="login" view=move || view! {
                        <LoginPage action=login />
                    }/>
                    // <Route path="resources/:page" view=|cx| view! { <ResourcesListingsPage /> } />
                    <Route path="/resources" view=|| view! { <Outlet />}>
                        <Route path="items/:page?" view=|| view! { <ItemsListPage /> } />
                        <Route path="offers/:item_id/:page?" view=|| view! { <ItemOffersPage /> } />
                        <Route path="offer/:offer_id" view=|| view! { <ResourcesListingsPage /> } />
                    </Route>

                </Routes>
            </main>
        </Router>
    }
}
