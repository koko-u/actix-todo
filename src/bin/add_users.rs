//! Programs to manually register users
//!
//! [Usage]
//!   cargo run add_users --name John --email john@example.com --password jhon001
//!

use std::env;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use clap::Parser;
use error_stack::ResultExt;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::PgPool;
use url::Url;

use todoapp::errors::AppError;
use todoapp::errors::AppResult;

#[derive(Debug, clap::Parser)]
struct AddUserOpt {
    #[arg(short = 'n', long = "name")]
    name: String,
    #[arg(short = 'e', long = "email")]
    email: String,
    #[arg(short = 'p', long = "password")]
    password: String,
}

async fn get_connection_pool(database_url: &str) -> AppResult<PgPool> {
    let database_url = Url::parse(database_url).change_context(AppError)?;
    let options = PgConnectOptions::from_url(&database_url).change_context(AppError)?;
    let pool = PgPoolOptions::default().connect_lazy_with(options);

    Ok(pool)
}

#[actix_web::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().change_context(AppError)?;

    let opt = AddUserOpt::parse();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2::Argon2::default();
    let hashed_password = argon2
        .hash_password(opt.password.as_bytes(), &salt)
        .change_context(AppError)?;

    let database_url = env::var("DATABASE_URL").change_context(AppError)?;
    let pool = get_connection_pool(&database_url).await?;

    {
        let mut tx = pool.begin().await.change_context(AppError)?;

        sqlx::query!(
            r#"
            INSERT INTO users (name, email, hashed_password, is_active)
            VALUES ($1, $2, $3, $4)
            "#,
            opt.name,
            opt.email,
            hashed_password.to_string(),
            true
        )
        .execute(tx.as_mut())
        .await
        .change_context(AppError)?;

        tx.commit().await.change_context(AppError)?;
    }

    Ok(())
}
