use actix_identity::Identity;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::AppResult;
use crate::states::DbRepository;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl UserModel {
    pub async fn try_from_identity<Repo>(
        identity: Option<Identity>,
        repo: &Repo,
    ) -> AppResult<Option<UserModel>>
    where
        Repo: DbRepository,
    {
        use crate::errors::AppError;
        use error_stack::ResultExt;

        let user = match identity {
            Some(identity) => {
                let id = identity.id().change_context(AppError)?;
                let id = id.parse::<i64>().change_context(AppError)?;

                repo.get_user_by_id(id).await?
            }
            None => None,
        };
        Ok(user)
    }
}
