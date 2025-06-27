import { invoke } from '@tauri-apps/api/core';
import {
    Response,
    WallpaperSources,
    Mode,
    Active,
    Wallpapers,
} from './types.ts';

export function get_screens(): Promise<Response<string[]>> {
    return invoke('get_screens');
}

export function add_wallpaper_source(props: {
    path: string;
}): Promise<Response<WallpaperSources>> {
    return invoke('add_wallpaper_source', props);
}

export function get_wallpaper_sources(): Promise<Response<WallpaperSources[]>> {
    return invoke('get_wallpaper_sources');
}

export function update_wallpaper_source_active(props: {
    id: string;
    active: boolean;
}): Promise<Response<WallpaperSources>> {
    return invoke('update_wallpaper_source_active', props);
}

export function remove_wallpaper_source(props: {
    id: string;
}): Promise<Response<WallpaperSources>> {
    return invoke('remove_wallpaper_source', props);
}

export function scan_source(props: {
    sourceId: string;
}): Promise<Response<Wallpapers[]>> {
    return invoke('scan_source', props);
}

export function scan_all_sources(): Promise<Response<Wallpapers[]>> {
    return invoke('scan_all_sources');
}

export function get_wallpapers(): Promise<Response<Wallpapers[]>> {
    return invoke('get_wallpapers');
}

export function set_wallpaper(props: {
    screens: string[];
    wallpaperId: string;
    mode: Mode;
}): Promise<Response<Active>> {
    return invoke('set_wallpaper', props);
}

export function get_active_wallpapers(): Promise<Response<Active[]>> {
    return invoke('get_active_wallpapers');
}
