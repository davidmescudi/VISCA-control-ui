use serde::Deserialize;
use super::camera::Camera;

#[derive(Deserialize)]
pub struct Config {
    pub cameras: Vec<Camera>,
}
