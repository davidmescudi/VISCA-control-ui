use serde::{Deserialize, Serialize};
use super::camera::Camera;
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub cameras: Vec<Camera>,
}
