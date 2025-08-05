import {
    RandomIcon,
    RestoreIcon,
    SaveIcon,
    ScanIcon,
    SearchIcon,
    SettingsIcon,
} from '@icons';
import * as ipc from '@ipc';
import { debounce } from '@solid-primitives/scheduled';
import { createSignal, For, onCleanup, onMount } from 'solid-js';
import toast from 'solid-toast';
import { useGlobalContext } from '@/store';

export function Header() {
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

    onMount(async () => {
        const screensRes = await ipc.cmd.get_screens().catch(ipc.handleError);
        if (!screensRes) return;

        setScreens(screensRes.data);
    });

    onCleanup(() => {
        debouncedPerformSearch.clear();
    });

    async function setWallpaper(
        isTemporary: boolean,
        random_wallpaper: boolean,
    ) {
        const selected_wallpaper = selectedWallpaper.get();

        if (
            (selected_wallpaper && selected_wallpaper !== undefined) ||
            random_wallpaper
        ) {
            const setWallpaperRes = await ipc.cmd
                .set_wallpaper({
                    screen: selectedScreen.get(),
                    wallpaperId: random_wallpaper
                        ? undefined
                        : selected_wallpaper,
                    mode: selectedMode.get(),
                    isTemporary: isTemporary,
                })
                .catch(ipc.handleError);

            if (!setWallpaperRes) return;

            const activeWallpapersRes = await ipc.cmd
                .get_active_wallpapers()
                .catch(ipc.handleError);

            if (!activeWallpapersRes) return;

            activeWallpapers.set(activeWallpapersRes.data);
        }
    }

    function handleSearchChange(e: Event) {
        const value = (e.target as HTMLInputElement).value;

        searchQuery.set(value);
        debouncedPerformSearch(value);
    }

    async function restoreWallpapers() {
        const restoreWallpapersRes = await ipc.cmd
            .restore_wallpapers()
            .catch(ipc.handleError);

        if (!restoreWallpapersRes) return;

        toast.success(`Restored wallpapers`);
    }

    function scanAll() {
        setScanButtonActive(false);
        toast
            .promise(ipc.cmd.scan_all_sources(), {
                loading: 'Scanning all sources...',
                success: 'Scan complete',
                error: 'Scan failed',
            })
            .then((res) => {
                wallpapers.set(res.data);
            })
            .catch(ipc.handleError)
            .finally(() => setScanButtonActive(true));
    }

    return (
        <div class='header'>
            <div class='header-left'>
                <div>
                    <input
                        type='text'
                        placeholder='Search...'
                        value={searchQuery.get()}
                        onInput={handleSearchChange}
                        style={{
                            padding: '10px 10px 10px 40px',
                        }}
                        title='Search wallpapers'
                    />
                    <SearchIcon
                        style={{
                            position: 'absolute',
                            left: '20px',
                            top: '50%',
                            transform: 'translateY(-50%)',
                            width: '18px',
                            height: '18px',
                        }}
                    />
                </div>
                <select
                    onInput={(e) =>
                        selectedScreen.set(
                            (e.target as HTMLSelectElement).value,
                        )
                    }
                    title='Select screen'
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
                    title='Select wallpaper mode'
                >
                    <For each={wallpaper_modes}>
                        {(x) => <option value={x}>{x}</option>}
                    </For>
                </select>
            </div>
            <span>{filteredItems().length} wallpapers</span>
            <div class='header-right'>
                <button
                    onClick={() => setWallpaper(false, true)}
                    title='Select a random wallpaper'
                >
                    <RandomIcon />
                </button>
                <button
                    onClick={restoreWallpapers}
                    title='Restore active wallpapers'
                >
                    <RestoreIcon />
                </button>
                <button
                    disabled={!scanButtonActive()}
                    onClick={scanAll}
                    title='Scan all sources'
                >
                    <ScanIcon />
                </button>
                <button
                    onClick={() => showSettings.set(true)}
                    title='Open settings menu'
                >
                    <SettingsIcon />
                </button>
                <button
                    onClick={() => setWallpaper(false, false)}
                    title='Save selected wallpaper'
                >
                    <SaveIcon />
                </button>
            </div>
        </div>
    );
}
