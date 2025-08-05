import { invoke } from '@tauri-apps/api/core';
import toast from 'solid-toast';
import * as types from './types.ts';

export { types };

export function handleError(err: string) {
    console.error(err);
    toast.error(err);
}
type CmdReturn<T> = Promise<types.Response<T>>;

export const cmd = {
    set_wallpaper: (args: {
        screen: string;
        wallpaperId?: string;
        mode: types.Mode;
        isTemporary: boolean;
    }): CmdReturn<types.Active> => invoke('cmd_set_wallpaper', args),
    add_wallpaper_source: (args: {
        path: string;
    }): CmdReturn<types.WallpaperSource> =>
        invoke('cmd_add_wallpaper_source', args),
    get_screens: (): CmdReturn<string[]> => invoke('cmd_get_screens'),
    get_wallpaper_sources: (): CmdReturn<types.WallpaperSource[]> =>
        invoke('cmd_get_wallpaper_sources'),
    get_wallpapers: (): CmdReturn<types.Wallpaper[]> =>
        invoke('cmd_get_wallpapers'),
    get_active_wallpapers: (): CmdReturn<types.Active[]> =>
        invoke('cmd_get_active_wallpapers'),
    remove_wallpaper_source: (args: {
        id: string;
    }): CmdReturn<types.WallpaperSource> =>
        invoke('cmd_remove_wallpaper_source', args),
    update_wallpaper_source_active: (args: {
        id: string;
        active: boolean;
    }): CmdReturn<types.WallpaperSource> =>
        invoke('cmd_update_wallpaper_source_active', args),
    update_wallpaper_favorite: (args: {
        id: string;
        newValue: boolean;
    }): CmdReturn<types.Wallpaper> =>
        invoke('cmd_update_wallpaper_favorite', args),
    scan_source: (args: { sourceId: string }): CmdReturn<types.Wallpaper[]> =>
        invoke('cmd_scan_source', args),
    scan_all_sources: (): CmdReturn<types.Wallpaper[]> =>
        invoke('cmd_scan_all_sources'),
    restore_wallpapers: (): CmdReturn<boolean> =>
        invoke('cmd_restore_wallpapers'),
};
