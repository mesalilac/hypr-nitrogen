use crate::hyprpaper;
use crate::ipc::Response;
use crate::{db_models, schema, DbPoolWrapper};
use diesel::prelude::*;
use diesel::upsert::excluded;
use tauri::State;

#[tauri::command]
pub fn set_wallpaper(
    state: State<'_, DbPoolWrapper>,
    screens: Vec<String>,
    wallpaper_id: String,
    mode: String,
    is_temporary: bool,
) -> Response<db_models::Wallpapers> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    let wallpaper = match schema::wallpapers::table
        .filter(schema::wallpapers::id.eq(&wallpaper_id))
        .get_result::<db_models::Wallpapers>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Response::error("Error getting wallpaper".to_string(), Some(e.to_string()))
        }
    };

    let h_mode = hyprpaper::Mode::from_string(mode);

    for screen in screens {
        if screen == "all" {
            continue;
        }

        match hyprpaper::set_wallpaper(&screen, wallpaper.path.clone(), &h_mode) {
            Ok(_) => {
                let active =
                    db_models::NewActive::new(screen, wallpaper_id.clone(), h_mode.to_string());

                if !is_temporary {
                    match diesel::insert_into(schema::active::table)
                        .values(&active)
                        .on_conflict(schema::active::dsl::screen)
                        .do_update()
                        .set((
                            schema::active::dsl::wallpaper_id
                                .eq(excluded(schema::active::dsl::wallpaper_id)),
                            schema::active::dsl::mode.eq(excluded(schema::active::dsl::mode)),
                        ))
                        .execute(&mut conn)
                    {
                        Ok(_) => {}
                        Err(e) => {
                            return Response::error(
                                "Error setting wallpaper as active".to_string(),
                                Some(e.to_string()),
                            )
                        }
                    };
                }
            }
            Err(e) => {
                return Response::error("Error setting wallpaper".to_string(), Some(e.to_string()))
            }
        };
    }

    Response::ok(wallpaper)
}

#[tauri::command]
pub fn add_wallpaper_source(
    state: State<'_, DbPoolWrapper>,
    path: String,
) -> Response<db_models::WallpaperSources> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };
    let wallpaper_source = db_models::NewWallpaperSource::new(path);

    match diesel::insert_into(schema::wallpaper_sources::table)
        .values(&wallpaper_source)
        .get_result::<db_models::WallpaperSources>(&mut conn)
    {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error(
            "Error adding wallpaper source".to_string(),
            Some(e.to_string()),
        ),
    }
}

#[tauri::command]
pub fn update_wallpaper_source_active(
    state: State<'_, DbPoolWrapper>,
    id: String,
    active: bool,
) -> Response<db_models::WallpaperSources> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match diesel::update(schema::wallpaper_sources::table.find(id))
        .set(schema::wallpaper_sources::active.eq(active))
        .get_result(&mut conn)
    {
        Ok(v) => Response::ok(v),
        Err(e) => Response::error(
            "Error updating wallpaper source".to_string(),
            Some(e.to_string()),
        ),
    }
}
