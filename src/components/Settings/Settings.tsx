import * as ipc from '@ipc';
import { Match, Switch, For, createSignal, onMount, Show } from 'solid-js';
import { open } from '@tauri-apps/plugin-dialog';
import toast from 'solid-toast';
import WallpaperSource from '@components/WallpaperSource';
import { useGlobalContext } from '@/store';

function Settings() {
    const { wallpapers, setWallpapers, showSettings, setShowSettings } =
        useGlobalContext();
    const [wallpaperSources, setWallpaperSources] = createSignal<
        ipc.types.WallpaperSource[]
    >([]);

    onMount(() => {
        ipc.get
            .wallpaper_sources()
            .then((res) => {
                setWallpaperSources(res.data);
            })
            .catch((e) => toast.error(e));
    });

    function update_source_list(id: string) {
        setWallpaperSources(wallpaperSources().filter((x) => x.id !== id));
        setWallpapers(wallpapers().filter((x) => x.wallpaper_source_id !== id));
    }

    async function addSource() {
        const directory = await open({
            directory: true,
        });

        if (directory) {
            toast
                .promise(ipc.add.wallpaper_source({ path: directory }), {
                    loading: 'Adding new source...',
                    success: 'Source added',
                    error: 'Failed to add source',
                })
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

    return (
        <Show when={showSettings()}>
            <div class='settings-container'>
                <div class='settings'>
                    <div class='settings-sources-list-header'>
                        <span>Sources list</span>
                        <button onClick={addSource}>+</button>
                    </div>
                    <div class='wallpaper-sources-list'>
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
                                        setWallpapers={setWallpapers}
                                    />
                                )}
                            </For>
                            <Switch>
                                <Match when={wallpaperSources().length === 0}>
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
    );
}

export default Settings;
