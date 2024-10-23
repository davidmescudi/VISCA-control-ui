#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CameraData {
    x: f64,
    y: f64,
    z: f64,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/position")]
fn camera_position() -> &'static str {
    "Return Camera Position"
}

#[post("/position", format = "json", data = "<camera_data>")]
fn post_camera_position(camera_data: Json<CameraData>) -> String {
    let data = camera_data.into_inner();
    format!("Received camera position: x = {}, y = {}, z = {}", data.x, data.y, data.z)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/camera", routes![camera_position])
}