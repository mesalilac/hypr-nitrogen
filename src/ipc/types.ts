export type Mode = 'default' | 'contain' | 'tile';

export interface Active {
    screen: string;
    wallpaper_id: string;
    mode: Mode;
}

export interface WallpaperSource {
    id: string;
    path: string;
    active: boolean;
}

export interface Wallpaper {
    id: string;
    signature: string;
    path: string;
    thumbnail_path: string;
    resolution: string;
    wallpaper_source_id: string;
    keywords: string;
}

export type Error = string;

export interface Response<T> {
    data: T;
}
