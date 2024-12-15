#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json};
use rocket::response::status::{BadRequest, Accepted};
use rocket::State;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};

mod structs {
    // TODO: Remove camera_data
    pub mod camera_data;
    pub mod camera_preset;
    pub mod camera;
}
// TODO: Remove camera_data
use structs::camera_data::CameraData;

use structs::camera_preset::{CameraPreset, Position, CameraSettings};
use structs::camera::Camera;

mod fairings {
    pub mod cors;
}
use fairings::cors::Cors;
// TODO: Remove if not needed
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

// TODO: Remove camera_data
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

// TODO: Remove camera_data
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

#[get("/camera_preset/<id>")]
fn get_camera_preset(id: u32, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Option<Json<CameraPreset>> {
    let presets = state.read().unwrap();
    presets.get(&id).cloned().map(Json)
}

#[get("/camera_presets")]
fn get_all_camera_presets(state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Json<Vec<CameraPreset>> {
    let presets = state.read().unwrap();
    Json(presets.values().cloned().collect())
}

#[post("/camera_preset/<id>", format = "json", data = "<camera_preset>")]
fn update_camera_preset(id: u32, camera_preset: Json<CameraPreset>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Option<String>>, BadRequest<Option<String>>> {
    let new_preset = camera_preset.into_inner();

    let mut presets = state.write().unwrap();
    let existing_ids: HashSet<u32> = presets.keys().cloned().collect();

    match new_preset.validate(&existing_ids) {
        Ok(_) => {
            presets.insert(id, new_preset);
            Ok(Accepted(Some(format!("CameraPreset with ID {} updated", id))))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        //.manage(Arc::new(RwLock::new(HashMap::<u32, CameraPreset>::new())))
        .manage(CameraData{ x: AtomicI64::from(0) , y: AtomicI64::from(0), z: AtomicI64::from(0)})
        .mount("/", routes![index, all_options])
        // TODO: Replace with new functions
        .mount("/api/camera", routes![camera_position,post_camera_position])
}