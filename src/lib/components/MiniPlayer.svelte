<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import {
    currentTrack,
    currentAlbum,
    isPlaying,
    isShuffled,
    repeatMode,
    toggleRepeat,
    playShuffledAll,
    pause,
    resume,
    playNext,
    playPrev,
    position,
    duration,
  } from '$lib/stores/player';
  import { albums } from '$lib/stores/library';
  import { playUiSfx } from '$lib/ui-sfx';

  let pinned = $state(false);

  const coverSrc = $derived(
    $currentAlbum?.cover_art ? convertFileSrc($currentAlbum.cover_art) : null
  );

  const progressPct = $derived(
    $duration > 0 ? Math.min(100, ($position / $duration) * 100) : 0
  );

  let bgColor = $state('rgb(14, 16, 28)');

  $effect(() => {
    if (coverSrc) {
      extractBg(coverSrc).then((c) => (bgColor = c));
    } else {
      bgColor = 'rgb(14, 16, 28)';
    }
  });

  async function extractBg(src: string): Promise<string> {
    try {
      const bitmap = await createImageBitmap(
        await fetch(src).then((r) => r.blob()),
        { resizeWidth: 8, resizeHeight: 8, resizeQuality: 'low' }
      );
      const canvas = document.createElement('canvas');
      canvas.width = 8; canvas.height = 8;
      const ctx = canvas.getContext('2d')!;
      ctx.drawImage(bitmap, 0, 0);
      bitmap.close();
      const data = ctx.getImageData(0, 0, 8, 8).data;
      let r = 0, g = 0, b = 0;
      const px = data.length / 4;
      for (let i = 0; i < data.length; i += 4) {
        r += data[i]; g += data[i + 1]; b += data[i + 2];
      }
      r = Math.round(r / px * 0.35 + 8);
      g = Math.round(g / px * 0.35 + 8);
      b = Math.round(b / px * 0.35 + 8);
      return `rgb(${r}, ${g}, ${b})`;
    } catch {
      return 'rgb(14, 16, 28)';
    }
  }

  async function handlePlayPause() {
    if ($isPlaying) await pause(); else await resume();
  }

  async function handlePrev() {
    if (!$currentAlbum) return;
    playUiSfx('nextPrev');
    await playPrev($currentAlbum);
  }

  async function handleNext() {
    if (!$currentAlbum) return;
    playUiSfx('nextPrev');
    await playNext($currentAlbum);
  }

  async function togglePin() {
    pinned = !pinned;
    await getCurrentWindow().setAlwaysOnTop(pinned);
    playUiSfx('confirm');
  }

  async function handleToggleRepeat() {
    playUiSfx('confirm');
    toggleRepeat();
  }

  async function handleToggleShuffle() {
    playUiSfx('confirm');
    await playShuffledAll($albums);
  }
</script>

<div class="mini-root" style="background: {bgColor}">
  <!-- Top row: art + info -->
  <div class="mini-top" data-tauri-drag-region>
    <div class="mini-art">
      {#if coverSrc}
        <img src={coverSrc} alt="" />
      {:else}
        <span class="mini-art-ph">♪</span>
      {/if}
    </div>

    <div class="mini-info" data-tauri-drag-region>
      <span class="mini-track">{$currentTrack?.title ?? '—'}</span>
      <span class="mini-artist">{$currentTrack?.artist ?? ''}</span>
    </div>
  </div>

  <!-- Bottom row: controls -->
  <div class="mini-bottom">
    <div class="mini-controls">
      <button
        class="mini-btn"
        class:active={$isShuffled}
        onclick={handleToggleShuffle}
        disabled={!$currentTrack}
        title="Shuffle"
      >
        <svg viewBox="0 0 10 10" fill="currentColor">
          <path d="M1,3 L3,3 L5,6 L7,6 L9,6 M1,7 L3,7 L5,4 L7,4 L9,4" stroke="currentColor" fill="none" stroke-width="1.2"/>
        </svg>
      </button>

      <button class="mini-btn" onclick={handlePrev} disabled={!$currentTrack} title="Previous">
        <svg viewBox="0 0 10 10" fill="currentColor">
          <polygon points="9,1 4,5 9,9"/>
          <rect x="1" y="1" width="2" height="8"/>
        </svg>
      </button>

      <button class="mini-btn mini-btn--play" onclick={handlePlayPause} disabled={!$currentTrack}>
        {#if $isPlaying}
          <svg viewBox="0 0 10 10" fill="currentColor">
            <rect x="1.5" y="1" width="3" height="8"/>
            <rect x="5.5" y="1" width="3" height="8"/>
          </svg>
        {:else}
          <svg viewBox="0 0 10 10" fill="currentColor">
            <polygon points="2,1 9,5 2,9"/>
          </svg>
        {/if}
      </button>

      <button class="mini-btn" onclick={handleNext} disabled={!$currentTrack} title="Next">
        <svg viewBox="0 0 10 10" fill="currentColor">
          <polygon points="1,1 6,5 1,9"/>
          <rect x="7" y="1" width="2" height="8"/>
        </svg>
      </button>

      <button
        class="mini-btn"
        class:active={$repeatMode !== 'none'}
        onclick={handleToggleRepeat}
        disabled={!$currentTrack}
        title="Repeat"
      >
        <svg viewBox="0 0 10 10" fill="currentColor">
          <path d="M2,4 L8,4 L8,7 L2,7 Z M8,4 L9,3 L9,5 Z" fill="none" stroke="currentColor" stroke-width="1"/>
          {#if $repeatMode === 'one'}
            <text x="5" y="6.5" font-size="4" text-anchor="middle" fill="currentColor" stroke="none">1</text>
          {/if}
        </svg>
      </button>
    </div>

    <!-- Pin toggle -->
    <button class="mini-pin-btn" class:pinned onclick={togglePin} title={pinned ? 'Unpin' : 'Pin on top'}>
      <svg viewBox="0 0 10 10" fill="currentColor">
        <path d="M5,0.5 L7.5,3 L6.5,4 L8.5,7 L6,7 L6,9.5 L4,9.5 L4,7 L1.5,7 L3.5,4 L2.5,3 Z"/>
      </svg>
    </button>
  </div>

  <!-- Progress bar -->
  <div class="mini-progress-track">
    <div class="mini-progress-fill" style="width: {progressPct}%"></div>
  </div>
</div>

<style>
  .mini-root {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    transition: background 0.4s ease;
  }

  /* Top row: art + track info */
  .mini-top {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px 4px;
    min-height: 0;
  }

  /* Bottom row: controls */
  .mini-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 6px 4px;
    gap: 4px;
  }

  .mini-art {
    width: 52px;
    height: 52px;
    flex-shrink: 0;
    border-radius: 3px;
    overflow: hidden;
    background: rgba(90, 95, 120, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mini-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .mini-art-ph {
    font-size: 20px;
    color: rgba(90, 95, 120, 0.5);
  }

  .mini-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .mini-track {
    font-size: 11px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .mini-artist {
    font-size: 9px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-weight: 400;
    text-shadow: none;
  }

  .mini-controls {
    display: flex;
    align-items: center;
    gap: 1px;
    flex-shrink: 0;
  }

  .mini-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 5px;
    color: rgba(255, 255, 255, 0.5);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.12s, background 0.12s;
  }

  .mini-btn svg {
    width: 12px;
    height: 12px;
    display: block;
  }

  .mini-btn:hover:not(:disabled) {
    color: rgba(255, 255, 255, 0.95);
    background: rgba(255, 255, 255, 0.08);
  }

  .mini-btn:disabled {
    opacity: 0.15;
    cursor: default;
  }

  .mini-btn.active {
    color: var(--track-active);
    filter: drop-shadow(0 0 2px var(--track-active));
  }

  .mini-btn--play {
    color: var(--track-active);
  }

  .mini-btn--play svg {
    width: 14px;
    height: 14px;
  }

  .mini-pin-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 5px;
    color: rgba(255, 255, 255, 0.28);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.12s;
    flex-shrink: 0;
  }

  .mini-pin-btn svg {
    width: 11px;
    height: 11px;
    display: block;
  }

  .mini-pin-btn:hover {
    color: rgba(255, 255, 255, 0.7);
  }

  .mini-pin-btn.pinned {
    color: var(--track-active);
  }

  .mini-progress-track {
    width: 100%;
    height: 3px;
    background: rgba(255, 255, 255, 0.08);
    flex-shrink: 0;
  }

  .mini-progress-fill {
    height: 100%;
    background: var(--track-active);
    transition: width 0.5s linear;
  }
</style>
