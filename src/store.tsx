import {
    JSXElement,
    createSignal,
    Accessor,
    Setter,
    createMemo,
    createContext,
    useContext,
} from 'solid-js';
import * as ipc from '@ipc';

interface ContextProps {
    selectedScreen: Accessor<string>;
    setSelectedScreen: Setter<string>;
    showSettings: Accessor<boolean>;
    setShowSettings: Setter<boolean>;
    searchQuery: Accessor<string>;
    setSearchQuery: Setter<string>;
    selectedMode: Accessor<ipc.types.Mode>;
    setSelectedMode: Setter<ipc.types.Mode>;
    debouncedSearchQuery: Accessor<string>;
    setDebouncedSearchQuery: Setter<string>;
    wallpapers: Accessor<ipc.types.Wallpaper[]>;
    setWallpapers: Setter<ipc.types.Wallpaper[]>;
    selectedWallpaper: Accessor<string>;
    setSelectedWallpaper: Setter<string>;
    activeWallpapers: Accessor<ipc.types.Active[]>;
    setActiveWallpapers: Setter<ipc.types.Active[]>;
    filteredItems: Accessor<ipc.types.Wallpaper[]>;
}

const GlobalContext = createContext<ContextProps>();

export function GlobalContextProvider(props: { children: JSXElement }) {
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

    return (
        <GlobalContext.Provider
            value={{
                selectedScreen,
                setSelectedScreen,
                showSettings,
                setShowSettings,
                searchQuery,
                setSearchQuery,
                selectedMode,
                setSelectedMode,
                debouncedSearchQuery,
                setDebouncedSearchQuery,
                wallpapers,
                setWallpapers,
                selectedWallpaper,
                setSelectedWallpaper,
                activeWallpapers,
                setActiveWallpapers,
                filteredItems,
            }}
        >
            {props.children}
        </GlobalContext.Provider>
    );
}

export const useGlobalContext = () => useContext(GlobalContext)!;
