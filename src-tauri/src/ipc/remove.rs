use crate::database::connection::DbPoolWrapper;
use crate::database::models::*;
use crate::ipc::Response;
use crate::schema;
use diesel::prelude::*;
use tauri::State;

#[tauri::command]
pub fn cmd_remove_wallpaper_source(
    state: State<'_, DbPoolWrapper>,
    id: String,
) -> Result<Response<WallpaperSource>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match diesel::delete(
        schema::wallpaper_sources::table.filter(schema::wallpaper_sources::id.eq(id)),
    )
    .get_result(&mut conn)
    {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}
