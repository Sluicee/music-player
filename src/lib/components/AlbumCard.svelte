<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import type { Album } from '../types';

  let {
    album,
    onclick,
    onhover,
  }: {
    album: Album;
    onclick: () => void;
    onhover: (album: Album | null) => void;
  } = $props();

  const coverSrc = $derived(album.cover_art ? convertFileSrc(album.cover_art) : null);
</script>

<button
  class="card"
  {onclick}
  onmouseenter={() => onhover(album)}
  onmouseleave={() => onhover(null)}
>
  <div class="art-wrap">
    <div class="art">
      {#if coverSrc}
        <img src={coverSrc} alt={album.title} draggable="false" />
      {:else}
        <div class="art-placeholder">♪</div>
      {/if}
    </div>
    <div class="edge-r"></div>
    <div class="edge-b"></div>
  </div>
</button>

<style>
  .card {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 93px;
    flex-shrink: 0;
  }

  .art-wrap {
    position: relative;
    width: 93px;
    height: 93px;
  }

  .art {
    position: absolute;
    inset: 0;
    background: rgba(90, 95, 120, 0.18);
    overflow: hidden;
    box-shadow: 2px 3px 6px rgba(0, 0, 0, 0.22);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .art-placeholder {
    font-size: 34px;
    color: rgba(90, 95, 120, 0.45);
    text-align: center;
  }

  .edge-r {
    position: absolute;
    top: 0;
    left: 93px;
    width: 4px;
    height: 93px;
    background: linear-gradient(to right,
      rgba(10, 10, 22, 0.55),
      rgba(10, 10, 22, 0.25)
    );
  }

  .edge-b {
    position: absolute;
    top: 93px;
    left: 0;
    width: 97px;
    height: 4px;
    background: linear-gradient(to bottom,
      rgba(10, 10, 22, 0.50),
      rgba(10, 10, 22, 0.20)
    );
  }
</style>
