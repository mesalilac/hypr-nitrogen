import { Thumbnail } from '@components';
import * as ipc from '@ipc';
import { For, Match, onMount, Switch } from 'solid-js';
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
        const wallpapersRes = await ipc.cmd
            .get_wallpapers()
            .catch(ipc.handleError);
        if (wallpapersRes) wallpapers.set(wallpapersRes.data);

        const activewallpapersRes = await ipc.cmd
            .get_active_wallpapers()
            .catch(ipc.handleError);
        if (!activewallpapersRes) return;

        activeWallpapers.set(activewallpapersRes.data);
    });

    async function handleThumbnailClick(id: string) {
        const lastActiveWallpaper = selectedWallpaper.get();
        selectedWallpaper.set(id);

        // set temporary wallpaper
        const setWallpaperRes = await ipc.cmd
            .set_wallpaper({
                screen: selectedScreen.get(),
                wallpaperId: id,
                mode: selectedMode.get(),
                isTemporary: true,
            })
            .catch(ipc.handleError);

        if (!setWallpaperRes) {
            selectedWallpaper.set(lastActiveWallpaper);
            return;
        }

        const activewallpapersRes = await ipc.cmd
            .get_active_wallpapers()
            .catch(ipc.handleError);

        if (!activewallpapersRes) return;

        activeWallpapers.set(activewallpapersRes.data);
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
