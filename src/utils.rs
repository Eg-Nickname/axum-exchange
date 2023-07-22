use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::auth::AuthSession;
        use sqlx::PgPool;
        use leptos::*;

        pub fn pool(cx: Scope) -> Result<PgPool, ServerFnError> {
            use_context::<PgPool>(cx)
                 .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
         }
     
         pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
             use_context::<AuthSession>(cx)
                 .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
         }
    }
}