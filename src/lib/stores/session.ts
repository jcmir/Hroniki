import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type SessionState = 'initializing' | 'pinNotConfigured' | 'unlocked' | 'locked' | 'error';

export interface SessionStoreData {
    state: SessionState;
    isLocked: boolean;
    isPinConfigured: boolean;
    error: string | null;
}

function createSessionStore() {
    const { subscribe, set, update } = writable<SessionStoreData>({
        state: 'initializing',
        isLocked: false,
        isPinConfigured: false,
        error: null,
    });

    let initPromise: Promise<void> | null = null;
    let listenersSet = false;

    return {
        subscribe,

        async init() {
            if (initPromise) return initPromise;

            initPromise = (async () => {
                try {
                    const configured = await invoke<boolean>('is_pin_configured');

                    update(s => ({
                        ...s,
                        isPinConfigured: configured,
                        isLocked: configured,
                        state: configured ? 'locked' : 'pinNotConfigured',
                    }));

                    if (!listenersSet) {
                        await listen('session:locked', () => {
                            update(s => ({ ...s, state: 'locked', isLocked: true }));
                        });

                        await listen('session:auth_required', () => {
                            update(s => ({ ...s, state: 'locked', isLocked: true }));
                        });

                        await listen('session:unlocked', () => {
                            update(s => ({ ...s, state: 'unlocked', isLocked: false, error: null }));
                        });
                        listenersSet = true;
                    }
                } catch (err) {
                    console.error('[sessionStore] Init error:', err);
                    update(s => ({
                        ...s,
                        state: 'error',
                        error: typeof err === 'string' ? err : 'Ошибка инициализации сессии'
                    }));
                    initPromise = null;
                }
            })();

            return initPromise;
        },

        async setupInitialPin(pin: string): Promise<boolean> {
            try {
                await invoke('set_pin', { pin });
                update(s => ({
                    ...s,
                    isPinConfigured: true,
                    isLocked: false,
                    state: 'unlocked',
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
                    update(s => ({ ...s, state: 'unlocked', isLocked: false, error: null }));
                    return true;
                } else {
                    update(s => ({ ...s, error: 'Неверный PIN-код' }));
                    return false;
                }
            } catch (err) {
                update(s => ({ ...s, error: typeof err === 'string' ? err : 'Ошибка проверки PIN-кода' }));
                return false;
            }
        },

        async lock() {
            try {
                await invoke('lock_application');
            } catch {
                // Simulation fallback
            }
            update(s => ({ ...s, state: 'locked', isLocked: true }));
        },

        async retryInit() {
            initPromise = null;
            return this.init();
        },

        clearError() {
            update(s => ({ ...s, error: null }));
        }
    };
}

export const sessionStore = createSessionStore();
