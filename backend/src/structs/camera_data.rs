use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CameraData {
    pub x: AtomicI64,
    pub y: AtomicI64,
    pub z: AtomicI64,
}

impl CameraData {
    // Validation method to ensure values are between 0 and 10
    pub fn validate(&self) -> Result<(), String> {
        let x = self.x.load(Ordering::SeqCst);
        let y = self.y.load(Ordering::SeqCst);
        let z = self.z.load(Ordering::SeqCst);

        if (0..=10).contains(&x) && (0..=10).contains(&y) && (0..=10).contains(&z) {
            Ok(())
        } else {
            Err(format!("Values out of range: x = {}, y = {}, z = {}. Values must be between 0 and 10.", x, y, z))
        }
    }
}
