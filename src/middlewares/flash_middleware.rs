use actix_web::cookie::Key;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;

pub fn build_flash(key: Key) -> FlashMessagesFramework {
    let cookie_store = CookieMessageStore::builder(key).build();
    FlashMessagesFramework::builder(cookie_store).build()
}
