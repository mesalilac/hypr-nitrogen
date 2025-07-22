import * as ipc from '@ipc';
import { createSignal, Setter } from 'solid-js';
import toast from 'solid-toast';

interface Props {
    id: string;
    path: string;
    active: boolean;
    update_source_list_fn: (id: string) => void;
    setWallpapers: Setter<ipc.types.Wallpaper[]>;
}

function WallpaperSource(props: Props) {
    const [active, setActive] = createSignal(props.active);

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
            })
            .catch((e) => toast.error(e));
    }

    function handleSourceRemove() {
        ipc.remove
            .wallpaper_source({ id: props.id })
            .then((res) => {
                props.update_source_list_fn(res.data.id);
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
            <button onClick={handleSourceRemove}>x</button>
        </div>
    );
}

export default WallpaperSource;
