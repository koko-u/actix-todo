use actix_files::Files;
use actix_web::dev::HttpServiceFactory;

pub fn assets_service() -> impl HttpServiceFactory {
    Files::new("/assets", "./assets")
}
