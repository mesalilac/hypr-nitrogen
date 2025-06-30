use crate::database::connection::DbPoolWrapper;
use crate::database::models::*;
use crate::ipc::Response;
use crate::schema;
use crate::utils::restore;
use crate::utils::scan::{scan, scan_all};
use diesel::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn cmd_scan_source(
    state: State<'_, DbPoolWrapper>,
    source_id: String,
) -> Result<Response<Vec<Wallpaper>>, String> {
    use schema::wallpaper_sources::dsl::*;

    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    let wallpaper_source = match wallpaper_sources
        .find(source_id)
        .get_result::<WallpaperSource>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let wallpapers = match scan(&mut conn, wallpaper_source.id, wallpaper_source.path) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(Response::new(wallpapers))
}

#[tauri::command]
pub async fn cmd_scan_all_sources(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<Vec<Wallpaper>>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(err) = diesel::delete(schema::wallpapers::table).execute(&mut conn) {
        return Err(err.to_string());
    }

    match scan_all(&mut conn) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn cmd_restore_wallpapers(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<bool>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match restore(&mut conn) {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e),
    }
}
