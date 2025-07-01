import { convertFileSrc } from '@tauri-apps/api/core';
import fallbackImage from '@assets/fallback-image.svg';
import * as ipc from '@ipc';

interface Props {
    wallpaper: ipc.types.Wallpaper;
    is_active: boolean;
    is_selected: boolean;
    onClick: () => void;
}

function Thumbnail(props: Props) {
    return (
        <div onClick={props.onClick}>
            <img
                id={props.is_selected ? 'thumbnail-selected' : ''}
                class={`thumbnail ${props.is_active ? 'thumbnail-active' : ''}`}
                src={convertFileSrc(props.wallpaper.thumbnail_path)}
                loading='lazy'
                onError={(e) => (e.currentTarget.src = fallbackImage)}
            />
        </div>
    );
}

export default Thumbnail;
