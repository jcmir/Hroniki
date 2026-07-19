import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type SessionState = 'active' | 'locked' | 'awaitingUnlock';

export interface SessionStoreData {
    state: SessionState;
    isLocked: boolean;
    error: string | null;
}

function createSessionStore() {
    const { subscribe, set, update } = writable<SessionStoreData>({
        state: 'active',
        isLocked: false,
        error: null,
    });

    return {
        subscribe,

        async init() {
            try {
                // Listen for IPC events emitted from Rust backend
                await listen('session:locked', () => {
                    update(s => ({ ...s, state: 'locked', isLocked: true }));
                });

                await listen('session:auth_required', () => {
                    update(s => ({ ...s, state: 'awaitingUnlock', isLocked: true }));
                });

                await listen('session:unlocked', () => {
                    update(s => ({ ...s, state: 'active', isLocked: false, error: null }));
                });
            } catch (err) {
                console.warn('[sessionStore] Tauri event listener warning:', err);
            }
        },

        async unlockWithPin(pin: string): Promise<boolean> {
            try {
                const success = await invoke<boolean>('verify_pin', { pin });
                if (success) {
                    update(s => ({ ...s, state: 'active', isLocked: false, error: null }));
                    return true;
                } else {
                    update(s => ({ ...s, error: 'Неверный PIN-код' }));
                    return false;
                }
            } catch (err) {
                const errorMsg = typeof err === 'string' ? err : 'Ошибка проверки PIN-кода';
                update(s => ({ ...s, error: errorMsg }));
                return false;
            }
        },

        async lock() {
            try {
                await invoke('lock_application');
            } catch {
                // Fallback for UI simulation if command not available
            }
            update(s => ({ ...s, state: 'locked', isLocked: true }));
        },

        clearError() {
            update(s => ({ ...s, error: null }));
        }
    };
}

export const sessionStore = createSessionStore();
