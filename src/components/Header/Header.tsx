import { createSignal, For, onMount, onCleanup } from 'solid-js';
import { debounce } from '@solid-primitives/scheduled';
import * as ipc from '@ipc';
import toast from 'solid-toast';
import { useGlobalContext } from '@/store';

function Header() {
    const {
        showSettings,
        filteredItems,
        selectedScreen,
        selectedMode,
        selectedWallpaper,
        searchQuery,
        debouncedSearchQuery,
        activeWallpapers,
        wallpapers,
    } = useGlobalContext();
    const [screens, setScreens] = createSignal<string[]>();
    const [scanButtonActive, setScanButtonActive] = createSignal(true);

    const wallpaper_modes: ipc.types.Mode[] = ['default', 'contain', 'tile'];

    const debouncedPerformSearch = debounce(
        (query: string) => debouncedSearchQuery.set(query),
        300,
    );

    onMount(() => {
        ipc.get
            .screens()
            .then((res) => {
                setScreens(res.data);
            })
            .catch((e) => toast.error(e));
    });

    onCleanup(() => {
        debouncedPerformSearch.clear();
    });

    function setWallpaper(isTemporary: boolean, random_wallpaper: boolean) {
        const selected_wallpaper = selectedWallpaper.get();

        if (
            (selected_wallpaper && selected_wallpaper !== undefined) ||
            random_wallpaper
        ) {
            toast
                .promise(
                    ipc.set.wallpaper({
                        screen: selectedScreen.get(),
                        wallpaperId: random_wallpaper
                            ? undefined
                            : selected_wallpaper,
                        mode: selectedMode.get(),
                        isTemporary: isTemporary,
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
                            activeWallpapers.set(res.data);
                        })
                        .catch((e) => toast.error(e));
                })
                .catch((e) => toast.error(e));
        }
    }

    function handleSearchChange(e: Event) {
        const value = (e.target as HTMLInputElement).value;

        searchQuery.set(value);
        debouncedPerformSearch(value);
    }

    function restoreWallpapers() {
        toast
            .promise(ipc.util.restore_wallpapers(), {
                loading: 'Restoring wallpapers...',
                success: 'Wallpapers restored',
                error: 'Failed to restore wallpapers',
            })
            .catch((e) => toast.error(e));
    }

    function scanAll() {
        setScanButtonActive(false);
        toast
            .promise(ipc.util.scan_all_sources(), {
                loading: 'Scanning all sources...',
                success: 'Scan complete',
                error: 'Scan failed',
            })
            .then((res) => {
                wallpapers.set(res.data);
            })
            .catch((e) => toast.error(e))
            .finally(() => setScanButtonActive(true));
    }

    return (
        <div class='header'>
            <div>
                <input
                    type='text'
                    placeholder='Search...'
                    value={searchQuery.get()}
                    onInput={handleSearchChange}
                />
                <select
                    onInput={(e) =>
                        selectedScreen.set(
                            (e.target as HTMLSelectElement).value,
                        )
                    }
                >
                    <option value='all'>all</option>
                    <For each={screens()}>
                        {(x) => <option value={x}>{x}</option>}
                    </For>
                </select>
                <select
                    onInput={(e) =>
                        selectedMode.set(
                            (e.target as HTMLSelectElement)
                                .value as ipc.types.Mode,
                        )
                    }
                >
                    <For each={wallpaper_modes}>
                        {(x) => <option value={x}>{x}</option>}
                    </For>
                </select>
            </div>
            <span>{filteredItems().length} wallpapers</span>
            <div class='header-right'>
                <button onClick={() => setWallpaper(false, true)}>
                    Random
                </button>
                <button onClick={restoreWallpapers}>Restore</button>
                <button disabled={!scanButtonActive()} onClick={scanAll}>
                    Scan
                </button>
                <button onClick={() => showSettings.set(true)}>Settings</button>
                <button onClick={() => setWallpaper(false, false)}>Save</button>
            </div>
        </div>
    );
}

export default Header;
