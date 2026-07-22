<script lang="ts">
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  let visible = true;

  onMount(() => {
    // Solid immediate state, only handles timeout for transition
    setTimeout(() => {
      visible = false;
      setTimeout(() => dispatch('done'), 400);
    }, 1200); // Slightly longer to cover all initial IPC
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
    z-index: 9999; /* Absolute highest during boot */
    background: #FAF5FF; /* Solid start to prevent flicker */
    background: linear-gradient(160deg, #FAF5FF 0%, #FCF7FA 60%, #FFF5F9 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1), visibility 0.4s;
    /* No fadeIn animation that starts from 0 opacity */
  }

  .splash.fade-out {
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
  }

  .splash-inner {
    text-align: center;
    /* Subtle movement ok, but no opacity change from 0 */
    animation: riseIn 0.5s cubic-bezier(0.16, 1, 0.3, 1) both;
  }

  @keyframes riseIn {
    from { transform: translateY(10px); }
    to { transform: translateY(0); }
  }

  .sparkle {
    font-size: 3.5rem;
    color: #7C3AED;
    margin-bottom: 0.5rem;
    filter: drop-shadow(0 4px 16px rgba(124, 58, 237, 0.35));
  }

  .brand {
    font-family: 'Outfit', sans-serif;
    font-size: 2.6rem;
    font-weight: 800;
    color: #171717;
    letter-spacing: 0.05em;
    margin-bottom: 0.4rem;
  }

  .tagline {
    font-size: 1.1rem;
    color: #737373;
    font-weight: 500;
  }
</style>
