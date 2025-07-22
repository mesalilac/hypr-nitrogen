import { createSignal, For, onMount, onCleanup } from 'solid-js';
import { debounce } from '@solid-primitives/scheduled';
import * as ipc from '@ipc';
import toast from 'solid-toast';
import { useGlobalContext } from '@/store';

function Header() {
    const {
        setSelectedScreen,
        setSelectedMode,
        setWallpapers,
        setActiveWallpapers,
        filteredItems,
        selectedScreen,
        selectedMode,
        selectedWallpaper,
        searchQuery,
        setDebouncedSearchQuery,
        setShowSettings,
        setSearchQuery,
    } = useGlobalContext();
    const [screens, setScreens] = createSignal<string[]>();
    const [scanButtonActive, setScanButtonActive] = createSignal(true);

    const wallpaper_modes: ipc.types.Mode[] = ['default', 'contain', 'tile'];

    const debouncedPerformSearch = debounce(
        (query: string) => setDebouncedSearchQuery(query),
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
        const selected_wallpaper = selectedWallpaper();

        if (
            (selected_wallpaper && selected_wallpaper !== undefined) ||
            random_wallpaper
        ) {
            toast
                .promise(
                    ipc.set.wallpaper({
                        screen: selectedScreen(),
                        wallpaperId: random_wallpaper
                            ? undefined
                            : selected_wallpaper,
                        mode: selectedMode(),
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
                            setActiveWallpapers(res.data);
                        })
                        .catch((e) => toast.error(e));
                })
                .catch((e) => toast.error(e));
        }
    }

    function handleSearchChange(e: Event) {
        const value = (e.target as HTMLInputElement).value;

        setSearchQuery(value);
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
                setWallpapers(res.data);
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
                    value={searchQuery()}
                    onInput={handleSearchChange}
                />
                <select
                    onInput={(e) =>
                        setSelectedScreen((e.target as HTMLSelectElement).value)
                    }
                >
                    <option value='all'>all</option>
                    <For each={screens()}>
                        {(x) => <option value={x}>{x}</option>}
                    </For>
                </select>
                <select
                    onInput={(e) =>
                        setSelectedMode(
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
                <button onClick={() => setShowSettings(true)}>Settings</button>
                <button onClick={() => setWallpaper(false, false)}>Save</button>
            </div>
        </div>
    );
}

export default Header;
