use crate::database::models::*;
use crate::{hyprpaper, schema};
use diesel::prelude::*;

pub fn restore(conn: &mut SqliteConnection) -> Result<bool, String> {
    let active_wallpapers = match schema::active::table.get_results::<Active>(conn) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    for active_wallpaper in active_wallpapers {
        let wallpaper = match schema::wallpapers::table
            .filter(schema::wallpapers::dsl::id.eq(active_wallpaper.wallpaper_id))
            .get_result::<Wallpaper>(conn)
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
