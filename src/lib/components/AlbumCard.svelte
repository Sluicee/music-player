<script lang="ts">
  import type { Album } from '../types';

  let { album, onclick }: { album: Album; onclick: () => void } = $props();
</script>

<button class="card" {onclick}>
  <div class="art-wrap">
    <div class="art">
      {#if album.cover_art}
        <img src={album.cover_art} alt={album.title} draggable="false" />
      {:else}
        <div class="art-placeholder">♪</div>
      {/if}
    </div>
    <!-- Right spine (thickness) -->
    <div class="spine-right"></div>
    <!-- Bottom spine (thickness) -->
    <div class="spine-bottom"></div>
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
    width: 150px;
    flex-shrink: 0;
  }

  .art-wrap {
    position: relative;
    width: 150px;
    height: 150px;
    transform-style: preserve-3d;
  }

  .art {
    position: absolute;
    inset: 0;
    background: rgba(90, 95, 120, 0.18);
    overflow: hidden;
    box-shadow:
      0 10px 30px rgba(0, 0, 0, 0.35),
      0 4px 10px rgba(0, 0, 0, 0.2);
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

  /* Right side — CD case spine */
  .spine-right {
    position: absolute;
    top: 4px;
    right: -7px;
    width: 7px;
    height: 100%;
    background: linear-gradient(to right,
      rgba(30, 30, 50, 0.55),
      rgba(30, 30, 50, 0.3)
    );
    transform: skewY(-0.5deg);
  }

  /* Bottom side — CD case base */
  .spine-bottom {
    position: absolute;
    bottom: -6px;
    left: 4px;
    width: 100%;
    height: 6px;
    background: linear-gradient(to bottom,
      rgba(30, 30, 50, 0.55),
      rgba(30, 30, 50, 0.25)
    );
    transform: skewX(-0.5deg);
  }

  .title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 150px;
  }
</style>
