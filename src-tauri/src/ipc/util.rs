use crate::hyprpaper;
use crate::ipc::Response;
use crate::scan::{scan, scan_all};
use crate::{db_models, schema, DbPoolWrapper};
use diesel::prelude::*;
use tauri::State;

use super::IpcError;

#[tauri::command]
pub async fn scan_source(
    state: State<'_, DbPoolWrapper>,
    source_id: String,
) -> Result<Response<Vec<db_models::Wallpapers>>, ()> {
    use schema::wallpaper_sources::dsl::*;

    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Ok(Response::error(
                "Error getting connection".to_string(),
                Some(e.to_string()),
            ))
        }
    };

    let wallpaper_source = match wallpaper_sources
        .find(source_id)
        .get_result::<db_models::WallpaperSources>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Ok(Response::error(
                "Error wallpaper source not found".to_string(),
                Some(e.to_string()),
            ))
        }
    };

    let wallpapers = match scan(&mut conn, wallpaper_source.id, wallpaper_source.path) {
        Ok(v) => v,
        Err(e) => return Ok(Response::error(e.message, e.details)),
    };

    Ok(Response::ok(wallpapers))
}

#[tauri::command]
pub async fn scan_all_sources(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<Vec<db_models::Wallpapers>>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Ok(Response::error(
                "Error getting connection".to_string(),
                Some(e.to_string()),
            ))
        }
    };

    if let Err(err) = diesel::delete(schema::wallpapers::table).execute(&mut conn) {
        return Ok(Response::error(
            "Error clearing wallpapers list".to_string(),
            Some(err.to_string()),
        ));
    }

    match scan_all(&mut conn) {
        Ok(v) => Ok(v),
        Err(e) => Ok(Response::error(e.message, e.details)),
    }
}

pub fn restore(conn: &mut SqliteConnection) -> Result<bool, String> {
    // TODO: fetch active wallpapers and set them using hyprpaper

    let active_wallpapers = match schema::active::table.get_results::<db_models::Active>(conn) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    for active_wallpaper in active_wallpapers {
        let wallpaper = match schema::wallpapers::table
            .filter(schema::wallpapers::dsl::id.eq(active_wallpaper.wallpaper_id))
            .get_result::<db_models::Wallpapers>(conn)
        {
            Ok(v) => v,
            Err(_) => continue,
        };

        match hyprpaper::set_wallpaper(
            &active_wallpaper.screen,
            wallpaper.id,
            &hyprpaper::Mode::from_string(active_wallpaper.mode),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(true)
}

#[tauri::command]
pub fn restore_wallpapers(state: State<'_, DbPoolWrapper>) -> Response<bool> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match restore(&mut conn) {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error("Error restoring wallpapers".to_string(), Some(e)),
    }
}
