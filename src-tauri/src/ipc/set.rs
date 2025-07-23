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
pub async fn cmd_set_wallpaper(
    state: State<'_, DbPoolWrapper>,
    screen: String,
    wallpaper_id: Option<String>,
    mode: String,
    is_temporary: bool,
) -> Result<Response<Wallpaper>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match hyprpaper::unload(hyprpaper::Unload::All) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    let wallpaper: Option<Wallpaper> = match wallpaper_id {
        Some(id) => {
            match schema::wallpapers::table
                .filter(schema::wallpapers::id.eq(&id))
                .get_result::<Wallpaper>(&mut conn)
            {
                Ok(v) => Some(v),
                Err(e) => return Err(e.to_string()),
            }
        }
        None => match schema::wallpapers::table
            .inner_join(schema::wallpaper_sources::table)
            .filter(schema::wallpaper_sources::dsl::active.eq(true))
            .select(schema::wallpapers::all_columns)
            .get_results::<Wallpaper>(&mut conn)
        {
            Ok(v) => {
                if v.is_empty() {
                    return Err(String::from("No wallpapers found"));
                }

                let mut rng = rand::rng();
                let r = rng.random_range(0..v.len());

                v.get(r).cloned()
            }
            Err(e) => return Err(e.to_string()),
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
                            Err(e) => return Err(e.to_string()),
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
                            Err(e) => return Err(e.to_string()),
                        };
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        };

        return Ok(Response::new(target_wallpaper));
    }

    Err(String::from("Failed to set wallpaper"))
}

#[tauri::command]
pub async fn cmd_add_wallpaper_source(
    state: State<'_, DbPoolWrapper>,
    path: String,
) -> Result<Response<WallpaperSource>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    let wallpaper_source = NewWallpaperSource::new(path);

    match diesel::insert_into(schema::wallpaper_sources::table)
        .values(&wallpaper_source)
        .get_result::<WallpaperSource>(&mut conn)
    {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn cmd_update_wallpaper_source_active(
    state: State<'_, DbPoolWrapper>,
    id: String,
    active: bool,
) -> Result<Response<WallpaperSource>, String> {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match diesel::update(schema::wallpaper_sources::table.find(id))
        .set(schema::wallpaper_sources::active.eq(active))
        .get_result(&mut conn)
    {
        Ok(v) => Ok(Response::new(v)),
        Err(e) => Err(e.to_string()),
    }
}
