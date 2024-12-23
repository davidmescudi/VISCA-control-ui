#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::response::status::{BadRequest, Accepted, NoContent};
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

#[get("/download")]
fn download_state(state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Accepted<Json<HashMap<u32, CameraPreset>>> {
    let state = state.read().unwrap();

    Accepted(Json(state.clone()))
}

#[post("/upload", format = "json", data = "<new_state>")]
fn upload_state(new_state: Json<HashMap<u32, CameraPreset>>, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<NoContent, BadRequest<Option<String>>> {
    let mut state = state.write().unwrap();
    *state = new_state.into_inner();
    Ok(NoContent)
}

#[get("/camera_preset/<id>")]
fn get_camera_preset(id: u32, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<Accepted<Json<CameraPreset>>, BadRequest<Option<String>>> {
    let presets = state.read().unwrap();

    match presets.get(&id) {
        Some(preset) => Ok(Accepted(Json(preset.clone()))),
        None => Err(BadRequest(Some(format!("CameraPreset with ID {} not found", id)))),
    }
}

#[delete("/camera_preset/delete/<id>")]
fn delete_camera_preset(id: u32, state: &State<Arc<RwLock<HashMap<u32, CameraPreset>>>>) -> Result<NoContent, BadRequest<Option<String>>> {
    info!("Deleting CameraPreset with ID {}", id);
    let mut presets = state.write().unwrap();

    match presets.remove(&id) {
        Some(_) => Ok(NoContent),
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
        .mount("/api", routes![insert_camera_preset, update_camera_preset, delete_camera_preset, get_all_camera_presets, get_camera_preset])
        .mount("/backup", routes![download_state, upload_state])
}