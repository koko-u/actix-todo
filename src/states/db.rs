use std::time::Duration;

use error_stack::ResultExt;
use log::LevelFilter;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::PgPool;
use url::Url;

use crate::errors::AppError;
use crate::errors::AppResult;

mod status;
mod tasks;

#[derive(Debug)]
pub struct DbState(PgPool);

impl DbState {
    pub fn init(database_url: &str) -> AppResult<Self> {
        let database_url = Url::parse(database_url).change_context(AppError)?;
        let options = PgConnectOptions::from_url(&database_url)
            .change_context(AppError)?
            .log_statements(LevelFilter::Info);
        let pool = PgPoolOptions::default()
            .after_connect(|conn, meta| {
                Box::pin(async move {
                    if meta.age == Duration::ZERO {
                        let version = sqlx::query_scalar!("SELECT version()")
                            .fetch_one(conn)
                            .await;
                        match version {
                            Ok(Some(version)) => log::info!("Connected to Databaes : {version}"),
                            _ => log::info!("Connected to Database ..."),
                        }
                    }
                    Ok(())
                })
            })
            .connect_lazy_with(options);

        Ok(Self(pool))
    }
}
