<script lang="ts">
  import { viewMode, type ViewMode } from "$lib/stores/viewMode";
  import { playUiSfx } from "$lib/ui-sfx";

  function switchMode(mode: ViewMode) {
    playUiSfx("confirm");
    viewMode.set(mode);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="mode-bar">
  <div class="strip"></div>
  <div class="buttons">
    <button
      class="mode-btn"
      class:active={$viewMode === "normal"}
      onclick={() => switchMode("normal")}
      title="Normal"
    >
      <!-- 2x2 grid -->
      <svg viewBox="0 0 10 10" fill="currentColor">
        <rect x="0" y="0" width="4" height="4" rx="0.5" />
        <rect x="6" y="0" width="4" height="4" rx="0.5" />
        <rect x="0" y="6" width="4" height="4" rx="0.5" />
        <rect x="6" y="6" width="4" height="4" rx="0.5" />
      </svg>
    </button>

    <button
      class="mode-btn"
      class:active={$viewMode === "focus"}
      onclick={() => switchMode("focus")}
      title="Focus"
    >
      <!-- disc/record -->
      <svg
        viewBox="0 0 10 10"
        fill="none"
        stroke="currentColor"
        stroke-width="1"
      >
        <circle cx="5" cy="5" r="4" />
        <circle cx="5" cy="5" r="1.5" fill="currentColor" stroke="none" />
      </svg>
    </button>

    <button
      class="mode-btn"
      class:active={$viewMode === "fullscreen"}
      onclick={() => switchMode("fullscreen")}
      title="Fullscreen"
    >
      <!-- expand arrows -->
      <svg
        viewBox="0 0 10 10"
        fill="none"
        stroke="currentColor"
        stroke-width="1.2"
        stroke-linecap="round"
      >
        <polyline points="0,3 0,0 3,0" />
        <polyline points="7,0 10,0 10,3" />
        <polyline points="10,7 10,10 7,10" />
        <polyline points="3,10 0,10 0,7" />
      </svg>
    </button>

    <button
      class="mode-btn"
      class:active={$viewMode === "mini"}
      onclick={() => switchMode("mini")}
      title="Mini Player"
    >
      <!-- compact bar -->
      <svg viewBox="0 0 10 10" fill="currentColor">
        <rect x="1" y="2" width="8" height="5" rx="1" />
        <rect x="0" y="8.5" width="10" height="1" rx="0.5" />
      </svg>
    </button>
  </div>
</div>

<style>
  .mode-bar {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
    width: 120px;
    height: 5px;
    border-radius: 0 0 6px 6px;
    transition: height 0.18s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  }

  .mode-bar:hover {
    height: 30px;
  }

  .strip {
    width: 40px;
    height: 3px;
    background: linear-gradient(180deg, #474747, #272727);
    border-radius: 0 0 3px 3px;
    flex-shrink: 0;
    transition: opacity 0.12s;
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
  }

  .mode-bar:hover .strip {
    opacity: 0;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 3px 8px;
    background: linear-gradient(180deg, #474747, #272727);
    border-radius: 0 0 8px 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-top: none;
    opacity: 0;
    transform: translateY(-4px);
    transition:
      opacity 0.12s 0.05s,
      transform 0.12s 0.05s;
    flex-shrink: 0;
  }

  .mode-bar:hover .buttons {
    opacity: 1;
    transform: translateY(0);
  }

  .mode-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 3px;
    color: rgba(255, 255, 255, 0.45);
    border-radius: 4px;
    transition:
      color 0.12s,
      background 0.12s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mode-btn svg {
    width: 12px;
    height: 12px;
    display: block;
  }

  .mode-btn:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
  }

  .mode-btn.active {
    color: var(--track-active);
  }
</style>
