use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub id: u32,
    pub name: String,
    pub color: Option<String>,
}

impl Camera {
    pub fn validate(&self, existing_ids: &HashSet<u32>) -> Result<(), String> {
        if existing_ids.contains(&self.id) {
            return Err(format!("ID {} is not unique", self.id));
        }
        Ok(())
    }
}