use crate::ipc::Response;
use crate::scan::{scan, scan_all};
use crate::{db_models, schema, DbPoolWrapper};
use diesel::prelude::*;
use tauri::State;

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
