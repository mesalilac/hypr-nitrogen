import {
    createSignal,
    Show,
    onMount,
    For,
    createMemo,
    Switch,
    Match,
} from 'solid-js';
import toast, { Toaster } from 'solid-toast';
import { convertFileSrc } from '@tauri-apps/api/core';
import fallbackImage from './assets/fallback-image.svg';
import Settings from './components/Settings';
import Header from './components/Header';
import * as ipc from './ipc';
import './App.css';

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
    const [selectedScreen, setSelectedScreen] = createSignal<string>('all');
    const [showSettings, setShowSettings] = createSignal(false);
    const [searchQuery, setSearchQuery] = createSignal('');
    const [selectedMode, setSelectedMode] =
        createSignal<ipc.types.Mode>('default');
    const [debouncedSearchQuery, setDebouncedSearchQuery] = createSignal('');
    const [wallpapers, setWallpapers] = createSignal<ipc.types.Wallpaper[]>([]);
    const [selectedWallpaper, setSelectedWallpaper] = createSignal<string>(''); // wallpaper_id
    const [activeWallpapers, setActiveWallpapers] = createSignal<
        ipc.types.Active[]
    >([]);

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
        fetch_active_wallpapers();
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
                fetch_active_wallpapers();
            })
            .catch((e) => toast.error(e));
    }

    return (
        <main class='container'>
            <Toaster position='bottom-right' />
            <Header
                selectedMode={selectedMode}
                selectedScreen={selectedScreen}
                selectedWallpaper={selectedWallpaper}
                setDebouncedSearchQuery={setDebouncedSearchQuery}
                setSearchQuery={setSearchQuery}
                setShowSettings={setShowSettings}
                setSelectedMode={setSelectedMode}
                setSelectedScreen={setSelectedScreen}
                searchQuery={searchQuery}
                filteredItems={filteredItems}
                wallpapers={wallpapers}
                setWallpapers={setWallpapers}
                fetch_active_wallpapers={fetch_active_wallpapers}
            />
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
                <Settings
                    setWallpapers={setWallpapers}
                    wallpapers={wallpapers}
                    setShowSettings={setShowSettings}
                />
            </Show>
        </main>
    );
}

export default App;
