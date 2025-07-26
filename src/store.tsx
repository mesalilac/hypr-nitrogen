import * as ipc from '@ipc';
import { createSignalObject, SignalObject } from '@utils';
import {
    Accessor,
    createContext,
    createMemo,
    JSXElement,
    useContext,
} from 'solid-js';

interface ContextProps {
    selectedScreen: SignalObject<string>;
    showSettings: SignalObject<boolean>;
    searchQuery: SignalObject<string>;
    selectedMode: SignalObject<ipc.types.Mode>;
    debouncedSearchQuery: SignalObject<string>;
    wallpapers: SignalObject<ipc.types.Wallpaper[]>;
    selectedWallpaper: SignalObject<string>;
    activeWallpapers: SignalObject<ipc.types.Active[]>;
    filteredItems: Accessor<ipc.types.Wallpaper[]>;
}

const GlobalContext = createContext<ContextProps>();

export function GlobalContextProvider(props: { children: JSXElement }) {
    const selectedScreen = createSignalObject<string>('all');
    const showSettings = createSignalObject(false);
    const searchQuery = createSignalObject('');
    const selectedMode = createSignalObject<ipc.types.Mode>('default');
    const debouncedSearchQuery = createSignalObject('');
    const wallpapers = createSignalObject<ipc.types.Wallpaper[]>([]);
    const selectedWallpaper = createSignalObject<string>(''); // wallpaper_id
    const activeWallpapers = createSignalObject<ipc.types.Active[]>([]);

    const filteredItems = createMemo(() => {
        const query = debouncedSearchQuery.get().toLowerCase();
        const sortedList = wallpapers
            .get()
            .sort((a: ipc.types.Wallpaper, b: ipc.types.Wallpaper) => {
                if (a.is_favorite && !b.is_favorite) return -1;
                if (!a.is_favorite && b.is_favorite) return 1;
                return 0;
            });

        if (!query) {
            return sortedList;
        }

        return sortedList.filter((item) =>
            item.keywords.toLowerCase().includes(query),
        );
    });

    return (
        <GlobalContext.Provider
            value={{
                selectedScreen,
                showSettings,
                searchQuery,
                selectedMode,
                debouncedSearchQuery,
                wallpapers,
                selectedWallpaper,
                activeWallpapers,
                filteredItems,
            }}
        >
            {props.children}
        </GlobalContext.Provider>
    );
}

export const useGlobalContext = () => useContext(GlobalContext)!;
