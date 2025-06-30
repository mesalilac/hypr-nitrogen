use crate::database::connection::DbPoolWrapper;
use crate::database::models::*;
use crate::hyprpaper;
use crate::ipc::Response;
use crate::schema;
use diesel::prelude::*;
use tauri::State;

/// returns list of active screens in hyprland
#[tauri::command]
pub fn get_screens() -> Result<Response<Vec<String>>, String> {
    match hyprpaper::active_screens() {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_wallpaper_sources(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<Vec<WallpaperSource>>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match schema::wallpaper_sources::table.get_results::<WallpaperSource>(&mut conn) {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_wallpapers(state: State<'_, DbPoolWrapper>) -> Result<Response<Vec<Wallpaper>>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match schema::wallpapers::table.get_results::<Wallpaper>(&mut conn) {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_active_wallpapers(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<Vec<Active>>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match schema::active::table.get_results::<Active>(&mut conn) {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}
