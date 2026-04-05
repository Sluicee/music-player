<script lang="ts">
  import type { Album } from '../types';

  let { album, onclick }: { album: Album; onclick: () => void } = $props();
</script>

<button class="card" {onclick}>
  <div class="art-wrap">
    <!-- Depth layer: shifted back-bottom-right, simulates physical thickness -->
    <div class="depth"></div>
    <div class="art">
      {#if album.cover_art}
        <img src={album.cover_art} alt={album.title} draggable="false" />
      {:else}
        <div class="art-placeholder">♪</div>
      {/if}
    </div>
  </div>
  <span class="title">{album.title}</span>
</button>

<style>
  .card {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 120px;
    flex-shrink: 0;
  }

  .art-wrap {
    position: relative;
    width: 120px;
    height: 120px;
  }

  /* The "thickness" — a dark copy of the cover shifted down-right.
     The parts not covered by .art (right strip + bottom strip) are the visible sides. */
  .depth {
    position: absolute;
    inset: 0;
    background: rgba(15, 15, 28, 0.75);
    transform: translate(7px, 7px);
    /* Right side is slightly lighter than bottom for lighting */
    box-shadow: inset -2px 0 4px rgba(255,255,255,0.04);
  }

  .art {
    position: absolute;
    inset: 0;
    background: rgba(90, 95, 120, 0.18);
    overflow: hidden;
    /* Front-face shadow */
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.25),
      0 6px 20px rgba(0, 0, 0, 0.2);
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
    font-size: 26px;
    color: rgba(90, 95, 120, 0.3);
  }

  .title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 120px;
  }
</style>
