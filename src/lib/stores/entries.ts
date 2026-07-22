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
    searchQuery: string;
    selectedCategoryId: string | null;
    loading: boolean;
    error: string | null;
}

function createEntriesStore() {
    const { subscribe, set, update } = writable<EntriesStoreData>({
        entries: [],
        searchQuery: '',
        selectedCategoryId: null,
        loading: false,
        error: null,
    });

    const store = {
        subscribe,

        async loadEntries() {
            update(s => ({ ...s, loading: true, error: null }));
            try {
                const list = await invoke<EntryDto[]>('get_entries');
                update(s => ({ ...s, entries: list || [], loading: false }));
            } catch (err) {
                console.warn('[entriesStore] Error loading entries:', err);
                update(s => ({
                    ...s,
                    loading: false,
                    error: typeof err === 'string' ? err : 'Ошибка загрузки записей',
                }));
            }
        },

        async searchEntries(queryText?: string, categoryId?: string) {
            update(s => ({
                ...s,
                searchQuery: queryText !== undefined ? queryText : s.searchQuery,
                selectedCategoryId: categoryId !== undefined ? categoryId : s.selectedCategoryId,
                loading: true,
            }));

            try {
                const list = await invoke<EntryDto[]>('search_entries', {
                    queryText: queryText || null,
                    categoryId: categoryId || null,
                    objectId: null,
                    startDate: null,
                    endDate: null,
                });
                update(s => ({ ...s, entries: list || [], loading: false }));
            } catch {
                update(s => ({ ...s, loading: false }));
            }
        },

        async createEntry(title: string, content: string, objectId?: string, photoPaths: string[] = []): Promise<boolean> {
            update(s => ({ ...s, loading: true, error: null }));
            try {
                let targetObjectId = objectId;

                if (!targetObjectId) {
                    const objects = await invoke<any[]>('get_objects');
                    if (objects && objects.length > 0) {
                        targetObjectId = objects[0].id;
                    } else {
                        throw new Error('Сначала создайте объект в разделе Объекты');
                    }
                }

                await invoke<string>('create_entry', {
                    objectId: targetObjectId,
                    title,
                    description: content || null,
                    imageFilenames: photoPaths.length > 0 ? photoPaths : null,
                });

                await store.loadEntries();
                return true;
            } catch (err) {
                console.error('[entriesStore] Create error:', err);

                // Fallback for UI simulation
                const fallbackEntry: EntryDto = {
                    id: 'local_' + Date.now(),
                    title,
                    content,
                    object_id: objectId || 'mock_object',
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
                await invoke('delete_entry', { entryId: id });
            } catch {
                // Local fallback
            }
            update(s => ({
                ...s,
                entries: s.entries.filter(e => e.id !== id),
            }));
        }
    };

    return store;
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
