use std::fmt;

use serde::de::Visitor;
use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TaskFilter {
    pub summary: Option<String>,
    pub status_ids: Vec<i64>,
}

impl TaskFilter {
    pub fn is_empty(&self) -> bool {
        self.summary.is_none() && self.status_ids.is_empty()
    }
}

impl<'de> Deserialize<'de> for TaskFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct FieldVisitor;
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = TaskFilter;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`summary` or `status_id`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut summary: Option<String> = None;
        let mut status_ids = Vec::new();
        while let Some(key) = map.next_key()? {
            match key {
                "summary" => {
                    let value = map.next_value::<String>()?;
                    if !value.is_empty() {
                        summary = Some(value)
                    }
                }
                "status_id" => {
                    status_ids.push(map.next_value()?);
                }
                _ => unreachable!(),
            }
        }

        Ok(Self::Value {
            summary,
            status_ids,
        })
    }
}
