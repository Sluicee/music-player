import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface UpdateInfo {
  version: string;
  url: string;
  is_available: boolean;
}

export const updateInfo = writable<UpdateInfo | null>(null);

export async function checkForUpdates() {
  try {
    const info = await invoke<UpdateInfo>('check_for_updates');
    if (info.is_available) {
      updateInfo.set(info);
      console.log('Update available:', info.version);
    } else {
      updateInfo.set(null);
    }
  } catch (e) {
    console.error('Failed to check for updates:', e);
    updateInfo.set(null);
  }
}
