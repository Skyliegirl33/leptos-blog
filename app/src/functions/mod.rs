pub mod post;
pub mod dark_mode;
pub mod auth;
pub mod user;

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use leptos::*;
    use sqlx::SqlitePool;
    use crate::functions::auth::AuthSession;

    pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
      use_context::<SqlitePool>(cx)
        .ok_or("Pool missing.")
        .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))
    }

    pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
      use_context::<AuthSession>(cx)
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    pub fn register_server_functions() {
      _ = post::GetPostMetadata::register();
      _ = post::GetPost::register();
      _ = dark_mode::ToggleDarkMode::register();
      _ = auth::Login::register();
      _ = auth::Signup::register();
      _ = auth::Logout::register();
      _ = user::GetUser::register();
    }
  }
}