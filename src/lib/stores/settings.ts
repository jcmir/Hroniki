import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface UserSettingsDto {
    profile_name: string | null;
    theme: 'System' | 'Dark' | 'Light';
    auto_lock_minutes: number;
    notifications: boolean;
    biometric_unlock: boolean;
}

export interface SettingsStoreData {
    settings: UserSettingsDto;
    appVersion: string;
    loading: boolean;
    error: string | null;
}

function createSettingsStore() {
    const { subscribe, set, update } = writable<SettingsStoreData>({
        settings: {
            profile_name: 'Пользователь',
            theme: 'System',
            auto_lock_minutes: 5,
            notifications: true,
            biometric_unlock: false,
        },
        appVersion: 'Beta 0.2.0',
        loading: false,
        error: null,
    });

    return {
        subscribe,

        updateProfileName(name: string) {
            update(s => ({
                ...s,
                settings: { ...s.settings, profile_name: name },
            }));
        },

        async exportBackup(password: string): Promise<string> {
            try {
                const res = await invoke<string>('export_archive', { password });
                return res;
            } catch (err) {
                throw typeof err === 'string' ? err : 'Ошибка экспорта резервной копии';
            }
        },

        async importBackup(password: string): Promise<void> {
            try {
                await invoke('import_archive', { password });
            } catch (err) {
                throw typeof err === 'string' ? err : 'Ошибка импорта резервной копии';
            }
        }
    };
}

export const settingsStore = createSettingsStore();
