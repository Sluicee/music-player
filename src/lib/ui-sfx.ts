import { browser } from '$app/environment';
import { writable, get } from 'svelte/store';

const SOURCES = {
  confirm: '/sfx/confirm.wav',
  back: '/sfx/back.wav',
  open: '/sfx/open.wav',
  nextPrev: '/sfx/next_prev.wav',
  steps: '/sfx/steps.wav',
  scan: '/sfx/scan.wav',
} as const;

const DEFAULT_VOLUMES: Record<UiSfxName, number> = {
  confirm: 0.225,
  back: 0.1,
  open: 0.11,
  nextPrev: 0.10,
  steps: 0.14,
  scan: 0.01,
};

export type UiSfxName = keyof typeof SOURCES;

// Persistence logic
const STORAGE_KEY = 'mc_sfx_enabled';

function getInitialState() {
  if (browser) {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored !== null) {
      return stored === 'true';
    }
  }
  return true;
}

export const sfxEnabled = writable(getInitialState());

if (browser) {
  sfxEnabled.subscribe(value => {
    localStorage.setItem(STORAGE_KEY, value.toString());
  });
}

const templates = new Map<UiSfxName, HTMLAudioElement>();

function getTemplate(name: UiSfxName): HTMLAudioElement | null {
  if (typeof Audio === 'undefined') return null;

  let audio = templates.get(name);
  if (!audio) {
    audio = new Audio(SOURCES[name]);
    audio.preload = 'auto';
    templates.set(name, audio);
  }

  return audio;
}

export function primeUiSfx() {
  for (const name of Object.keys(SOURCES) as UiSfxName[]) {
    getTemplate(name)?.load();
  }
}

export function playUiSfx(name: UiSfxName, volume = DEFAULT_VOLUMES[name]) {
  if (!get(sfxEnabled)) return;

  const template = getTemplate(name);
  if (!template) return;

  const instance = template.cloneNode(true) as HTMLAudioElement;
  instance.volume = volume;
  instance.currentTime = 0;
  void instance.play().catch(() => {});
}
