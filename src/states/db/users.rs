use async_trait::async_trait;
use chrono::NaiveDateTime;
use error_stack::ResultExt;

use crate::crypt;
use crate::dtos::NewUser;
use crate::errors::AppError;
use crate::errors::AppResult;
use crate::models::UserModel;
use crate::states::repositories::UsersRepository;

use super::DbState;

#[async_trait]
impl UsersRepository for DbState {
    async fn create_user(&self, new_user: &NewUser) -> AppResult<UserModel> {
        let user: UserModel;
        {
            let mut tx = self.0.begin().await.change_context(AppError)?;
            let hashed_password = crypt::hash(&new_user.password)?;
            user = sqlx::query_file_as!(
                UserModel,
                "sql/insert_user.sql",
                new_user.name,
                new_user.email,
                hashed_password,
                true
            )
            .fetch_one(tx.as_mut())
            .await
            .change_context(AppError)?;

            tx.commit().await.change_context(AppError)?;
        }

        Ok(user)
    }

    async fn auth_user(&self, email: &str, password: &str) -> AppResult<Option<UserModel>> {
        let Some(raw_user) = sqlx::query_file_as!(RawUser, "sql/get_user_by_email.sql", email)
            .fetch_optional(&self.0)
            .await
            .change_context(AppError)?
        else {
            return Ok(None);
        };

        let ok = crypt::verify(password, &raw_user.hashed_password)?;
        if ok {
            Ok(Some(raw_user.into()))
        } else {
            Ok(None)
        }
    }

    async fn get_user_by_id(&self, id: i64) -> AppResult<Option<UserModel>> {
        let user = sqlx::query_file_as!(UserModel, "sql/get_user_by_id.sql", id)
            .fetch_optional(&self.0)
            .await
            .change_context(AppError)?;

        Ok(user)
    }
}

struct RawUser {
    id: i64,
    name: String,
    email: String,
    hashed_password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl From<RawUser> for UserModel {
    fn from(
        RawUser {
            id,
            name,
            email,
            created_at,
            updated_at,
            ..
        }: RawUser,
    ) -> UserModel {
        UserModel {
            id,
            name,
            email,
            created_at,
            updated_at,
        }
    }
}
