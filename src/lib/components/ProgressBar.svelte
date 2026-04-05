<script lang="ts">
  import { position, duration } from '../stores/player';

  function fmt(s: number): string {
    const m = Math.floor(s / 60);
    const sec = Math.floor(s % 60);
    return `${m}:${sec.toString().padStart(2, '0')}`;
  }

  let pct = $derived($duration > 0 ? ($position / $duration) * 100 : 0);
</script>

<div class="progress-wrap">
  <span class="time">{fmt($position)}</span>
  <div class="bar">
    <div class="fill" style="width:{pct}%"></div>
  </div>
  <span class="time">{fmt($duration)}</span>
</div>

<style>
  .progress-wrap {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .time {
    font-size: 13px;
    color: var(--text-dim);
    min-width: 34px;
    text-align: center;
    font-variant-numeric: tabular-nums;
  }

  .bar {
    width: 200px;
    height: 3px;
    background: rgba(90, 95, 120, 0.25);
    border-radius: 2px;
    overflow: hidden;
  }

  .fill {
    height: 100%;
    background: var(--text-secondary);
    border-radius: 2px;
    transition: width 0.9s linear;
  }
</style>
