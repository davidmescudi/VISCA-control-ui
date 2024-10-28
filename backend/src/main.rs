#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::response::status::{BadRequest, Accepted};
use rocket::State;
use std::sync::atomic::{AtomicI64, Ordering};

mod structs {
    pub mod camera_data;
}
use structs::camera_data::CameraData;

mod fairings {
    pub mod cors;
}
use fairings::cors::Cors;

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
fn camera_position(state: &State<CameraData>) -> Json<CameraData> {
    let x = state.x.load(Ordering::SeqCst);
    let y = state.y.load(Ordering::SeqCst);
    let z = state.z.load(Ordering::SeqCst);

    let position = CameraData {
        x: AtomicI64::from(x),
        y: AtomicI64::from(y),
        z: AtomicI64::from(z),
    };

    Json(position)
}

#[post("/position", format = "json", data = "<camera_data>")]
fn post_camera_position(camera_data: Json<CameraData>, state: &State<CameraData>) -> Result<Accepted<Option<String>>, BadRequest<Option<String>>> {
    let request = camera_data.into_inner();

    match request.validate() {
        Ok(()) => {
            state.x.store(request.x.load(Ordering::SeqCst), Ordering::SeqCst);
            state.y.store(request.y.load(Ordering::SeqCst), Ordering::SeqCst);
            state.z.store(request.z.load(Ordering::SeqCst), Ordering::SeqCst);

            Ok(Accepted(Some(format!(
                "Received camera position: x = {:?}, y = {:?}, z = {:?}",
                request.x, request.y, request.z
            ))))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .manage(CameraData{ x: AtomicI64::from(0) , y: AtomicI64::from(0), z: AtomicI64::from(0)})
        .mount("/", routes![index, all_options])
        .mount("/api/camera", routes![camera_position,post_camera_position])
}