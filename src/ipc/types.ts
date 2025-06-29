export type Mode = 'default' | 'contain' | 'tile';

export interface Active {
    screen: string;
    wallpaper_id: string;
    mode: Mode;
}

export interface WallpaperSources {
    id: string;
    path: string;
    active: boolean;
}

export interface Wallpapers {
    id: string;
    signature: string;
    path: string;
    resolution: string;
    wallpaper_source_id: string;
    keywords: string;
}

export type Error = string;

export interface Response<T> {
    data: T;
}
