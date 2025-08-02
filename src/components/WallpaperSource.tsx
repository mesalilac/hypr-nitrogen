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

    function handleCheckboxChange(v: boolean) {
        setActive(v);
        ipc.update
            .wallpaper_source_active({ id: props.id, active: active() })
            .then(async (res) => {
                toast.success(
                    `Source is now ${res.data.active ? 'active' : 'inactive'}`,
                );
                const wallpapersRes = await ipc.get
                    .wallpapers()
                    .catch(ipc.handleError);
                if (wallpapersRes) props.setWallpapers(wallpapersRes.data);

                const wallpaperSources = await ipc.get
                    .wallpaper_sources()
                    .catch(ipc.handleError);
                if (wallpaperSources)
                    props.wallpaperSources.set(wallpaperSources.data);
            })
            .catch((e) => toast.error(e));
    }

    async function handleSourceRemove() {
        const removeWallpaperSourceRes = await ipc.remove
            .wallpaper_source({ id: props.id })
            .catch(ipc.handleError);

        if (removeWallpaperSourceRes) {
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
