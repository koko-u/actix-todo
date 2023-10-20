use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusModel {
    pub id: i64,
    pub name: String,
}
