import { Thumbnail } from '@components';
import * as ipc from '@ipc';
import { For, Match, onMount, Switch } from 'solid-js';
import toast from 'solid-toast';
import { useGlobalContext } from '@/store';

export function ThumbnailsList() {
    const {
        filteredItems,
        selectedScreen,
        selectedMode,
        selectedWallpaper,
        activeWallpapers,
        searchQuery,
        wallpapers,
    } = useGlobalContext();

    onMount(async () => {
        const wallpapersRes = await ipc.get.wallpapers().catch(ipc.handleError);
        if (wallpapersRes) wallpapers.set(wallpapersRes.data);

        const activewallpapersRes = await ipc.get
            .active_wallpapers()
            .catch(ipc.handleError);
        if (activewallpapersRes) activeWallpapers.set(activewallpapersRes.data);
    });

    function handleThumbnailClick(id: string) {
        selectedWallpaper.set(id);

        // set temporary wallpaper
        toast
            .promise(
                ipc.set.wallpaper({
                    screen: selectedScreen.get(),
                    wallpaperId: selectedWallpaper.get(),
                    mode: selectedMode.get(),
                    isTemporary: true,
                }),
                {
                    loading: 'Setting wallpaper...',
                    success: 'Wallpaper set',
                    error: 'Failed to set wallpaper',
                },
            )
            .then(async () => {
                const activewallpapersRes = await ipc.get
                    .active_wallpapers()
                    .catch(ipc.handleError);
                if (activewallpapersRes)
                    activeWallpapers.set(activewallpapersRes.data);
            })
            .catch(ipc.handleError);
    }

    return (
        <div class='thumbnails-list'>
            <For each={filteredItems()}>
                {(x) => {
                    return (
                        <Thumbnail
                            is_active={activeWallpapers
                                .get()
                                .some(
                                    (y) =>
                                        y.wallpaper_id === x.id &&
                                        (y.screen === selectedScreen.get() ||
                                            selectedScreen.get() === 'all'),
                                )}
                            is_selected={x.id === selectedWallpaper.get()}
                            wallpaper={x}
                            onClick={() => handleThumbnailClick(x.id)}
                        />
                    );
                }}
            </For>
            <Switch>
                <Match
                    when={filteredItems().length === 0 && !searchQuery.get()}
                >
                    <div>
                        No wallpapers found. Add a source in the settings.
                    </div>
                </Match>
                <Match when={filteredItems().length === 0 && searchQuery.get()}>
                    <div>{filteredItems().length} wallpapers found</div>
                </Match>
            </Switch>
        </div>
    );
}
