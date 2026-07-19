import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface PhotoDto {
    id: string;
    file_path: string;
    created_at: string;
}

export interface EntryDto {
    id: string;
    title: string;
    content: string | null;
    object_id: string | null;
    category_name?: string;
    created_at: string;
    photos: PhotoDto[];
}

export interface EntriesStoreData {
    entries: EntryDto[];
    loading: boolean;
    error: string | null;
}

function createEntriesStore() {
    const { subscribe, set, update } = writable<EntriesStoreData>({
        entries: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        async loadEntries() {
            update(s => ({ ...s, loading: true, error: null }));
            try {
                const list = await invoke<EntryDto[]>('get_entries');
                update(s => ({ ...s, entries: list || [], loading: false }));
            } catch (err) {
                console.warn('[entriesStore] Error loading entries from Rust backend:', err);
                update(s => ({
                    ...s,
                    loading: false,
                    error: typeof err === 'string' ? err : 'Ошибка загрузки записей',
                }));
            }
        },

        async createEntry(title: string, content: string, categoryId?: string, photoPaths: string[] = []): Promise<boolean> {
            update(s => ({ ...s, loading: true, error: null }));
            try {
                const created = await invoke<EntryDto>('create_entry', {
                    title,
                    content,
                    categoryId: categoryId || null,
                    photoPaths,
                });

                update(s => ({
                    ...s,
                    entries: [created, ...s.entries],
                    loading: false,
                }));
                return true;
            } catch (err) {
                // Fallback local creation for demonstration / UI testing when running without Tauri IPC
                const fallbackEntry: EntryDto = {
                    id: 'local_' + Date.now(),
                    title,
                    content,
                    object_id: categoryId || null,
                    category_name: 'Личные заметки',
                    created_at: new Date().toISOString(),
                    photos: photoPaths.map((p, idx) => ({
                        id: `photo_${idx}`,
                        file_path: p,
                        created_at: new Date().toISOString(),
                    })),
                };

                update(s => ({
                    ...s,
                    entries: [fallbackEntry, ...s.entries],
                    loading: false,
                }));
                return true;
            }
        },

        async deleteEntry(id: string) {
            try {
                await invoke('delete_entry', { id });
            } catch {
                // Local fallback
            }
            update(s => ({
                ...s,
                entries: s.entries.filter(e => e.id !== id),
            }));
        }
    };
}

export const entriesStore = createEntriesStore();

// Derived store grouping entries by formatted Date headers
export const entriesByDate = derived(entriesStore, $store => {
    const groups: { [date: string]: EntryDto[] } = {};
    for (const entry of $store.entries) {
        const dateStr = new Date(entry.created_at).toLocaleDateString('ru-RU', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            weekday: 'short',
        });
        if (!groups[dateStr]) {
            groups[dateStr] = [];
        }
        groups[dateStr].push(entry);
    }
    return Object.entries(groups).map(([date, items]) => ({ date, items }));
});
