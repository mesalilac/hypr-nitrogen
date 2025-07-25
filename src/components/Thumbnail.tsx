import fallbackImage from '@assets/fallback-image.svg';
import * as ipc from '@ipc';
import { createVisibilityObserver } from '@solid-primitives/intersection-observer';
import { convertFileSrc } from '@tauri-apps/api/core';

interface Props {
    wallpaper: ipc.types.Wallpaper;
    is_active: boolean;
    is_selected: boolean;
    onClick: () => void;
}

export function Thumbnail(props: Props) {
    let imgRef: HTMLImageElement | undefined;

    const useVisibilityObserver = createVisibilityObserver({ threshold: 0.2 });

    const visible = useVisibilityObserver(() => imgRef);

    return (
        <img
            id={props.is_selected ? 'thumbnail-selected' : ''}
            class={`thumbnail ${props.is_active ? 'thumbnail-active' : ''}`}
            src={
                visible()
                    ? convertFileSrc(props.wallpaper.thumbnail_path)
                    : undefined
            }
            onClick={props.onClick}
            loading='lazy'
            ref={imgRef}
            onError={(e) => (e.currentTarget.src = fallbackImage)}
            title={props.wallpaper.keywords}
        />
    );
}
