use actix_identity::Identity;
use actix_web::web;
use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::RootTemplate;

pub async fn root_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let user = UserModel::try_from_identity(identity, &app_data.repo).await?;

    Ok(RootTemplate { login_user: user })
}
