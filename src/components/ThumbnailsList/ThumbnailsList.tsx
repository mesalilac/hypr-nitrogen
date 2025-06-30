import { For, Switch, Match, Accessor, Setter } from 'solid-js';
import toast from 'solid-toast';
import * as ipc from '@ipc';
import Thumbnail from '@components/Thumbnail';

interface Props {
    activeWallpapers: Accessor<ipc.types.Active[]>;
    filteredItems: Accessor<ipc.types.Wallpaper[]>;
    selectedScreen: Accessor<string>;
    selectedWallpaper: Accessor<string>;
    selectedMode: Accessor<ipc.types.Mode>;
    searchQuery: Accessor<string>;
    setSelectedWallpaper: Setter<string>;
    setSelectedScreen: Setter<string>;

    fetch_active_wallpapers: () => void;
}

function ThumbnailsList(props: Props) {
    function handleThumbnailClick(id: string) {
        props.setSelectedWallpaper(id);

        // set temporary wallpaper
        toast
            .promise(
                ipc.set.wallpaper({
                    screen: props.selectedScreen(),
                    wallpaperId: props.selectedWallpaper(),
                    mode: props.selectedMode(),
                    isTemporary: true,
                }),
                {
                    loading: 'Setting wallpaper...',
                    success: 'Wallpaper set',
                    error: 'Failed to set wallpaper',
                },
            )
            .then(() => {
                props.fetch_active_wallpapers();
            })
            .catch((e) => toast.error(e));
    }

    return (
        <div class='thumbnails-list'>
            <For each={props.filteredItems()}>
                {(x) => {
                    return (
                        <Thumbnail
                            is_active={props
                                .activeWallpapers()
                                .some(
                                    (y) =>
                                        y.wallpaper_id === x.id &&
                                        (y.screen === props.selectedScreen() ||
                                            props.selectedScreen() === 'all'),
                                )}
                            is_selected={x.id === props.selectedWallpaper()}
                            wallpaper={x}
                            onClick={() => handleThumbnailClick(x.id)}
                        />
                    );
                }}
            </For>
            <Switch>
                <Match
                    when={
                        props.filteredItems().length === 0 &&
                        !props.searchQuery()
                    }
                >
                    <div>
                        No wallpapers found. Add a source in the settings.
                    </div>
                </Match>
                <Match
                    when={
                        props.filteredItems().length === 0 &&
                        props.searchQuery()
                    }
                >
                    <div>{props.filteredItems().length} wallpapers found</div>
                </Match>
            </Switch>
        </div>
    );
}

export default ThumbnailsList;
