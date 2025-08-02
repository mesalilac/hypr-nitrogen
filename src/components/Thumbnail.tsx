import fallbackImage from '@assets/fallback-image.svg';
import { FullHeartIcon, HeartIcon } from '@icons';
import * as ipc from '@ipc';
import { createVisibilityObserver } from '@solid-primitives/intersection-observer';
import { convertFileSrc } from '@tauri-apps/api/core';
import { createSignal, Match, Switch } from 'solid-js';

interface Props {
    wallpaper: ipc.types.Wallpaper;
    is_active: boolean;
    is_selected: boolean;
    onClick: () => void;
}

export function Thumbnail(props: Props) {
    const [favorite, setFavorite] = createSignal(props.wallpaper.is_favorite);
    let imgRef: HTMLImageElement | undefined;

    const useVisibilityObserver = createVisibilityObserver({ threshold: 0.2 });

    const visible = useVisibilityObserver(() => imgRef);

    async function handleFavoriteClick() {
        const wallpaperFavoriteRes = await ipc.update
            .wallpaper_favorite({
                id: props.wallpaper.id,
                newValue: !favorite(),
            })
            .catch(ipc.handleError);

        if (wallpaperFavoriteRes)
            setFavorite(wallpaperFavoriteRes.data.is_favorite);
    }

    return (
        <div class='thumbnail'>
            <img
                id={props.is_selected ? 'thumbnail-img-selected' : ''}
                class={`thumbnail-img ${props.is_active ? 'thumbnail-img-active' : ''}`}
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
            <button class='thumbnail-favorite' onClick={handleFavoriteClick}>
                <Switch>
                    <Match when={favorite()}>
                        <FullHeartIcon />
                    </Match>
                    <Match when={!favorite()}>
                        <HeartIcon />
                    </Match>
                </Switch>
            </button>
        </div>
    );
}
