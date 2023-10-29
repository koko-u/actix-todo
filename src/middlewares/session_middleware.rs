use actix_session::storage::CookieSessionStore;
use actix_session::storage::SessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;

pub fn build_session(secret_key: Key) -> SessionMiddleware<impl SessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), secret_key).build()
}
