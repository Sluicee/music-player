<script lang="ts">
  import type { Album } from '../types';

  let { album, onclick }: { album: Album; onclick: () => void } = $props();

  function formatDuration(secs: number): string {
    const m = Math.floor(secs / 60);
    const h = Math.floor(m / 60);
    if (h > 0) return `${h}h ${m % 60}m`;
    return `${m}m`;
  }
</script>

<button class="card" {onclick}>
  <div class="art">
    {#if album.cover_art}
      <img src={album.cover_art} alt={album.title} />
    {:else}
      <div class="art-placeholder">
        <span>♪</span>
      </div>
    {/if}
  </div>
  <div class="info">
    <span class="title">{album.title}</span>
    <span class="artist">{album.artist}</span>
    <span class="meta">
      {album.tracks.length} tracks · {formatDuration(album.total_duration)}
    </span>
  </div>
</button>

<style>
  .card {
    background: var(--card-bg);
    border: none;
    border-radius: 12px;
    padding: 10px;
    cursor: pointer;
    text-align: left;
    box-shadow: var(--card-shadow);
    backdrop-filter: blur(10px);
    transition: transform 0.18s ease, box-shadow 0.18s ease;
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 100%;
  }

  .card:hover {
    transform: translateY(-4px);
    box-shadow: var(--card-shadow-hover);
  }

  .card:active {
    transform: translateY(-1px);
  }

  .art {
    width: 100%;
    aspect-ratio: 1;
    border-radius: 8px;
    overflow: hidden;
    background: rgba(90, 95, 120, 0.12);
  }

  .art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .art-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    color: var(--text-dim);
    opacity: 0.5;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0 2px 2px;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .artist {
    font-size: 12px;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 2px;
  }
</style>
