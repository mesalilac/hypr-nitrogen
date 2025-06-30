import { invoke } from '@tauri-apps/api/core';
import * as types from './types.ts';

export { types };

export const set = {
    wallpaper(props: {
        screen: string;
        wallpaperId: string | undefined;
        mode: types.Mode;
        isTemporary: boolean;
    }): Promise<types.Response<types.Active>> {
        return invoke('set_wallpaper', props);
    },
};

export const add = {
    wallpaper_source(props: {
        path: string;
    }): Promise<types.Response<types.WallpaperSource>> {
        return invoke('add_wallpaper_source', props);
    },
};

export const get = {
    screens(): Promise<types.Response<string[]>> {
        return invoke('get_screens');
    },
    wallpaper_sources(): Promise<types.Response<types.WallpaperSource[]>> {
        return invoke('get_wallpaper_sources');
    },
    wallpapers(): Promise<types.Response<types.Wallpaper[]>> {
        return invoke('get_wallpapers');
    },
    active_wallpapers(): Promise<types.Response<types.Active[]>> {
        return invoke('get_active_wallpapers');
    },
};

export const remove = {
    wallpaper_source(props: {
        id: string;
    }): Promise<types.Response<types.WallpaperSource>> {
        return invoke('remove_wallpaper_source', props);
    },
};

export const update = {
    wallpaper_source_active(props: {
        id: string;
        active: boolean;
    }): Promise<types.Response<types.WallpaperSource>> {
        return invoke('update_wallpaper_source_active', props);
    },
};

export const util = {
    scan_source(props: {
        sourceId: string;
    }): Promise<types.Response<types.Wallpaper[]>> {
        return invoke('scan_source', props);
    },
    scan_all_sources(): Promise<types.Response<types.Wallpaper[]>> {
        return invoke('scan_all_sources');
    },
    restore_wallpapers(): Promise<types.Response<boolean>> {
        return invoke('restore_wallpapers');
    },
};
