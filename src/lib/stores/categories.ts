import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface CategoryDto {
    id: string;
    name: string;
    icon: string;
    color: string;
    system_type?: string | null;
}

export interface CategoriesStoreData {
    categories: CategoryDto[];
    selectedCategory: string | null;
    loading: boolean;
}

function createCategoriesStore() {
    const defaultCategories: CategoryDto[] = [
        { id: 'all', name: 'Все', icon: '✨', color: '#F59E0B' },
        { id: 'vehicle', name: 'Машина', icon: '🚗', color: '#3B82F6', system_type: 'vehicle' },
        { id: 'home', name: 'Дом', icon: '🏡', color: '#10B981', system_type: 'home' },
        { id: 'travel', name: 'Путешествия', icon: '✈️', color: '#EC4899', system_type: 'travel' },
        { id: 'people', name: 'Люди', icon: '👥', color: '#8B5CF6', system_type: 'people' },
    ];

    const { subscribe, set, update } = writable<CategoriesStoreData>({
        categories: defaultCategories,
        selectedCategory: null,
        loading: false,
    });

    return {
        subscribe,

        async loadCategories() {
            update(s => ({ ...s, loading: true }));
            try {
                const list = await invoke<CategoryDto[]>('get_categories');
                if (list && list.length > 0) {
                    update(s => ({ ...s, categories: list, loading: false }));
                } else {
                    update(s => ({ ...s, loading: false }));
                }
            } catch {
                update(s => ({ ...s, loading: false }));
            }
        },

        async createCategory(name: string, icon: string = '✨', color: string = '#F59E0B'): Promise<boolean> {
            try {
                const created = await invoke<CategoryDto>('create_category', { name, icon, color });
                update(s => ({
                    ...s,
                    categories: [...s.categories, created],
                }));
                return true;
            } catch {
                // Fallback for UI simulation
                const localCat: CategoryDto = {
                    id: 'custom_' + Date.now(),
                    name,
                    icon,
                    color,
                };
                update(s => ({
                    ...s,
                    categories: [...s.categories, localCat],
                }));
                return true;
            }
        },

        selectCategory(id: string | null) {
            update(s => ({ ...s, selectedCategory: id === 'all' ? null : id }));
        }
    };
}

export const categoriesStore = createCategoriesStore();
