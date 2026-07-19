<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import '../../../app.css';

  interface EntryItem {
    id: string;
    title: string;
    description?: string | null;
    occurred_at: string;
  }

  interface ObjectDetails {
    object: {
      id: string;
      name: string;
      description?: string | null;
      created_at: string;
    };
    entries_count: number;
    photos_count: number;
    entries: EntryItem[];
  }

  let objectId = $page.params.id;
  let details: ObjectDetails | null = null;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    try {
      details = await invoke<ObjectDetails>('get_object_details', { objectId });
      loading = false;
    } catch (err) {
      error = typeof err === 'string' ? err : 'Ошибка загрузки объекта';
      loading = false;
    }
  });

  function handleBack() {
    window.history.back();
  }
</script>

<div class="object-page">
  <header class="page-bar">
    <button class="back-btn" on:click={handleBack}>← Назад</button>
    <h1 class="page-title">{details?.object?.name || 'Хроника Объекта'}</h1>
  </header>

  <main class="page-content">
    {#if loading}
      <div class="state-box">
        <div class="spinner"></div>
        <p>Загрузка хроники объекта...</p>
      </div>
    {:else if error || !details}
      <div class="state-box error">
        <span class="state-icon">⚠️</span>
        <p>{error || 'Объект не найден'}</p>
        <button class="back-link-btn" on:click={handleBack}>Вернуться в Таймлайн</button>
      </div>
    {:else}
      <!-- Header Info Card -->
      <section class="object-card">
        <div class="card-header">
          <span class="object-icon">🚗</span>
          <div>
            <h2>{details.object.name}</h2>
            {#if details.object.description}
              <p class="desc">{details.object.description}</p>
            {/if}
          </div>
        </div>

        <div class="stats-row">
          <div class="stat-badge">
            <span class="stat-val">{details.entries_count}</span>
            <span class="stat-lbl">Записей</span>
          </div>
          <div class="stat-badge">
            <span class="stat-val">{details.photos_count}</span>
            <span class="stat-lbl">Фотографий</span>
          </div>
        </div>
      </section>

      <!-- History Timeline -->
      <section class="history-section">
        <h3>История Владения и Событий</h3>

        {#if details.entries.length === 0}
          <p class="no-history">К этому объекту пока не привязано событий.</p>
        {:else}
          <div class="timeline-stack">
            {#each details.entries as item (item.id)}
              <div class="history-item">
                <div class="item-date">
                  {new Date(item.occurred_at).toLocaleDateString('ru-RU', {
                    day: 'numeric',
                    month: 'long',
                    year: 'numeric'
                  })}
                </div>
                <h4 class="item-title">{item.title}</h4>
                {#if item.description}
                  <p class="item-desc">{item.description}</p>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </section>
    {/if}
  </main>
</div>

<style>
  .object-page {
    min-height: 100vh;
    background-color: var(--bg-app);
    color: var(--text-main);
    display: flex;
    flex-direction: column;
  }

  .page-bar {
    position: sticky;
    top: 0;
    z-index: 100;
    background-color: var(--bg-glass);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    padding: 1rem 1.25rem;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .back-btn {
    background: none;
    border: none;
    color: var(--accent-amber);
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
  }

  .page-title {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    font-weight: 700;
  }

  .page-content {
    flex: 1;
    max-width: 640px;
    width: 100%;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .object-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 1.25rem;
    margin-bottom: 1.5rem;
    box-shadow: var(--shadow-card);
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .object-icon {
    font-size: 2.2rem;
  }

  .card-header h2 {
    font-family: var(--font-heading);
    font-size: 1.3rem;
    color: var(--text-main);
  }

  .desc {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-top: 0.25rem;
  }

  .stats-row {
    display: flex;
    gap: 1rem;
    border-top: 1px solid var(--border-subtle);
    padding-top: 0.85rem;
  }

  .stat-badge {
    display: flex;
    flex-direction: column;
  }

  .stat-val {
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--accent-amber);
  }

  .stat-lbl {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .history-section h3 {
    font-family: var(--font-heading);
    font-size: 1.05rem;
    margin-bottom: 1rem;
    color: var(--text-main);
  }

  .timeline-stack {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  .history-item {
    background-color: var(--bg-surface);
    border-left: 3px solid var(--accent-amber);
    border-radius: 0 var(--radius-md) var(--radius-md) 0;
    padding: 0.85rem 1rem;
  }

  .item-date {
    font-size: 0.75rem;
    color: var(--accent-amber);
    font-weight: 600;
  }

  .item-title {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-main);
    margin-top: 0.2rem;
  }

  .item-desc {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-top: 0.3rem;
  }

  .state-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 1rem;
    text-align: center;
    color: var(--text-muted);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-subtle);
    border-top-color: var(--accent-amber);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .no-history {
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .back-link-btn {
    margin-top: 1rem;
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    font-weight: 600;
    padding: 0.6rem 1.2rem;
    border-radius: var(--radius-pill);
    cursor: pointer;
  }
</style>
