import {
    createSignal,
    Show,
    onMount,
    For,
    createMemo,
    Switch,
    Match,
    onCleanup,
} from 'solid-js';
import { open } from '@tauri-apps/plugin-dialog';
import toast, { Toaster } from 'solid-toast';
import { debounce } from '@solid-primitives/scheduled';
import { convertFileSrc } from '@tauri-apps/api/core';
import fallbackImage from './assets/fallback-image.svg';
import * as ipc from './ipc';
import './App.css';

function WallpaperSource(props: {
    id: string;
    path: string;
    active: boolean;
    update_source_list_fn: (id: string) => void;
}) {
    const [active, setActive] = createSignal(props.active);

    function handleCheckboxChange(v: boolean) {
        setActive(v);
        ipc.update
            .wallpaper_source_active({ id: props.id, active: active() })
            .then((res) => {
                toast.success(
                    `Source is now ${res.data.active ? 'active' : 'inactive'}`,
                );
            })
            .catch((e) => toast.error(e));
    }

    function handleSourceRemove() {
        ipc.remove
            .wallpaper_source({ id: props.id })
            .then((res) => {
                props.update_source_list_fn(res.data.id);
            })
            .catch((e) => toast.error(e));
    }

    return (
        <div class='wallpaper-source-item'>
            <div>
                <input
                    type='checkbox'
                    checked={active()}
                    onChange={(e) =>
                        handleCheckboxChange(e.currentTarget.checked)
                    }
                />
                {props.path}
            </div>
            <button onClick={handleSourceRemove}>remove</button>
        </div>
    );
}

function WallpaperPreview(props: {
    wallpaper: ipc.types.Wallpaper;
    is_active: boolean;
    is_selected: boolean;
    onClick: () => void;
}) {
    return (
        <div onClick={props.onClick}>
            <img
                id={props.is_selected ? 'wallpaper-preview-selected' : ''}
                class={`preview-image ${props.is_active ? 'wallpaper-preview-active' : ''}`}
                src={convertFileSrc(props.wallpaper.path)}
                loading='lazy'
                onError={(e) => (e.currentTarget.src = fallbackImage)}
            />
        </div>
    );
}

function App() {
    const [scanButtonActive, setScanButtonActive] = createSignal(true);
    const [showSettings, setShowSettings] = createSignal(false);
    const [searchQuery, setSearchQuery] = createSignal('');
    const [debouncedSearchQuery, setDebouncedSearchQuery] = createSignal('');
    const [wallpaperSources, setWallpaperSources] = createSignal<
        ipc.types.WallpaperSource[]
    >([]);
    const [wallpapers, setWallpapers] = createSignal<ipc.types.Wallpaper[]>([]);
    const [screens, setScreens] = createSignal<string[]>();
    const [selectedScreen, setSelectedScreen] = createSignal<string>('all');
    const [selectedMode, setSelectedMode] =
        createSignal<ipc.types.Mode>('default');
    const [selectedWallpaper, setSelectedWallpaper] = createSignal<string>(); // wallpaper_id
    const [activeWallpapers, setActiveWallpapers] = createSignal<
        ipc.types.Active[]
    >([]);

    const wallpaper_modes: ipc.types.Mode[] = ['default', 'contain', 'tile'];

    const debouncedPerformSearch = debounce(
        (query: string) => setDebouncedSearchQuery(query),
        300,
    );

    function handleSearchChange(e: Event) {
        const value = (e.target as HTMLInputElement).value;

        setSearchQuery(value);
        debouncedPerformSearch(value);
    }

    const filteredItems = createMemo(() => {
        const query = debouncedSearchQuery().toLowerCase();

        if (!query) {
            return wallpapers();
        }

        return wallpapers().filter((item) =>
            item.keywords.toLowerCase().includes(query),
        );
    });

    function fetch_active_wallpapers() {
        ipc.get
            .active_wallpapers()
            .then((res) => {
                setActiveWallpapers(res.data);
            })
            .catch((e) => toast.error(e));
    }

    onMount(async () => {
        ipc.get
            .wallpapers()
            .then((res) => {
                setWallpapers(res.data);
            })
            .catch((e) => toast.error(e));
        ipc.get
            .wallpaper_sources()
            .then((res) => {
                setWallpaperSources(res.data);
            })
            .catch((e) => toast.error(e));
        ipc.get
            .screens()
            .then((res) => {
                setScreens(res.data);
            })
            .catch((e) => toast.error(e));
        fetch_active_wallpapers();
    });

    onCleanup(() => {
        debouncedPerformSearch.clear();
    });

    async function addSource() {
        const directory = await open({
            directory: true,
        });

        if (directory) {
            ipc.add
                .wallpaper_source({ path: directory })
                .then((res) => {
                    setWallpaperSources([...wallpaperSources(), res.data!]);
                    toast
                        .promise(
                            ipc.util.scan_source({ sourceId: res.data.id }),
                            {
                                loading: 'Scanning...',
                                success: 'Scan complete',
                                error: 'Scan failed',
                            },
                        )
                        .then((res2) => {
                            setWallpapers([...wallpapers(), ...res2.data]);
                        })
                        .catch((e) => toast.error(e));
                })
                .catch((e) => {
                    toast.error(e);
                });
        }
    }

    function update_source_list(id: string) {
        setWallpaperSources(wallpaperSources().filter((x) => x.id !== id));
        setWallpapers(wallpapers().filter((x) => x.wallpaper_source_id !== id));
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
                    fetch_active_wallpapers();
                })
                .catch((e) => toast.error(e));
        }
    }

    function handleThumbnailClick(id: string) {
        setSelectedWallpaper(id);

        setWallpaper(true, false);
    }

    function restoreWallpapers() {
        toast
            .promise(ipc.util.restore_wallpapers(), {
                loading: 'Restoring wallpapers...',
                success: 'Wallpapers restored',
                error: 'Failed to restore wallpapers',
            })
            .then(() => {
                toast.success('Restored wallpapers');
            })
            .catch((e) => toast.error(e));
    }

    return (
        <main class='container'>
            <Toaster position='bottom-right' />
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
                            setSelectedScreen(
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
                    <button onClick={() => setShowSettings(true)}>
                        Settings
                    </button>
                    <button onClick={() => setWallpaper(false, false)}>
                        Save
                    </button>
                </div>
            </div>
            <div class='preview-list'>
                <For each={filteredItems()}>
                    {(x) => {
                        return (
                            <WallpaperPreview
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
                    <Match
                        when={filteredItems().length === 0 && !searchQuery()}
                    >
                        <div>
                            No wallpapers found. Add a source in the settings.
                        </div>
                    </Match>
                    <Match when={filteredItems().length === 0 && searchQuery()}>
                        <div>{filteredItems().length} wallpapers found</div>
                    </Match>
                </Switch>
            </div>
            <Show when={showSettings()}>
                <div class='settings-container'>
                    <div class='settings'>
                        <div class='wallpaper-sources-list'>
                            <button onClick={addSource}>Add source</button>
                            <div>
                                <For each={wallpaperSources()}>
                                    {(x) => (
                                        <WallpaperSource
                                            id={x.id}
                                            path={x.path}
                                            active={x.active}
                                            update_source_list_fn={
                                                update_source_list
                                            }
                                        />
                                    )}
                                </For>
                                <Switch>
                                    <Match
                                        when={wallpaperSources().length === 0}
                                    >
                                        <div>
                                            No wallpaper sources found. Add a
                                            source.
                                        </div>
                                    </Match>
                                </Switch>
                            </div>
                        </div>
                        <button
                            class='settings-close-btn'
                            onClick={() => setShowSettings(false)}
                        >
                            Close
                        </button>
                    </div>
                </div>
            </Show>
        </main>
    );
}

export default App;
