import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface ObjectDto {
  id: string;
  category_id: string;
  name: string;
  description: string | null;
  created_at: string;
}

export interface ObjectsStoreData {
  objects: ObjectDto[];
  loading: boolean;
  saving: boolean;
  error: string | null;
}

function createObjectsStore() {
  const { subscribe, update } = writable<ObjectsStoreData>({
    objects: [],
    loading: false,
    saving: false,
    error: null,
  });

  return {
    subscribe,

    async loadObjects(): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const objects = await invoke<ObjectDto[]>('get_objects');
        update((state) => ({ ...state, objects: objects ?? [], loading: false }));
        return true;
      } catch (error) {
        update((state) => ({
          ...state,
          loading: false,
          error: typeof error === 'string' ? error : 'Ошибка загрузки объектов',
        }));
        return false;
      }
    },

    async createObject(categoryId: string, name: string, description?: string): Promise<string | null> {
      update((state) => ({ ...state, saving: true, error: null }));
      try {
        const objectId = await invoke<string>('create_object', {
          categoryId,
          name: name.trim(),
          description: description?.trim() || null,
        });

        const objects = await invoke<ObjectDto[]>('get_objects');
        update((state) => ({
          ...state,
          objects: objects ?? [],
          saving: false,
          error: null,
        }));
        return objectId;
      } catch (error) {
        update((state) => ({
          ...state,
          saving: false,
          error: typeof error === 'string' ? error : 'Ошибка создания объекта',
        }));
        return null;
      }
    },

    clearError() {
      update((state) => ({ ...state, error: null }));
    },
  };
}

export const objectsStore = createObjectsStore();
