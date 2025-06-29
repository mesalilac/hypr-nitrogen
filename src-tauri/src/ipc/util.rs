use crate::database::connection::DbPoolWrapper;
use crate::database::models::*;
use crate::hyprpaper;
use crate::ipc::Response;
use crate::schema;
use crate::utils::scan::{scan, scan_all};
use diesel::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn scan_source(
    state: State<'_, DbPoolWrapper>,
    source_id: String,
) -> Result<Response<Vec<Wallpapers>>, String> {
    use schema::wallpaper_sources::dsl::*;

    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    let wallpaper_source = match wallpaper_sources
        .find(source_id)
        .get_result::<WallpaperSources>(&mut conn)
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
pub async fn scan_all_sources(
    state: State<'_, DbPoolWrapper>,
) -> Result<Response<Vec<Wallpapers>>, String> {
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

pub fn restore(conn: &mut SqliteConnection) -> Result<bool, String> {
    let active_wallpapers = match schema::active::table.get_results::<Active>(conn) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    for active_wallpaper in active_wallpapers {
        let wallpaper = match schema::wallpapers::table
            .filter(schema::wallpapers::dsl::id.eq(active_wallpaper.wallpaper_id))
            .get_result::<Wallpapers>(conn)
        {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        };

        // NOTE: Multi screen is disabled for now
        // Also if you run `hyprpaper::set_wallpaper` in a loop that can crash hyprpaper daemon
        match hyprpaper::set_wallpaper(
            // active_wallpaper.screen,
            "all".to_string(),
            wallpaper.path,
            &hyprpaper::Mode::from_string(active_wallpaper.mode),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }

        break;
    }

    Ok(true)
}

#[tauri::command]
pub fn restore_wallpapers(state: State<'_, DbPoolWrapper>) -> Result<Response<bool>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match restore(&mut conn) {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e),
    }
}
