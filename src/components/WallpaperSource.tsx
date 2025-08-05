import * as ipc from '@ipc';
import { createSignal, Setter } from 'solid-js';
import toast from 'solid-toast';
import { RemoveIcon } from '@/icons';
import { useGlobalContext } from '@/store';
import { SignalObject } from '@/utils';

interface Props {
    id: string;
    path: string;
    active: boolean;
    wallpaperSources: SignalObject<ipc.types.WallpaperSource[]>;
    setWallpapers: Setter<ipc.types.Wallpaper[]>;
}

export function WallpaperSource(props: Props) {
    const [active, setActive] = createSignal(props.active);
    const { wallpapers } = useGlobalContext();

    async function handleCheckboxChange(v: boolean) {
        setActive(v);
        const updateWallpaperSourceActiveRes = await ipc.cmd
            .update_wallpaper_source_active({ id: props.id, active: active() })
            .catch(ipc.handleError);

        if (!updateWallpaperSourceActiveRes) return;

        toast.success(
            `Source is now ${updateWallpaperSourceActiveRes.data.active ? 'active' : 'inactive'}`,
        );
        const wallpapersRes = await ipc.cmd
            .get_wallpapers()
            .catch(ipc.handleError);
        if (!wallpapersRes) return;

        props.setWallpapers(wallpapersRes.data);

        const wallpaperSources = await ipc.cmd
            .get_wallpaper_sources()
            .catch(ipc.handleError);
        if (!wallpaperSources) return;

        props.wallpaperSources.set(wallpaperSources.data);
    }

    async function handleSourceRemove() {
        const removeWallpaperSourceRes = await ipc.cmd
            .remove_wallpaper_source({ id: props.id })
            .catch(ipc.handleError);

        if (!removeWallpaperSourceRes) return;

        props.wallpaperSources.set(
            props.wallpaperSources
                .get()
                .filter((x) => x.id !== removeWallpaperSourceRes.data.id),
        );
        wallpapers.set(
            wallpapers
                .get()
                .filter(
                    (x) =>
                        x.wallpaper_source_id !==
                        removeWallpaperSourceRes.data.id,
                ),
        );
    }

    return (
        <div class='wallpaper-source-item'>
            <input
                type='checkbox'
                checked={active()}
                onChange={(e) => handleCheckboxChange(e.currentTarget.checked)}
            />
            <span>{props.path}</span>
            <button onClick={handleSourceRemove} title='Remove source'>
                <RemoveIcon />
            </button>
        </div>
    );
}
