use actix_identity::Identity;
use actix_web::http;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::states::DbRepository;

pub async fn logout_handler<Repo>(
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    if let Some(identity) = identity {
        identity.logout();
    }

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/"))
        .finish())
}
