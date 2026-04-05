<script lang="ts">
  import AlbumGrid from '$lib/components/AlbumGrid.svelte';
  import AlbumView from '$lib/components/AlbumView.svelte';
  import VolumeControl from '$lib/components/VolumeControl.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import PS2Btn from '$lib/components/PS2Btn.svelte';
  import OptionsMenu from '$lib/components/OptionsMenu.svelte';
  import {
    albums,
    isScanning,
    librarySize,
    selectedAlbum,
    scanStatus,
  } from '$lib/stores/library';
  import {
    currentTrack,
    currentAlbum,
    isPlaying,
    isPaused,
    pause,
    resume,
    playNext,
    playPrev,
  } from '$lib/stores/player';
  import type { Album } from '$lib/types';

  let hoveredAlbum = $state<Album | null>(null);
  let optionsOpen = $state(false);

  function selectAlbum(album: Album) {
    selectedAlbum.set(album);
  }

  function openCurrentAlbum() {
    if ($currentAlbum) selectedAlbum.set($currentAlbum);
  }
</script>

<div class="shell">

  <!-- Header -->
  <header class="header">
    <div class="header-left">
      <div class="memory-block">
        <span class="memory-label">Memory Card</span>
        {#if $librarySize !== '0 MB'}
          <span class="lib-size">{$librarySize}</span>
        {/if}
      </div>
    </div>

    <div class="header-right">
      {#if $isScanning}
        <span class="scanning">Scanning…</span>
      {/if}
      {#if hoveredAlbum}
        <span class="hovered-title">{hoveredAlbum.title}</span>
      {/if}
    </div>
  </header>

  <!-- Album grid -->
  <main class="content">
    {#if $isScanning && $albums.length === 0}
      <div class="state-msg">
        <div class="spinner"></div>
        <p class="scan-info">
          {#if $scanStatus.filesScanned > 0}
            {$scanStatus.filesScanned} files · {$scanStatus.albumsFound} albums
          {:else}
            Starting scan…
          {/if}
        </p>
      </div>
    {:else if $albums.length === 0}
      <div class="state-msg">
        <p class="hint">Click <strong>Memory Card</strong> to choose a music folder</p>
      </div>
    {:else}
      {#if $isScanning}
        <div class="scan-bar">
          <div class="spinner-sm"></div>
          <span>{$scanStatus.filesScanned} files · {$scanStatus.albumsFound} albums found</span>
        </div>
      {/if}
      <AlbumGrid albums={$albums} onselect={selectAlbum} onhover={(a) => (hoveredAlbum = a)} />
    {/if}
  </main>

  <!-- Footer -->
  <footer class="footer">
    <!-- Row 1: transport + progress, centered -->
    <div class="footer-top">
      <div class="transport">
        <button
          class="transport-btn"
          onclick={() => $currentAlbum && playPrev($currentAlbum)}
          disabled={!$currentTrack}
          title="Previous"
        >⏮</button>
        <button
          class="transport-btn play-btn"
          onclick={() => $isPlaying ? pause() : resume()}
          disabled={!$currentTrack}
          title={$isPlaying ? 'Pause' : 'Play'}
        >{$isPlaying ? '⏸' : '▶'}</button>
        <button
          class="transport-btn"
          onclick={() => $currentAlbum && playNext($currentAlbum)}
          disabled={!$currentTrack}
          title="Next"
        >⏭</button>
      </div>
      <ProgressBar />
      <VolumeControl />
    </div>

    <!-- Row 2: now-playing | volume | hints -->
    <div class="footer-bottom">
      <!-- Now playing -->
      <button
        class="now-playing"
        class:active={!!$currentTrack}
        onclick={openCurrentAlbum}
        disabled={!$currentTrack}
      >
        <div class="now-playing-art">
          {#if $currentAlbum?.cover_art}
            <img src={$currentAlbum.cover_art} alt="" />
          {:else}
            <span>♪</span>
          {/if}
        </div>
        <div class="now-playing-info">
          <span class="track-name">{$currentTrack?.title ?? 'No track playing'}</span>
          <span class="track-artist">{$currentTrack?.artist ?? '—'}</span>
        </div>
      </button>

    <!-- PS2 action hints -->
    <div class="actions">
      <div class="action-hint">
        <PS2Btn type="cross" />
        <span class="btn-label">Select</span>
      </div>
      <div class="action-hint">
        <PS2Btn type="circle" />
        <span class="btn-label">Back</span>
      </div>
      <div class="action-hint">
        <PS2Btn type="square" />
        <span class="btn-label">Shuffle</span>
      </div>
      <button class="action-hint action-btn" onclick={() => optionsOpen = true}>
        <PS2Btn type="triangle" />
        <span class="btn-label">Options</span>
      </button>
    </div>
    </div><!-- /footer-bottom -->
  </footer>

</div>

{#if $selectedAlbum}
  <AlbumView album={$selectedAlbum} onclose={() => selectedAlbum.set(null)} />
{/if}

{#if optionsOpen}
  <OptionsMenu onclose={() => optionsOpen = false} />
{/if}

<style>
  .shell {
    width: 100vw;
    height: 100vh;
    display: grid;
    grid-template-rows: auto 1fr auto;
    padding: 18px 24px 14px;
    filter: saturate(0.82) contrast(1.08) brightness(0.97);
  }

  /* ── Header ── */
  .header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding-bottom: 4px;
    min-height: 52px;
  }

  .header-left {
    display: flex;
    align-items: flex-start;
  }

  .memory-block {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .action-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    transition: opacity 0.15s;
  }
  .action-btn:hover { opacity: 0.75; }

  .memory-label {
    font-size: 22px;

    color: var(--text-primary);
    letter-spacing: 0.01em;
  }

  .lib-size {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .header-right {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .scanning {
    font-size: 12px;
    color: var(--text-dim);
    letter-spacing: 0.05em;
    padding-top: 4px;
  }

  .hovered-title {
    font-size: 28px;
    color: var(--track-active);
    letter-spacing: 0.01em;
    white-space: nowrap;
    line-height: 1;
    animation: fadein 0.15s ease;
  }

  @keyframes fadein {
    from { opacity: 0; transform: translateY(-3px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  /* ── Content ── */
  .content {
    overflow: hidden;
    padding: 10px 0;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .state-msg {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    color: var(--text-dim);
  }

  .hint { font-size: 14px; }
  .hint strong { color: var(--text-secondary); }

  .scan-info {
    font-size: 12px;
    color: var(--text-dim);
    letter-spacing: 0.04em;
  }

  .scan-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 2px 12px;
    font-size: 12px;
    color: var(--text-dim);
  }

  .spinner-sm {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(90, 95, 120, 0.2);
    border-top-color: var(--text-secondary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  .spinner {
    width: 28px;
    height: 28px;
    border: 2px solid rgba(90, 95, 120, 0.2);
    border-top-color: var(--text-secondary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  /* ── Footer ── */
  .footer {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-top: 8px;
  }

  .footer-top {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 4px 0 2px;
  }

  .footer-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .now-playing {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--card-bg);
    border: none;
    border-radius: 12px;
    padding: 10px 14px 10px 10px;
    cursor: pointer;
    box-shadow: var(--btn-shadow);
    backdrop-filter: blur(12px);
    transition: box-shadow 0.2s, transform 0.15s, opacity 0.2s;
    max-width: 280px;
  }

  .now-playing:disabled { opacity: 0.45; cursor: default; }
  .now-playing:not(:disabled):hover {
    box-shadow: var(--card-shadow-hover);
    transform: translateY(-1px);
  }

  .now-playing-art {
    width: 40px;
    height: 40px;
    border-radius: 7px;
    background: rgba(90, 95, 120, 0.15);
    flex-shrink: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    color: var(--text-dim);
  }

  .now-playing-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .now-playing-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .track-name {
    font-size: 13px;

    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 160px;
  }

  .track-artist {
    font-size: 11px;
    color: var(--text-secondary);
  }

  /* Transport controls */
  .transport {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .transport-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 14px;
    color: var(--text-secondary);
    padding: 4px 6px;
    border-radius: 6px;
    transition: color 0.15s, background 0.15s;
    line-height: 1;
  }

  .transport-btn:hover:not(:disabled) {
    color: var(--track-hover);
    background: none;
  }

  .transport-btn:disabled { opacity: 0.35; cursor: default; }

  .play-btn { font-size: 16px; }

  /* PS2 buttons */
  .actions {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .action-hint {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .btn-label {
    font-size: 11px;
    color: var(--text-secondary);
    letter-spacing: 0.03em;
  }
</style>
