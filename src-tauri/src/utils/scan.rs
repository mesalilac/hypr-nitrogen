use crate::database::models::*;
use crate::ipc::Response;
use crate::schema;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize, Clone, Debug)]
pub struct WallpaperMetadata {
    pub signature: String,
    pub caption: String,
    pub category: String,
    pub tags: Vec<String>,
}

type MetadataHashMap = HashMap<String, WallpaperMetadata>;
type WallpapersHashMap = HashMap<String, NewWallpaper>;

static IMAGE_EXTENSIONS_ARRAY: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];

fn extract_metadata(metadata: &mut MetadataHashMap, path: &Path) {
    match std::fs::read_to_string(path) {
        Ok(t) => match serde_json::from_str::<Vec<WallpaperMetadata>>(&t) {
            Ok(d) => {
                for item in d {
                    metadata.insert(item.signature.clone(), item.clone());
                }
            }
            Err(e) => {
                println!("Error reading metadata file: {}", e);
            }
        },
        Err(e) => {
            println!("Error reading metadata file: {}", e);
        }
    }
}

fn is_image_extension(ext_os_str: &OsStr) -> bool {
    ext_os_str.to_str().is_some_and(|s| {
        let lowercased_s = s.to_lowercase(); // Creates a temporary String
        IMAGE_EXTENSIONS_ARRAY.contains(&lowercased_s.as_str()) // Linear search
    })
}

fn generate_signature(path: &Path) -> Option<String> {
    match std::fs::read(path) {
        Ok(bytes) => {
            let hash = blake3::hash(&bytes);

            return Some(hash.to_string());
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    None
}

fn keywords_from_file_name(file_name: &OsStr) -> String {
    let lossy_string = file_name.to_string_lossy();

    let parts: Vec<&str> = lossy_string.split('_').collect();

    parts.join(" ")
}

pub fn scan(
    conn: &mut SqliteConnection,
    source_id: String,
    source_path: String,
) -> Result<Vec<Wallpaper>, String> {
    let mut wallpapers_hashmap: WallpapersHashMap = HashMap::new();
    let mut metadata: MetadataHashMap = HashMap::new();

    for entry in WalkDir::new(source_path)
        .into_iter()
        .filter_map(|e| e.ok().filter(|x| x.path().is_file()))
    {
        if entry.file_name().to_string_lossy() == "wallpaper_metadata.json" {
            extract_metadata(&mut metadata, entry.path());
            continue;
        }

        match entry.path().extension() {
            Some(ext) => {
                if is_image_extension(ext) {
                    if let Some(signature) = generate_signature(entry.path()) {
                        let new_wallpaper = NewWallpaper::new(
                            signature.clone(),
                            entry.path().to_string_lossy().to_string(),
                            None, // TODO: Extract resolution
                            source_id.clone(),
                            None,
                        );

                        wallpapers_hashmap.insert(signature, new_wallpaper);
                    }
                }
            }
            None => {
                continue;
            }
        }
    }

    wallpapers_hashmap.iter_mut().for_each(|w| {
        if let Some(metadata) = metadata.get(&w.1.signature) {
            let mut temp_string = String::new();

            temp_string.push_str(metadata.category.clone().as_str());
            temp_string.push(' ');

            metadata.tags.iter().for_each(|x| {
                temp_string.push_str(x);
                temp_string.push(' ');
            });

            temp_string.push_str(metadata.caption.clone().as_str());

            if let Some(file_name) = Path::new(&w.1.path).file_stem() {
                temp_string.push_str(&keywords_from_file_name(file_name));
            }

            w.1.keywords = Some(temp_string);
        } else {
            let mut temp_string = String::new();

            if let Some(file_name) = Path::new(&w.1.path).file_stem() {
                temp_string.push_str(&keywords_from_file_name(file_name));
            }

            w.1.keywords = Some(temp_string);
        }
    });

    let mut wallpapers_list: Vec<Wallpaper> = Vec::new();

    for w in wallpapers_hashmap.values() {
        match diesel::insert_into(schema::wallpapers::table)
            .values(w)
            .get_result::<Wallpaper>(conn)
        {
            Ok(v) => {
                wallpapers_list.push(v);
            }
            Err(DieselError::DatabaseError(kind, _)) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    continue;
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Ok(wallpapers_list)
}

pub fn scan_all(conn: &mut SqliteConnection) -> Result<Response<Vec<Wallpaper>>, String> {
    let mut wallpapers_list: Vec<Wallpaper> = Vec::new();

    let wallpaper_sources: Vec<WallpaperSource> =
        match schema::wallpaper_sources::table.get_results::<WallpaperSource>(conn) {
            Ok(v) => v,
            Err(e) => {
                return Err(e.to_string());
            }
        };

    for source in wallpaper_sources {
        match scan(conn, source.id, source.path) {
            Ok(wallpapers) => {
                wallpapers_list.extend(wallpapers);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(Response::new(wallpapers_list))
}
