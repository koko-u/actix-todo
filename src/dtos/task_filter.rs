use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct TaskFilter {
    pub summary: Option<String>,
}
