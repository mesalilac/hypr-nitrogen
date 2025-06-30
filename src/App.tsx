import { createSignal, Show, onMount, createMemo } from 'solid-js';
import toast, { Toaster } from 'solid-toast';
import Settings from './components/Settings';
import Header from './components/Header';
import ThumbnailsList from './components/ThumbnailsList';
import * as ipc from './ipc';
import './App.css';

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
            <ThumbnailsList
                activeWallpapers={activeWallpapers}
                filteredItems={filteredItems}
                selectedScreen={selectedScreen}
                selectedWallpaper={selectedWallpaper}
                selectedMode={selectedMode}
                searchQuery={searchQuery}
                setSelectedWallpaper={setSelectedWallpaper}
                setSelectedScreen={setSelectedScreen}
                fetch_active_wallpapers={fetch_active_wallpapers}
            />
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
