#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::response::status::{BadRequest, Accepted};
use rocket::State;
use std::sync::RwLock;

mod structs {
    pub mod camera_data;
}
use structs::camera_data::CameraData;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/position")]
fn camera_position(state: &State<RwLock<CameraData>>) -> String {
    let camera_data = state.read().unwrap();
    format!("Current Camera Position: x = {}, y = {}, z = {}", camera_data.x, camera_data.y, camera_data.z)
}

#[post("/position", format = "json", data = "<camera_data>")]
fn post_camera_position(camera_data: Json<CameraData>, state: &State<RwLock<CameraData>>) -> Result<Accepted<Option<String>>, BadRequest<Option<String>>> {
    let request = camera_data.into_inner();

    match request.validate() {
        Ok(()) => {
            let mut camera_data = state.write().unwrap();

            camera_data.x = request.x;
            camera_data.y = request.y;
            camera_data.z = request.z;

            Ok(Accepted(Some(format!(
                "Received camera position: x = {}, y = {}, z = {}",
                request.x, request.y, request.z
            ))))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(RwLock::new(CameraData { x: 0.0 , y: 0.0, z: 0.0}))
        .mount("/", routes![index])
        .mount("/api/camera", routes![camera_position,post_camera_position])
}