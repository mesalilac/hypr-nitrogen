mod cli;
mod database;
mod hyprpaper;
mod ipc;
mod schema;
mod utils;

use clap::Parser;
use cli::Cli;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use utils::restore;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const APP_NAME: &str = "hypr-nitrogen";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::new();

    builder.filter_level(log::LevelFilter::Info);

    if cli.verbose {
        builder.filter_level(log::LevelFilter::Trace);
    }

    builder.init();

    let pool = database::connection::get_connection_pool();

    if let Ok(mut conn) = pool.get() {
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => {}
            Err(e) => panic!("Failed to run migrations: {e}"),
        }
    }

    if cli.restore {
        if let Ok(mut conn) = pool.get() {
            match restore(&mut conn) {
                Ok(_) => log::info!("Wallpapers restored successfully"),
                Err(e) => {
                    log::error!("Failed to restore wallpapers: {e}");
                    std::process::exit(1);
                }
            }
        }

        std::process::exit(0);
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
            ipc::cmd_set_wallpaper,
            ipc::cmd_add_wallpaper_source,
            ipc::cmd_update_wallpaper_source_active,
            ipc::cmd_get_screens,
            ipc::cmd_get_wallpaper_sources,
            ipc::cmd_get_wallpapers,
            ipc::cmd_get_active_wallpapers,
            ipc::cmd_remove_wallpaper_source,
            ipc::cmd_scan_source,
            ipc::cmd_scan_all_sources,
            ipc::cmd_restore_wallpapers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
