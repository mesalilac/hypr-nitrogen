use crate::database::DbPoolWrapper;
use crate::db_models;
use crate::ipc::Response;
use crate::schema;
use diesel::prelude::*;
use tauri::State;

#[tauri::command]
pub fn remove_wallpaper_source(
    state: State<'_, DbPoolWrapper>,
    id: String,
) -> Response<db_models::WallpaperSources> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match diesel::delete(
        schema::wallpaper_sources::table.filter(schema::wallpaper_sources::id.eq(id)),
    )
    .get_result(&mut conn)
    {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error(
            "Error deleting wallpaper source".to_string(),
            Some(e.to_string()),
        ),
    }
}
