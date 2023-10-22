use serde::Deserialize;

use crate::utils::AsOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
pub struct TaskFilter {
    pub summary: Option<String>,
    pub status_id: Option<i64>,
}
impl TaskFilter {
    pub fn is_empty(&self) -> bool {
        self.summary.is_none() && self.status_id.is_none()
    }

    // empty key is treat as None
    pub fn normalize(self) -> Self {
        Self {
            summary: self.summary.as_ref().and_then(AsOption::empty_as_none),
            status_id: self
                .status_id
                .and_then(|status_id| if status_id > 0 { Some(status_id) } else { None }),
        }
    }
}
