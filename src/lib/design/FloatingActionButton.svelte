<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();
</script>

<button
  class="fab-button"
  aria-label="Создать новое воспоминание"
  on:click={() => dispatch('click')}
>
  <span class="fab-plus">+</span>
</button>

<style>
  .fab-button {
    position: fixed;
    bottom: calc(var(--bottom-total-offset) + 1.5rem);
    right: calc(var(--safe-area-right) + 1.5rem);
    z-index: 600;
    width: 60px;
    height: 60px;
    border-radius: var(--radius-pill);
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink));
    border: none;
    color: #FFF;
    box-shadow: 0 8px 24px rgba(124, 58, 237, 0.35);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .fab-button:hover {
    transform: scale(1.06);
  }

  .fab-plus {
    font-size: 2.2rem;
    font-weight: 300;
    line-height: 1;
  }
</style>
