import { writable } from 'svelte/store';

export type ViewMode = 'normal' | 'focus' | 'fullscreen' | 'mini';

export const viewMode = writable<ViewMode>('normal');
