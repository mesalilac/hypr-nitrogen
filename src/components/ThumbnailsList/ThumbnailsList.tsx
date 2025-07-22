import { For, Switch, Match, onMount } from 'solid-js';
import toast from 'solid-toast';
import * as ipc from '@ipc';
import Thumbnail from '@components/Thumbnail';
import { useGlobalContext } from '@/store';

function ThumbnailsList() {
    const {
        setWallpapers,
        setActiveWallpapers,
        filteredItems,
        setSelectedWallpaper,
        selectedScreen,
        selectedMode,
        selectedWallpaper,
        activeWallpapers,
        searchQuery,
    } = useGlobalContext();

    onMount(async () => {
        ipc.get
            .wallpapers()
            .then((res) => {
                setWallpapers(res.data);
            })
            .catch((e) => toast.error(e));
        ipc.get
            .active_wallpapers()
            .then((res) => {
                setActiveWallpapers(res.data);
            })
            .catch((e) => toast.error(e));
    });

    function handleThumbnailClick(id: string) {
        setSelectedWallpaper(id);

        // set temporary wallpaper
        toast
            .promise(
                ipc.set.wallpaper({
                    screen: selectedScreen(),
                    wallpaperId: selectedWallpaper(),
                    mode: selectedMode(),
                    isTemporary: true,
                }),
                {
                    loading: 'Setting wallpaper...',
                    success: 'Wallpaper set',
                    error: 'Failed to set wallpaper',
                },
            )
            .then(() => {
                ipc.get
                    .active_wallpapers()
                    .then((res) => {
                        setActiveWallpapers(res.data);
                    })
                    .catch((e) => toast.error(e));
            })
            .catch((e) => toast.error(e));
    }

    return (
        <div class='thumbnails-list'>
            <For each={filteredItems()}>
                {(x) => {
                    return (
                        <Thumbnail
                            is_active={activeWallpapers().some(
                                (y) =>
                                    y.wallpaper_id === x.id &&
                                    (y.screen === selectedScreen() ||
                                        selectedScreen() === 'all'),
                            )}
                            is_selected={x.id === selectedWallpaper()}
                            wallpaper={x}
                            onClick={() => handleThumbnailClick(x.id)}
                        />
                    );
                }}
            </For>
            <Switch>
                <Match when={filteredItems().length === 0 && !searchQuery()}>
                    <div>
                        No wallpapers found. Add a source in the settings.
                    </div>
                </Match>
                <Match when={filteredItems().length === 0 && searchQuery()}>
                    <div>{filteredItems().length} wallpapers found</div>
                </Match>
            </Switch>
        </div>
    );
}

export default ThumbnailsList;
