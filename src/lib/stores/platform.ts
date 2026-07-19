import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface PlatformCapabilitiesDto {
    notifications: boolean;
    exact_alarms: boolean;
    saf_backup: boolean;
    biometric: boolean;
    strongbox: boolean;
    secure_hardware: boolean;
}

export interface PlatformStoreData {
    capabilities: PlatformCapabilitiesDto;
    loading: boolean;
}

function createPlatformStore() {
    const defaultCaps: PlatformCapabilitiesDto = {
        notifications: true,
        exact_alarms: true,
        saf_backup: true,
        biometric: false,
        strongbox: false,
        secure_hardware: false,
    };

    const { subscribe, set, update } = writable<PlatformStoreData>({
        capabilities: defaultCaps,
        loading: false,
    });

    return {
        subscribe,

        async loadCapabilities() {
            update(s => ({ ...s, loading: true }));
            try {
                const caps = await invoke<PlatformCapabilitiesDto>('get_platform_capabilities');
                update(s => ({ ...s, capabilities: caps, loading: false }));
            } catch {
                update(s => ({ ...s, loading: false }));
            }
        }
    };
}

export const platformStore = createPlatformStore();
