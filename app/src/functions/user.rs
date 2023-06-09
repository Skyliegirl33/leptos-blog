use leptos::*;
use crate::functions::auth;
use crate::models::User;

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
  let auth = auth(cx)?;

  Ok(auth.current_user)
}