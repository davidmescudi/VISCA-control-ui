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

#[get("/camera_preset/<id>")]
fn get_camera_preset(id: u32, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Json<CameraPreset>>, BadRequest<Option<String>>> {
    let presets = state.read().unwrap();

    match presets.get(&id) {
        Some(preset) => Ok(Accepted(Json(preset.clone()))),
        None => Err(BadRequest(Some(format!("CameraPreset with ID {} not found", id)))),
    }
}

#[get("/camera_presets")]
fn get_all_camera_presets(state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Accepted<Json<Vec<CameraPreset>>> {
    let presets = state.read().unwrap();
    Accepted(Json(presets.values().cloned().collect()))
}

#[post("/camera_preset/insert", format = "json", data = "<camera_preset>")]
fn insert_camera_preset(camera_preset: Json<CameraPreset>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Json<CameraPreset>>, BadRequest<Option<String>>>  {
    let new_preset = camera_preset.into_inner();

    let mut presets = state.write().unwrap();
    let existing_ids: HashSet<u32> = presets.keys().cloned().collect();

    match new_preset.validate_insert(&existing_ids) {
        Ok(_) => {
            presets.insert(new_preset.id, new_preset.clone());
            info!("CameraPreset with ID {} inserted", new_preset.id);
            Ok(Accepted(Json(new_preset)))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[post("/camera_preset/update/<id>", format = "json", data = "<camera_preset>")]
fn update_camera_preset(id: u32, camera_preset: Json<CameraPreset>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Json<CameraPreset>>, BadRequest<Option<String>>> {
    let new_preset = camera_preset.into_inner();

    let mut presets = state.write().unwrap();
    let existing_ids: HashSet<u32> = presets.keys().cloned().collect();

    match new_preset.validate_update(&existing_ids) {
        Ok(_) => {
            presets.insert(id, new_preset.clone());
            Ok(Accepted(Json(new_preset)))
        }
        Err(e) => Err(BadRequest(Some(e))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .manage(Arc::new(RwLock::new(HashMap::<u32, CameraPreset>::new())))
        .mount("/", routes![all_options])
        // TODO: Replace with new functions
        .mount("/api", routes![insert_camera_preset, update_camera_preset, get_all_camera_presets, get_camera_preset])
}