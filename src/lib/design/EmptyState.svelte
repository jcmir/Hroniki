<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let title: string = 'История Пока Пуста';
  export let description: string = 'Сохраняйте важные жизненные объекты и события. Нажмите +, чтобы добавить первое воспоминание.';
  export let buttonText: string = '+ Создать Воспоминание';

  const dispatch = createEventDispatcher();
</script>

<div class="empty-state-card">
  <div class="empty-sparkle">✨</div>
  <h3>{title}</h3>
  <p>{description}</p>
  {#if buttonText}
    <button class="cta-btn" on:click={() => dispatch('action')}>
      {buttonText}
    </button>
  {/if}
</div>

<style>
  .empty-state-card {
    background-color: var(--bg-surface);
    border: 1px dashed var(--border-accent);
    border-radius: var(--radius-card);
    padding: 3rem 1.5rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .empty-sparkle {
    font-size: 2.5rem;
    margin-bottom: 0.75rem;
  }

  .empty-state-card h3 {
    font-family: var(--font-heading);
    font-size: 1.2rem;
    color: var(--text-main);
    margin-bottom: 0.5rem;
  }

  .empty-state-card p {
    font-size: 0.875rem;
    color: var(--text-muted);
    line-height: 1.5;
    max-width: 360px;
    margin-bottom: 1.25rem;
  }

  .cta-btn {
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink));
    border: none;
    color: #FFF;
    font-weight: 600;
    padding: 0.7rem 1.5rem;
    border-radius: var(--radius-pill);
    box-shadow: 0 4px 16px rgba(124, 58, 237, 0.25);
    cursor: pointer;
  }
</style>
