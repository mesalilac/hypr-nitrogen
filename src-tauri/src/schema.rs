// @generated automatically by Diesel CLI.

diesel::table! {
    active (screen) {
        screen -> Text,
        wallpaper_id -> Text,
        mode -> Text,
    }
}

diesel::table! {
    wallpaper_sources (id) {
        id -> Text,
        path -> Text,
        active -> Bool,
    }
}

diesel::table! {
    wallpapers (id) {
        id -> Text,
        is_favorite -> Bool,
        signature -> Text,
        path -> Text,
        thumbnail_path -> Text,
        resolution -> Nullable<Text>,
        wallpaper_source_id -> Text,
        keywords -> Nullable<Text>,
    }
}

diesel::joinable!(active -> wallpapers (wallpaper_id));
diesel::joinable!(wallpapers -> wallpaper_sources (wallpaper_source_id));

diesel::allow_tables_to_appear_in_same_query!(
    active,
    wallpaper_sources,
    wallpapers,
);
