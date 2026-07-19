import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type SessionState = 'active' | 'locked' | 'awaitingUnlock';

export interface SessionStoreData {
    state: SessionState;
    isLocked: boolean;
    isPinConfigured: boolean;
    error: string | null;
}

function createSessionStore() {
    const { subscribe, set, update } = writable<SessionStoreData>({
        state: 'active',
        isLocked: false,
        isPinConfigured: true,
        error: null,
    });

    return {
        subscribe,

        async init() {
            try {
                const configured = await invoke<boolean>('is_pin_configured');
                update(s => ({
                    ...s,
                    isPinConfigured: configured,
                    isLocked: configured,
                    state: configured ? 'locked' : 'active',
                }));

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
                console.warn('[sessionStore] Initialization warning:', err);
            }
        },

        async setupInitialPin(pin: string): Promise<boolean> {
            try {
                await invoke('set_pin', { pin });
                update(s => ({
                    ...s,
                    isPinConfigured: true,
                    isLocked: false,
                    state: 'active',
                    error: null,
                }));
                return true;
            } catch (err) {
                update(s => ({
                    ...s,
                    error: typeof err === 'string' ? err : 'Ошибка установки PIN-кода',
                }));
                return false;
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
