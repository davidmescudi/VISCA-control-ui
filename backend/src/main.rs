#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::response::status::{BadRequest, Accepted};
use rocket::State;
use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};

mod structs {
    pub mod camera_preset;
    pub mod camera;
}

use structs::camera_preset::CameraPreset;

mod fairings {
    pub mod cors;
}
use fairings::cors::Cors;
/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

// TODO: Remove if not needed
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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

#[post("/camera_preset/insert", format = "json", data = "<camera_preset>")]
fn insert_camera_preset(camera_preset: Json<CameraPreset>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Json<CameraPreset>, BadRequest<Option<String>>>  {
    let new_preset = camera_preset.into_inner();

    let mut presets = state.write().unwrap();
    let existing_ids: HashSet<u32> = presets.keys().cloned().collect();

    match new_preset.validate_insert(&existing_ids) {
        Ok(_) => {
            presets.insert(new_preset.id, new_preset.clone());
            // TODO: Remove *presets when done as this clutters the output
            info!("CameraPreset with ID {} inserted, current state {:?}", new_preset.id, *presets);
            Ok(Json(new_preset))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[post("/camera_preset/update/<id>", format = "json", data = "<camera_preset>")]
fn update_camera_preset(id: u32, camera_preset: Json<CameraPreset>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Option<String>>, BadRequest<Option<String>>> {
    let new_preset = camera_preset.into_inner();

    let mut presets = state.write().unwrap();
    let existing_ids: HashSet<u32> = presets.keys().cloned().collect();

    match new_preset.validate_update(&existing_ids) {
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
        .manage(Arc::new(RwLock::new(HashMap::<u32, CameraPreset>::new())))
        .mount("/", routes![index, all_options])
        // TODO: Replace with new functions
        .mount("/api", routes![insert_camera_preset, update_camera_preset, get_all_camera_presets])
}