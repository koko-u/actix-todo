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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
