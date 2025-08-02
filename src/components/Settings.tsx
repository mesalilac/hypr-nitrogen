import { WallpaperSource } from '@components';
import * as ipc from '@ipc';
import { open } from '@tauri-apps/plugin-dialog';
import { For, Match, onMount, Show, Switch } from 'solid-js';
import toast from 'solid-toast';
import { AddIcon } from '@/icons';
import { useGlobalContext } from '@/store';
import { createSignalObject } from '@/utils';

export function Settings() {
    const { wallpapers, showSettings } = useGlobalContext();
    const wallpaperSources = createSignalObject<ipc.types.WallpaperSource[]>(
        [],
    );

    onMount(async () => {
        const wallpaperSourcesRes = await ipc.get
            .wallpaper_sources()
            .catch(ipc.handleError);

        if (!wallpaperSourcesRes) return;

        wallpaperSources.set(wallpaperSourcesRes.data);
    });

    async function addSource() {
        const directory = await open({
            directory: true,
        });

        if (!directory) return;

        const addWallpaperSourceRes = await toast
            .promise(ipc.add.wallpaper_source({ path: directory }), {
                loading: 'Adding new source...',
                success: 'Source added',
                error: 'Failed to add source',
            })
            .catch(ipc.handleError);

        if (!addWallpaperSourceRes) return;

        wallpaperSources.set([
            ...wallpaperSources.get(),
            addWallpaperSourceRes.data!,
        ]);

        const utilScanSource = await toast
            .promise(
                ipc.util.scan_source({
                    sourceId: addWallpaperSourceRes.data.id,
                }),
                {
                    loading: 'Scanning...',
                    success: 'Scan complete',
                    error: 'Scan failed',
                },
            )
            .catch(ipc.handleError);

        if (!utilScanSource) return;

        wallpapers.set([...wallpapers.get(), ...utilScanSource.data]);
    }

    return (
        <Show when={showSettings.get()}>
            <div class='settings-container'>
                <div class='settings'>
                    <div class='settings-sources-list-header'>
                        <span>Sources list</span>
                        <button onClick={addSource} title='Add new source'>
                            <AddIcon />
                        </button>
                    </div>
                    <div class='wallpaper-sources-list'>
                        <div>
                            <For each={wallpaperSources.get()}>
                                {(x) => (
                                    <WallpaperSource
                                        id={x.id}
                                        path={x.path}
                                        active={x.active}
                                        wallpaperSources={wallpaperSources}
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
                        title='Close settings menu'
                    >
                        Close
                    </button>
                </div>
            </div>
        </Show>
    );
}
