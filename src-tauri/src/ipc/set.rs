use crate::database::connection::DbPoolWrapper;
use crate::database::models::*;
use crate::hyprpaper;
use crate::ipc::Response;
use crate::schema;
use diesel::prelude::*;
use diesel::upsert::excluded;
use rand::Rng;
use tauri::State;

#[tauri::command]
pub fn set_wallpaper(
    state: State<'_, DbPoolWrapper>,
    screen: String,
    wallpaper_id: Option<String>,
    mode: String,
    is_temporary: bool,
) -> Response<Wallpapers> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };

    match hyprpaper::unload(hyprpaper::Unload::All) {
        Ok(_) => {}
        Err(e) => {
            return Response::error(
                "Error unloading wallpapers".to_string(),
                Some(e.to_string()),
            )
        }
    }

    println!("wallpaper_id: {:#?}", wallpaper_id);

    let wallpaper: Option<Wallpapers> = match wallpaper_id {
        Some(id) => {
            match schema::wallpapers::table
                .filter(schema::wallpapers::id.eq(&id))
                .get_result::<Wallpapers>(&mut conn)
            {
                Ok(v) => Some(v),
                Err(e) => {
                    return Response::error(
                        "Error getting wallpaper".to_string(),
                        Some(e.to_string()),
                    )
                }
            }
        }
        None => match schema::wallpapers::table.get_results::<Wallpapers>(&mut conn) {
            Ok(v) => {
                let mut rng = rand::rng();
                let r = rng.random_range(0..v.len());

                println!("{}", r);
                v.get(r).cloned()
            }
            Err(e) => {
                return Response::error("Error getting wallpaper".to_string(), Some(e.to_string()))
            }
        },
    };

    if let Some(target_wallpaper) = wallpaper {
        let h_mode = hyprpaper::Mode::from_string(mode);

        match hyprpaper::set_wallpaper(screen.clone(), target_wallpaper.path.clone(), &h_mode) {
            Ok(_) => {
                if !is_temporary {
                    let mut actives_list: Vec<NewActive> = Vec::new();

                    if screen == "all" {
                        match hyprpaper::active_screens() {
                            Ok(v) => {
                                for a in v {
                                    actives_list.push(NewActive::new(
                                        a,
                                        target_wallpaper.id.clone(),
                                        h_mode.to_string(),
                                    ));
                                }
                            }
                            Err(e) => {
                                return Response::error(
                                    "Error getting active screens".to_string(),
                                    Some(e.to_string()),
                                )
                            }
                        }
                    } else {
                        actives_list.push(NewActive::new(
                            screen,
                            target_wallpaper.id.clone(),
                            h_mode.to_string(),
                        ));
                    }

                    for active in actives_list {
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
            }
            Err(e) => {
                return Response::error("Error setting wallpaper".to_string(), Some(e.to_string()))
            }
        };

        return Response::ok(target_wallpaper);
    }

    Response::error("Failed to set wallpaper".to_string(), None)
}

#[tauri::command]
pub fn add_wallpaper_source(
    state: State<'_, DbPoolWrapper>,
    path: String,
) -> Response<WallpaperSources> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Response::error("Error getting connection".to_string(), Some(e.to_string()))
        }
    };
    let wallpaper_source = NewWallpaperSource::new(path);

    match diesel::insert_into(schema::wallpaper_sources::table)
        .values(&wallpaper_source)
        .get_result::<WallpaperSources>(&mut conn)
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
) -> Response<WallpaperSources> {
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
