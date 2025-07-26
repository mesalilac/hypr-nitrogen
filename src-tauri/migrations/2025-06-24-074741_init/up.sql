CREATE TABLE wallpaper_sources (
    id TEXT NOT NULL PRIMARY KEY,                   -- source ID (nanoid)
    path TEXT NOT NULL UNIQUE,                             -- absolute or relative filesystem path
    -- active INTEGER NOT NULL DEFAULT 1 CHECK(active IN (0,1)) -- 1 = enabled, 0 = disabled
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE wallpapers (
    id TEXT NOT NULL PRIMARY KEY,       -- wallpaper ID (nanoid)
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    signature TEXT NOT NULL UNIQUE,     -- signature (blake3 hash)
    path TEXT NOT NULL,                 -- path to image file
    thumbnail_path TEXT NOT NULL,       -- path to thumbnail file
    resolution TEXT,                    -- e.g., "3840x2160"
    wallpaper_source_id TEXT NOT NULL REFERENCES wallpaper_sources(id) ON DELETE CASCADE,
    keywords TEXT                           -- space-separated category/tags
);

CREATE TABLE active (
    screen TEXT NOT NULL PRIMARY KEY UNIQUE,   -- screen identifier, e.g. "HDMI-1"
    wallpaper_id TEXT NOT NULL REFERENCES wallpapers(id) ON DELETE CASCADE,
    mode TEXT NOT NULL
);
