<script lang="ts">
  import type { Album } from '../types';
  import AlbumCard from './AlbumCard.svelte';

  const COLS = 4;
  const ROWS = 3;
  const PER_PAGE = COLS * ROWS;

  let {
    albums,
    onselect,
  }: {
    albums: Album[];
    onselect: (album: Album) => void;
  } = $props();

  let currentPage = $state(0);

  let totalPages = $derived(Math.max(1, Math.ceil(albums.length / PER_PAGE)));
  let pageAlbums = $derived(albums.slice(currentPage * PER_PAGE, (currentPage + 1) * PER_PAGE));

  // Reset page when album list changes (new scan)
  $effect(() => {
    if (albums) currentPage = 0;
  });

  function prevPage() {
    if (currentPage > 0) currentPage--;
  }

  function nextPage() {
    if (currentPage < totalPages - 1) currentPage++;
  }
</script>

<div class="wrapper">
  <!-- Page slider -->
  <div class="slider" style="transform: translateX({-currentPage * 100}%)">
    {#each Array(totalPages) as _, pageIdx}
      <div class="page">
        <div class="grid">
          {#each albums.slice(pageIdx * PER_PAGE, (pageIdx + 1) * PER_PAGE) as album (album.id)}
            <AlbumCard {album} onclick={() => onselect(album)} />
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <!-- Pagination -->
  {#if totalPages > 1}
    <div class="pagination">
      <button class="nav-btn" onclick={prevPage} disabled={currentPage === 0}>‹</button>

      <div class="dots">
        {#each Array(totalPages) as _, i}
          <button
            class="dot"
            class:active={i === currentPage}
            onclick={() => (currentPage = i)}
            aria-label="Page {i + 1}"
          />
        {/each}
      </div>

      <button class="nav-btn" onclick={nextPage} disabled={currentPage === totalPages - 1}>›</button>
    </div>
  {/if}
</div>

<style>
  .wrapper {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* Sliding track */
  .slider {
    flex: 1;
    display: flex;
    transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    min-height: 0;
  }

  .page {
    flex-shrink: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: start;
  }

  .grid {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
    gap: 16px;
    padding: 4px 2px;
  }

  /* Pagination bar */
  .pagination {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 10px 0 4px;
  }

  .nav-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 22px;
    color: var(--text-secondary);
    line-height: 1;
    padding: 0 4px;
    transition: color 0.15s, opacity 0.15s;
  }

  .nav-btn:disabled { opacity: 0.2; cursor: default; }
  .nav-btn:not(:disabled):hover { color: var(--text-primary); }

  .dots {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    border: none;
    background: var(--text-dim);
    cursor: pointer;
    padding: 0;
    transition: background 0.2s, transform 0.2s;
  }

  .dot.active {
    background: var(--text-secondary);
    transform: scale(1.4);
  }
</style>
