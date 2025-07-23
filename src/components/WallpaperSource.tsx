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
            .then((res) => {
                toast.success(
                    `Source is now ${res.data.active ? 'active' : 'inactive'}`,
                );
                ipc.get
                    .wallpapers()
                    .then((res) => {
                        props.setWallpapers(res.data);
                    })
                    .catch((e) => toast.error(e));
                ipc.get
                    .wallpaper_sources()
                    .then((res) => {
                        props.wallpaperSources.set(res.data);
                    })
                    .catch((e) => toast.error(e));
            })
            .catch((e) => toast.error(e));
    }

    function handleSourceRemove() {
        ipc.remove
            .wallpaper_source({ id: props.id })
            .then((res) => {
                props.wallpaperSources.set(
                    props.wallpaperSources
                        .get()
                        .filter((x) => x.id !== res.data.id),
                );
                wallpapers.set(
                    wallpapers
                        .get()
                        .filter((x) => x.wallpaper_source_id !== res.data.id),
                );
            })
            .catch((e) => toast.error(e));
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
