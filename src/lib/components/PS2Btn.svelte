<script lang="ts">
  let { type }: { type: 'cross' | 'circle' | 'square' | 'triangle' | 'start' | 'select' } = $props();

  const colors = {
    cross:    '#5b8cff',
    circle:   '#e03040',
    square:   '#e040b0',
    triangle: '#30b855',
  };

  let color = $derived(type in colors ? colors[type as keyof typeof colors] : '#d9deef');
  let centerLabel = $derived(type === 'start' ? 'START' : type === 'select' ? 'SELECT' : '');
</script>

<span class="btn" class:btn--center={type === 'start' || type === 'select'}>
  {#if type === 'start' || type === 'select'}
    <span class="center-label">{centerLabel}</span>
  {:else}
    <svg viewBox="0 0 20 20" width="20" height="20" xmlns="http://www.w3.org/2000/svg">
      {#if type === 'cross'}
        <line x1="5"  y1="5"  x2="15" y2="15" stroke={color} stroke-width="2.8" stroke-linecap="round"/>
        <line x1="15" y1="5"  x2="5"  y2="15" stroke={color} stroke-width="2.8" stroke-linecap="round"/>
      {:else if type === 'circle'}
        <circle cx="10" cy="10" r="5.5" fill="none" stroke={color} stroke-width="2.5"/>
      {:else if type === 'square'}
        <rect x="4.5" y="4.5" width="11" height="11" fill="none" stroke={color} stroke-width="2.5" stroke-linejoin="round"/>
      {:else if type === 'triangle'}
        <polygon points="10,4 17,16 3,16" fill="none" stroke={color} stroke-width="2.5" stroke-linejoin="round" stroke-linecap="round"/>
      {/if}
    </svg>
  {/if}
</span>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    flex-shrink: 0;
    /* Gray plastic button cap */
    background: radial-gradient(circle at 38% 32%, #474747, #272727);
    box-shadow:
      0 2px 4px rgba(0,0,0,0.5),
      inset 0 1px 1px rgba(255,255,255,0.35),
      inset 0 -1px 2px rgba(0,0,0,0.3);
  }

  .btn--center {
    width: auto;
    min-width: 36px;
    height: 16px;
    padding: 0 8px;
    border-radius: 999px;
    background: linear-gradient(180deg, #474747, #272727);
  }

  .center-label {
    font-size: 7px;
    letter-spacing: 0.14em;
    color: #d9deef;
    text-shadow: none;
  }

  /* suppress global text-shadow on the SVG wrapper */
  .btn { text-shadow: none; }
</style>
