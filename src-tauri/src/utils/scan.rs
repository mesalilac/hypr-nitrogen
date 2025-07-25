use crate::database::models::*;
use crate::ipc::Response;
use crate::schema;
use crate::utils::fs::get_cache_dir;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use futures::StreamExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tauri::async_runtime;
use walkdir::WalkDir;

#[derive(Deserialize, Clone, Debug)]
pub struct WallpaperMetadata {
    pub signature: String,
    pub caption: String,
    pub category: String,
    pub tags: String,
}

type MetadataHashMap = HashMap<String, WallpaperMetadata>;
type WallpapersHashMap = HashMap<String, NewWallpaper>;
type ImageSource = PathBuf;
type ThumbnailDest = PathBuf;
type ThumbnailTask = (ImageSource, ThumbnailDest);

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
                log::error!("Error reading metadata file: {e}");
            }
        },
        Err(e) => {
            log::error!("Error reading metadata file: {e}");
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
            log::error!("Error reading file: {e}");
        }
    }

    None
}

fn keywords_from_file_name(file_name: &OsStr) -> String {
    let lossy_string = file_name.to_string_lossy();

    let parts: Vec<&str> = lossy_string.split('_').collect();

    parts.join(" ")
}

fn create_thumbnail_path(signature: &str) -> String {
    let mut thumbnail_path = get_cache_dir();
    thumbnail_path.push("thumbnails");

    if !thumbnail_path.exists() {
        if let Err(e) = std::fs::create_dir(&thumbnail_path) {
            log::error!("Failed to create thumbnails dir: {e}");
        }
    }

    thumbnail_path.push(format!("{}.{}", signature, "jpeg"));
    thumbnail_path.to_string_lossy().to_string()
}

async fn process_thumbnail_task_list(list: Vec<ThumbnailTask>) {
    let total_threads = std::thread::available_parallelism()
        .map(|x| x.get())
        .unwrap_or(4);

    let stream = futures::stream::iter(list.into_iter().map(|(src, dest)| {
        async_runtime::spawn_blocking(move || {
            if !dest.exists() {
                match std::process::Command::new("magick")
                    .arg(src.as_os_str())
                    .arg("-thumbnail")
                    .arg("400x200^")
                    .arg("-format")
                    .arg("jpeg")
                    .arg(dest.as_os_str())
                    .output()
                {
                    Ok(cmd) => {
                        if cmd.status.success() {
                            log::debug!("thumbnail generated: {}", dest.to_string_lossy())
                        } else {
                            log::warn!("Failed to generate thumbnail: {}", src.to_string_lossy());
                        }
                    }
                    Err(e) => log::warn!("Failed to run magick command: {e}"),
                }
            }
        })
    }))
    .buffer_unordered(total_threads);

    stream
        .for_each(|result| async {
            if let Err(e) = result {
                log::error!("Thread paniced: {e}");
            }
        })
        .await;
}

pub async fn scan(
    conn: &mut SqliteConnection,
    source_id: String,
    source_path: String,
) -> Result<Vec<Wallpaper>, String> {
    let mut wallpapers_hashmap: WallpapersHashMap = HashMap::new();
    let mut metadata: MetadataHashMap = HashMap::new();
    let mut thumbnail_generation_list: Vec<ThumbnailTask> = Vec::new();

    for entry in WalkDir::new(source_path)
        .into_iter()
        .filter_map(|e| e.ok().filter(|x| x.path().is_file()))
    {
        if entry.file_name().to_string_lossy() == "image_metadata.json" {
            extract_metadata(&mut metadata, entry.path());
            continue;
        }

        match entry.path().extension() {
            Some(ext) => {
                if is_image_extension(ext) {
                    if let Some(signature) = generate_signature(entry.path()) {
                        let thumbnail_path: String = create_thumbnail_path(&signature);

                        thumbnail_generation_list
                            .push((PathBuf::from(entry.path()), PathBuf::from(&thumbnail_path)));

                        let new_wallpaper = NewWallpaper::new(
                            signature.clone(),
                            entry.path().to_string_lossy().to_string(),
                            thumbnail_path,
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
            w.1.keywords = Some(metadata.tags.clone());
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

    if let Err(e) = std::process::Command::new("magick").arg("--help").output() {
        log::error!("Failed to find magick command: {e}");
        return Err(format!("Failed to find magick command: {e}"));
    }

    process_thumbnail_task_list(thumbnail_generation_list).await;

    Ok(wallpapers_list)
}

pub async fn scan_all(conn: &mut SqliteConnection) -> Result<Response<Vec<Wallpaper>>, String> {
    let mut wallpapers_list: Vec<Wallpaper> = Vec::new();

    let wallpaper_sources: Vec<WallpaperSource> =
        match schema::wallpaper_sources::table.get_results::<WallpaperSource>(conn) {
            Ok(v) => v,
            Err(e) => {
                return Err(e.to_string());
            }
        };

    for source in wallpaper_sources {
        match scan(conn, source.id, source.path).await {
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
