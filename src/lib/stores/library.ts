import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Album } from '../types';

export const folderPath = writable<string | null>(null);
export const albums = writable<Album[]>([]);
export const isScanning = writable(false);
export const librarySize = writable('0 MB');
export const selectedAlbum = writable<Album | null>(null);

export const albumCount = derived(albums, ($albums) => $albums.length);

export async function scanFolder(path: string) {
  isScanning.set(true);
  folderPath.set(path);

  try {
    const [scanned, size] = await Promise.all([
      invoke<Album[]>('scan_music_folder', { path }),
      invoke<string>('get_library_size', { path }),
    ]);
    albums.set(scanned);
    librarySize.set(size);
  } catch (e) {
    console.error('Scan failed:', e);
    albums.set([]);
  } finally {
    isScanning.set(false);
  }
}
