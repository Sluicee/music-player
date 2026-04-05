<script lang="ts">
  let librarySize = '0.0 GB';
</script>

<div class="shell">

  <!-- Top: Memory Card info -->
  <header class="header">
    <div class="memory-card-label">Memory Card</div>
    <div class="library-size">{librarySize}</div>
  </header>

  <!-- Middle: Album grid -->
  <main class="content">
    <div class="placeholder">
      <p>No albums found</p>
    </div>
  </main>

  <!-- Bottom: Controls bar -->
  <footer class="footer">
    <!-- Current track button -->
    <button class="now-playing">
      <div class="now-playing-art"></div>
      <div class="now-playing-info">
        <span class="now-playing-track">No track playing</span>
        <span class="now-playing-artist">—</span>
      </div>
    </button>

    <!-- PS2-style action hints -->
    <div class="actions">
      <div class="action-hint">
        <span class="btn-icon cross">✕</span>
        <span class="btn-label">Select</span>
      </div>
      <div class="action-hint">
        <span class="btn-icon circle">○</span>
        <span class="btn-label">Back</span>
      </div>
      <div class="action-hint">
        <span class="btn-icon square">□</span>
        <span class="btn-label">Shuffle</span>
      </div>
      <div class="action-hint">
        <span class="btn-icon triangle">△</span>
        <span class="btn-label">Options</span>
      </div>
    </div>
  </footer>

</div>

<style>
  .shell {
    width: 100vw;
    height: 100vh;
    display: grid;
    grid-template-rows: auto 1fr auto;
    padding: 28px 32px 20px;
    gap: 0;
  }

  /* ── Header ── */
  .header {
    display: flex;
    align-items: baseline;
    gap: 14px;
    padding-bottom: 4px;
  }

  .memory-card-label {
    font-size: 22px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: 0.02em;
  }

  .library-size {
    font-size: 13px;
    font-weight: 400;
    color: var(--text-secondary);
    letter-spacing: 0.03em;
  }

  /* ── Content ── */
  .content {
    overflow-y: auto;
    padding: 24px 0;
  }

  .placeholder {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
    font-size: 13px;
    letter-spacing: 0.05em;
  }

  /* ── Footer ── */
  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 8px;
  }

  /* Now playing button */
  .now-playing {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--card-bg);
    border: none;
    border-radius: 12px;
    padding: 10px 16px 10px 10px;
    cursor: pointer;
    box-shadow: var(--btn-shadow);
    backdrop-filter: blur(12px);
    transition: box-shadow 0.2s, transform 0.15s;
    max-width: 280px;
  }

  .now-playing:hover {
    box-shadow: var(--card-shadow-hover);
    transform: translateY(-1px);
  }

  .now-playing-art {
    width: 40px;
    height: 40px;
    border-radius: 6px;
    background: rgba(90, 95, 120, 0.2);
    flex-shrink: 0;
  }

  .now-playing-info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 2px;
  }

  .now-playing-track {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .now-playing-artist {
    font-size: 11px;
    color: var(--text-secondary);
  }

  /* PS2 action hints */
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

  .btn-icon {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 600;
    flex-shrink: 0;
    box-shadow: 0 1px 4px rgba(0,0,0,0.18);
  }

  .btn-label {
    font-size: 11px;
    color: var(--text-secondary);
    letter-spacing: 0.03em;
  }

  /* PlayStation button colors */
  .cross   { background: #4a90d9; color: #fff; }
  .circle  { background: #d94a4a; color: #fff; }
  .square  { background: #d94aaa; color: #fff; }
  .triangle { background: #4aad6e; color: #fff; }
</style>
