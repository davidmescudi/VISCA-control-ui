use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CameraData {
    // Validation method to ensure values are between 0 and 10
    pub fn validate(&self) -> Result<(), String> {
        if (0.0..=10.0).contains(&self.x) && (0.0..=10.0).contains(&self.y) && (0.0..=10.0).contains(&self.z) {
            Ok(())
        } else {
            Err(format!("Values out of range: x = {}, y = {}, z = {}. Values must be between 0 and 10.", self.x, self.y, self.z))
        }
    }
}
