mod database;
mod hyprpaper;
mod ipc;
mod schema;
mod utils;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const APP_NAME: &str = "hypr-nitrogen";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    let pool = database::connection::get_connection_pool();

    if let Ok(mut conn) = pool.get() {
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => {}
            Err(e) => panic!("Failed to run migrations: {}", e),
        }
    }

    // NOTE:
    // Forces the appimage to use wayland and not xwayland
    //      https://github.com/tauri-apps/tauri/issues/11790
    //      TODO: Add a clap flag to disable or enable this
    std::env::set_var("GDK_BACKEND", "wayland");

    tauri::Builder::default()
        .manage(database::connection::DbPoolWrapper { pool })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ipc::set_wallpaper,
            ipc::add_wallpaper_source,
            ipc::update_wallpaper_source_active,
            ipc::get_screens,
            ipc::get_wallpaper_sources,
            ipc::get_wallpapers,
            ipc::get_active_wallpapers,
            ipc::remove_wallpaper_source,
            ipc::scan_source,
            ipc::scan_all_sources,
            ipc::restore_wallpapers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
