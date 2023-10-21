use serde::Deserialize;

#[derive(Deserialize, Default, Clone, PartialEq, Eq)]
pub struct TaskFilter {
    pub summary: String,
}
