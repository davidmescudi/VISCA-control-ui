use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraPreset {
    pub id: u32,
    pub camera_id: Option<u32>,
    pub workspace_position: Position,
    pub camera_settings: CameraSettings,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraSettings {
    pub zoom: i32,
    pub position: Position,
}

impl CameraPreset {
    pub fn validate(&self, existing_ids: &HashSet<u32>) -> Result<(), String> {
        if existing_ids.contains(&self.id) {
            return Err(format!("ID {} is not unique", self.id));
        }

        if self.workspace_position.x < 0 || self.workspace_position.y < 0 {
            return Err("Workspace position values must be non-negative".to_string());
        }

        if self.camera_settings.position.x < 0 || self.camera_settings.position.y < 0 {
            return Err("Camera position values must be non-negative".to_string());
        }

        Ok(())
    }
}

