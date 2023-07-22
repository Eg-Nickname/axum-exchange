use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::get,
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };

    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use sqlx::{PgPool, postgres::{PgPoolOptions}};
    use axum_session::{SessionPgPool, SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthSession, AuthSessionLayer, AuthConfig};
    

    use axum_exchange_narody::app::*;
    use axum_exchange_narody::fileserv::file_and_error_handler;
    use axum_exchange_narody::state::AppState;
    use axum_exchange_narody::auth::*;

    async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession<User, i64, SessionPgPool, PgPool>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, raw_query, move |cx| {
            provide_context(cx, auth_session.clone());
            provide_context(cx, app_state.pool.clone());
        }, request).await
    }

    async fn leptos_routes_handler(auth_session: AuthSession<User, i64, SessionPgPool, PgPool>, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
            move |cx| {
                provide_context(cx, auth_session.clone());
                provide_context(cx, app_state.pool.clone());
            },
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
        use dotenv::dotenv;
        dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("Db url must be set").to_owned();
        // Db connection
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url[..])
            .await
            .expect("Couldnt connect to db");

        let session_config = SessionConfig::default()
            .with_table_name("sessions_table");
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config).await.unwrap();
        session_store.initiate().await.unwrap();

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        let app_state = AppState{
            leptos_options,
            pool: pool.clone(),
        };

        // build our application with a route
        let app = Router::new()
            .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
            .fallback(file_and_error_handler)
            .layer(AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool.clone()))
            .with_config(auth_config))
            .layer(SessionLayer::new(session_store))
            .with_state(app_state);

        
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
