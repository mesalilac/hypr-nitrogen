use crate::hyprpaper;
use crate::ipc::Response;
use crate::{db_models, schema, DbPoolWrapper};
use diesel::prelude::*;
use tauri::State;

/// returns list of active screens in hyprland
#[tauri::command]
pub fn get_screens() -> Response<Vec<String>> {
    match hyprpaper::active_screens() {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error("Error getting screens".to_string(), Some(e.to_string())),
    }
}

#[tauri::command]
pub fn get_wallpaper_sources(
    state: State<'_, DbPoolWrapper>,
) -> Response<Vec<db_models::WallpaperSources>> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match schema::wallpaper_sources::table.get_results::<db_models::WallpaperSources>(&mut conn) {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error(
            "Error getting wallpaper sources".to_string(),
            Some(e.to_string()),
        ),
    }
}

#[tauri::command]
pub fn get_wallpapers(state: State<'_, DbPoolWrapper>) -> Response<Vec<db_models::Wallpapers>> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match schema::wallpapers::table.get_results::<db_models::Wallpapers>(&mut conn) {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error("Error getting wallpapers".to_string(), Some(e.to_string())),
    }
}

#[tauri::command]
pub fn get_active_wallpapers(state: State<'_, DbPoolWrapper>) -> Response<Vec<db_models::Active>> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match schema::active::table.get_results::<db_models::Active>(&mut conn) {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error("Error getting wallpapers".to_string(), Some(e.to_string())),
    }
}
