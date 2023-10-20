use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::templates::RootTemplate;

pub async fn root_handler() -> Result<impl Responder, AppResponseError> {
    Ok(RootTemplate)
}
