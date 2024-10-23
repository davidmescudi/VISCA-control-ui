#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::response::status::{BadRequest, Accepted};

mod structs {
    pub mod camera_data;
}
use structs::camera_data::CameraData;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/position")]
fn camera_position() -> &'static str {
    "Return Camera Position"
}

#[post("/position", format = "json", data = "<camera_data>")]
fn post_camera_position(camera_data: Json<CameraData>) -> Result<Accepted<Option<String>>, BadRequest<Option<String>>> {
    let request = camera_data.into_inner();

    match request.validate() {
        Ok(()) => {
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
        .mount("/", routes![index])
        .mount("/api/camera", routes![camera_position,post_camera_position])
}