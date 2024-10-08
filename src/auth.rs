use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::PgPool;
    use axum_session_auth::{SessionPgPool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionPgPool, PgPool>;
    use crate::utils::{pool, auth};
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: -1,
            username: "Guest".into(),
            password: "".into(),
            permissions,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;

    impl User {
        pub async fn get(id: i64, pool: &PgPool) -> Option<Self> {
            let sqluser = sqlx::query_as!(
                SqlUser,
                r#"
                    SELECT id, username, password FROM users WHERE id = $1
                "#,
                id
            )
            .fetch_one(pool)
                .await
                .ok()?;

            let sql_user_perms = sqlx::query_as!(
                SqlPermissionTokens,
                r#"SELECT token FROM user_permissions WHERE user_id = $1"#,
                sqluser.id
            )
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_username(name: String, pool: &PgPool) -> Option<Self> {
            let sqluser = sqlx::query_as!(
                SqlUser,
                r#"select id, username, password FROM users WHERE username = $1"#,
                name
            )
            .fetch_one(pool)
            .await.ok()?;

            let sql_user_perms = sqlx::query_as!(
                SqlPermissionTokens,
                r#"SELECT token FROM user_permissions WHERE user_id = $1"#,
                sqluser.id
            )
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, i64, PgPool> for User {
        async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            self.permissions.contains(perm)
        }
    }

    #[derive(Clone)]
    pub struct SqlUser {
        pub id: i64,
        pub username: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
                username: self.username,
                password: self.password,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }
}
}

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query!(
            // language=PostgreSQL
            r#"
                insert into users(username, password)
                values ($1, $2)
            "#,
            username.clone(),
            password_hashed
        )
        .execute(&pool)
        .await?;

    let user =
        User::get_from_username(username, &pool)
            .await
            .ok_or_else(|| {
                ServerFnError::ServerError(
                    "Signup failed: User does not exist.".into(),
                )
            })?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect(cx, "/");

    Ok(())
}

#[server(Logout, "/api", "Url", "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}