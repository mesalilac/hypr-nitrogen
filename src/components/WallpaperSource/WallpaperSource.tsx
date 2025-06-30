import { createSignal } from 'solid-js';
import './WallpaperSource.css';
import * as ipc from '../../ipc';
import toast from 'solid-toast';

interface Props {
    id: string;
    path: string;
    active: boolean;
    update_source_list_fn: (id: string) => void;
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
            <div>
                <input
                    type='checkbox'
                    checked={active()}
                    onChange={(e) =>
                        handleCheckboxChange(e.currentTarget.checked)
                    }
                />
                {props.path}
            </div>
            <button onClick={handleSourceRemove}>remove</button>
        </div>
    );
}

export default WallpaperSource;
