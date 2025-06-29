use crate::utils::get_app_data_dir;
use diesel::{r2d2, RunQueryDsl, SqliteConnection};
use std::path::PathBuf;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub struct DbPoolWrapper {
    pub pool: DbPool,
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

fn get_database_location() -> PathBuf {
    let dir = get_app_data_dir();

    dir.join("data.db")
}

pub fn get_connection_pool() -> DbPool {
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

pub fn get_database_url() -> String {
    if cfg!(dev) {
        "./dev.db".to_string()
    } else {
        get_database_location().to_str().unwrap().to_string()
    }
}
