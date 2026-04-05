<script lang="ts">
  import type { Album } from '../types';
  import AlbumCard from './AlbumCard.svelte';

  let {
    albums,
    onselect,
  }: {
    albums: Album[];
    onselect: (album: Album) => void;
  } = $props();
</script>

<div class="grid">
  {#each albums as album (album.id)}
    <AlbumCard {album} onclick={() => onselect(album)} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(155px, 1fr));
    gap: 16px;
    padding: 4px 2px;
  }
</style>
