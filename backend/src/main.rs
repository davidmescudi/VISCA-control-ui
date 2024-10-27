#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::response::status::{BadRequest, Accepted};
use rocket::State;
use std::sync::RwLock;

mod structs {
    pub mod camera_data;
}
use structs::camera_data::CameraData;

mod fairings {
    pub mod cors;
}
use fairings::cors::CORS;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
#[get("/position")]
fn camera_position(state: &State<RwLock<CameraData>>) -> Json<CameraData> {
    let camera_data = state.read().unwrap();

    let position = CameraData {
        x: camera_data.x,
        y: camera_data.y,
        z: camera_data.z,
    };

    Json(position)
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
        .attach(CORS)
        .manage(RwLock::new(CameraData { x: 0.0 , y: 0.0, z: 0.0}))
        .mount("/", routes![index, all_options])
        .mount("/api/camera", routes![camera_position,post_camera_position])
}