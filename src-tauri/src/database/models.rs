#![allow(unused)]
#![allow(clippy::all)]

use crate::schema;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::Serialize;

#[derive(Queryable, Debug, Associations, Identifiable, Serialize, Clone)]
#[diesel(primary_key(screen))]
#[diesel(table_name = schema::active)]
#[diesel(belongs_to(Wallpaper, foreign_key = wallpaper_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Active {
    pub screen: String,
    pub wallpaper_id: String,
    pub mode: String,
}

#[derive(Queryable, Identifiable, Debug, Serialize, Clone)]
#[diesel(table_name = schema::wallpaper_sources)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WallpaperSource {
    pub id: String,
    pub path: String,
    pub active: bool,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Clone)]
#[diesel(table_name = schema::wallpapers)]
#[diesel(belongs_to(WallpaperSource, foreign_key = wallpaper_source_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wallpaper {
    pub id: String,
    pub signature: String,
    pub path: String,
    pub thumbnail_path: String,
    pub resolution: Option<String>,
    pub wallpaper_source_id: String,
    pub keywords: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::active)]
pub struct NewActive {
    pub screen: String,
    pub wallpaper_id: String,
    pub mode: String,
}

impl NewActive {
    pub fn new(screen: String, wallpaper_id: String, mode: String) -> Self {
        Self {
            screen,
            wallpaper_id,
            mode,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::wallpaper_sources)]
pub struct NewWallpaperSource {
    pub id: String,
    pub path: String,
    pub active: bool,
}

impl NewWallpaperSource {
    pub fn new(path: String) -> Self {
        Self {
            id: nanoid!(),
            path,
            active: true,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::wallpapers)]
pub struct NewWallpaper {
    pub id: String,
    pub signature: String,
    pub path: String,
    pub thumbnail_path: String,
    pub resolution: Option<String>,
    pub wallpaper_source_id: String,
    pub keywords: Option<String>,
}

impl NewWallpaper {
    pub fn new(
        signature: String,
        path: String,
        thumbnail_path: String,
        resolution: Option<String>,
        wallpaper_source_id: String,
        keywords: Option<String>,
    ) -> Self {
        Self {
            id: nanoid!(),
            signature,
            path,
            thumbnail_path,
            resolution,
            wallpaper_source_id,
            keywords,
        }
    }
}
