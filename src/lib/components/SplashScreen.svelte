<script lang="ts">
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  let visible = true;

  onMount(() => {
    // Auto-dismiss after 900ms (300ms fade-in + 300ms hold + 300ms fade-out)
    setTimeout(() => {
      visible = false;
      setTimeout(() => dispatch('done'), 350);
    }, 900);
  });
</script>

<div class="splash" class:fade-out={!visible}>
  <div class="splash-inner">
    <div class="sparkle">✦</div>
    <h1 class="brand">ХРОНИКИ</h1>
    <p class="tagline">Сохраняй моменты жизни</p>
  </div>
</div>

<style>
  .splash {
    position: fixed;
    inset: 0;
    z-index: 999;
    background: linear-gradient(160deg, #FAF5FF 0%, #FCF7FA 60%, #FFF5F9 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.3s ease both;
    transition: opacity 0.3s ease;
  }

  .splash.fade-out {
    opacity: 0;
    pointer-events: none;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .splash-inner {
    text-align: center;
    animation: riseIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) 0.05s both;
  }

  @keyframes riseIn {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .sparkle {
    font-size: 3rem;
    color: #7C3AED;
    margin-bottom: 0.4rem;
    filter: drop-shadow(0 4px 16px rgba(124, 58, 237, 0.35));
  }

  .brand {
    font-family: 'Outfit', -apple-system, sans-serif;
    font-size: 2.4rem;
    font-weight: 700;
    color: #171717;
    letter-spacing: 0.04em;
    margin-bottom: 0.3rem;
  }

  .tagline {
    font-family: 'Inter', -apple-system, sans-serif;
    font-size: 1rem;
    color: #737373;
  }
</style>
