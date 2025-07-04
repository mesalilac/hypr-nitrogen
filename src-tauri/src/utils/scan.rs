use crate::database::models::*;
use crate::ipc::Response;
use crate::schema;
use crate::utils::fs::get_cache_dir;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use image::imageops::thumbnail;
use image::{ImageFormat, ImageReader};
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{process, thread};
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
                log::error!("Error reading metadata file: {}", e);
            }
        },
        Err(e) => {
            log::error!("Error reading metadata file: {}", e);
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
            log::error!("Error reading file: {}", e);
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
            log::error!("Failed to create thumbnails dir: {}", e);
        }
    }

    thumbnail_path.push(format!("{}.jpeg", signature));
    thumbnail_path.to_string_lossy().to_string()
}

fn process_thumbnail_task_list(list: Vec<ThumbnailTask>) {
    let total_threads = thread::available_parallelism()
        .map(|x| x.get())
        .unwrap_or(4);

    let mut batches: Vec<Vec<ThumbnailTask>> = vec![Vec::new(); total_threads];

    for (i, item) in list.into_iter().enumerate() {
        let batch_index = i % total_threads;
        batches[batch_index].push(item);
    }

    let mut handles = Vec::new();

    for batch in batches {
        let handle = thread::spawn(move || {
            for (target_image_path, thumbnail_path) in batch {
                if thumbnail_path.exists() {
                    continue;
                }

                if let Ok(image) = ImageReader::open(&target_image_path) {
                    if let Ok(decoded_image) = image.decode() {
                        // `.to_rgb8` Because The JPEG file format doesn't support alpha (see https://github.com/image-rs/image/issues/2211)
                        let new_image = thumbnail(&decoded_image.to_rgb8(), 400, 200);

                        match new_image.save_with_format(&thumbnail_path, ImageFormat::Jpeg) {
                            Ok(_) => log::info!(
                                "Thumbnail generated: '{}'",
                                thumbnail_path.to_string_lossy()
                            ),
                            Err(e) => log::error!("Failed to generate thumbnail(Image): '{}'", e),
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if handle.join().is_err() {
            log::error!("Failed to join thread");
        }
    }
}

pub fn scan(
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
        if entry.file_name().to_string_lossy() == "wallpaper_metadata.json" {
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

    process_thumbnail_task_list(thumbnail_generation_list);

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
