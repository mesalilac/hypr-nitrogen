import * as ipc from '@ipc';
import { Match, Switch, For, onMount, Show } from 'solid-js';
import { open } from '@tauri-apps/plugin-dialog';
import toast from 'solid-toast';
import WallpaperSource from '@components/WallpaperSource';
import { useGlobalContext } from '@/store';
import { createSignalObject } from '@/utils';

function Settings() {
    const { wallpapers, showSettings } = useGlobalContext();
    const wallpaperSources = createSignalObject<ipc.types.WallpaperSource[]>(
        [],
    );

    onMount(() => {
        ipc.get
            .wallpaper_sources()
            .then((res) => {
                wallpaperSources.set(res.data);
            })
            .catch((e) => toast.error(e));
    });

    function update_source_list(id: string) {
        wallpaperSources.set(wallpaperSources.get().filter((x) => x.id !== id));
        wallpapers.set(
            wallpapers.get().filter((x) => x.wallpaper_source_id !== id),
        );
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
                    wallpaperSources.set([
                        ...wallpaperSources.get(),
                        res.data!,
                    ]);
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
                            wallpapers.set([...wallpapers.get(), ...res2.data]);
                        })
                        .catch((e) => toast.error(e));
                })
                .catch((e) => {
                    toast.error(e);
                });
        }
    }

    return (
        <Show when={showSettings.get()}>
            <div class='settings-container'>
                <div class='settings'>
                    <div class='settings-sources-list-header'>
                        <span>Sources list</span>
                        <button onClick={addSource}>+</button>
                    </div>
                    <div class='wallpaper-sources-list'>
                        <div>
                            <For each={wallpaperSources.get()}>
                                {(x) => (
                                    <WallpaperSource
                                        id={x.id}
                                        path={x.path}
                                        active={x.active}
                                        update_source_list_fn={
                                            update_source_list
                                        }
                                        setWallpapers={wallpapers.set}
                                    />
                                )}
                            </For>
                            <Switch>
                                <Match
                                    when={wallpaperSources.get().length === 0}
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
                        onClick={() => showSettings.set(false)}
                    >
                        Close
                    </button>
                </div>
            </div>
        </Show>
    );
}

export default Settings;
