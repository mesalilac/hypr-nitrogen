mod db_models;
mod hyprpaper;
mod ipc;
mod scan;
mod schema;

use std::path::PathBuf;

use diesel::prelude::*;
use diesel::{r2d2, Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const APP_NAME: &str = "hypr-nitrogen";

pub struct DbPoolWrapper {
    pub pool: DbPool,
}

fn get_config_dir() -> PathBuf {
    let config_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .config_dir()
        .join(APP_NAME);

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Failed to create cache dir");
    }

    config_dir
}

fn get_cache_dir() -> PathBuf {
    let cache_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .cache_dir()
        .join(APP_NAME);

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).expect("Failed to create cache dir");
    }

    cache_dir
}

fn get_app_data_dir() -> PathBuf {
    let app_data_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .data_local_dir()
        .join(APP_NAME);

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create cache dir");
    }

    app_data_dir
}

fn get_database_location() -> PathBuf {
    let dir = get_app_data_dir();

    dir.join("data.db")
}

fn get_database_url() -> String {
    if cfg!(dev) {
        "./dev.db".to_string()
    } else {
        get_database_location().to_str().unwrap().to_string()
    }
}

#[derive(Debug)]
struct EnableForeignKeys;

impl r2d2::CustomizeConnection<SqliteConnection, r2d2::Error> for EnableForeignKeys {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), r2d2::Error> {
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)
            .map(|_| ())
            .map_err(diesel::r2d2::Error::QueryError)
    }
}

fn get_connection_pool() -> DbPool {
    let url = get_database_url();
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    let pool = r2d2::Pool::builder()
        .test_on_check_out(true)
        .connection_customizer(Box::new(EnableForeignKeys))
        .build(manager)
        .expect("Could not build connection pool");

    let mut conn = pool.get().unwrap();

    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)
        .expect("Failed to enable foreign_keys");

    pool
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    let pool = get_connection_pool();

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
        .manage(DbPoolWrapper { pool })
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
