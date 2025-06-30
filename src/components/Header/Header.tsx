import {
    Accessor,
    Setter,
    createSignal,
    For,
    onMount,
    onCleanup,
} from 'solid-js';
import { debounce } from '@solid-primitives/scheduled';
import * as ipc from '@ipc';
import toast from 'solid-toast';

interface Props {
    selectedScreen: Accessor<string>;
    setSelectedScreen: Setter<string>;
    selectedMode: Accessor<ipc.types.Mode>;
    setSelectedMode: Setter<ipc.types.Mode>;
    selectedWallpaper: Accessor<string>;
    setSearchQuery: Setter<string>;
    searchQuery: Accessor<string>;
    filteredItems: Accessor<ipc.types.Wallpaper[]>;
    wallpapers: Accessor<ipc.types.Wallpaper[]>;
    setWallpapers: Setter<ipc.types.Wallpaper[]>;
    setShowSettings: Setter<boolean>;
    setDebouncedSearchQuery: Setter<string>;

    fetch_active_wallpapers: () => void;
}

function Header(props: Props) {
    const [screens, setScreens] = createSignal<string[]>();
    const [scanButtonActive, setScanButtonActive] = createSignal(true);

    const wallpaper_modes: ipc.types.Mode[] = ['default', 'contain', 'tile'];

    const debouncedPerformSearch = debounce(
        (query: string) => props.setDebouncedSearchQuery(query),
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
        const selected_wallpaper = props.selectedWallpaper();

        if (
            (selected_wallpaper && selected_wallpaper !== undefined) ||
            random_wallpaper
        ) {
            toast
                .promise(
                    ipc.set.wallpaper({
                        screen: props.selectedScreen(),
                        wallpaperId: random_wallpaper
                            ? undefined
                            : selected_wallpaper,
                        mode: props.selectedMode(),
                        isTemporary: isTemporary,
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
    }

    function handleSearchChange(e: Event) {
        const value = (e.target as HTMLInputElement).value;

        props.setSearchQuery(value);
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
                props.setWallpapers(res.data);
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
                    value={props.searchQuery()}
                    onInput={handleSearchChange}
                />
                <select
                    onInput={(e) =>
                        props.setSelectedScreen(
                            (e.target as HTMLSelectElement).value,
                        )
                    }
                    disabled={true}
                >
                    <option value='all'>all</option>
                    <For each={screens()}>
                        {(x) => <option value={x}>{x}</option>}
                    </For>
                </select>
                <select
                    onInput={(e) =>
                        props.setSelectedMode(
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
            <span>{props.filteredItems().length} wallpapers</span>
            <div class='header-right'>
                <button onClick={() => setWallpaper(false, true)}>
                    Random
                </button>
                <button onClick={restoreWallpapers}>Restore</button>
                <button disabled={!scanButtonActive()} onClick={scanAll}>
                    Scan
                </button>
                <button onClick={() => props.setShowSettings(true)}>
                    Settings
                </button>
                <button onClick={() => setWallpaper(false, false)}>Save</button>
            </div>
        </div>
    );
}

export default Header;
