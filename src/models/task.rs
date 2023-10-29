use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: i64,
    pub summary: String,
    pub description: Option<String>,
    pub status_id: i64,
    pub status_name: String,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
