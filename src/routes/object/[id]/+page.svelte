<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import TimelineLine from '$lib/design/TimelineLine.svelte';
  import DateChip from '$lib/design/DateChip.svelte';
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
        <button class="back-link-btn" on:click={handleBack}>Вернуться в Ленту</button>
      </div>
    {:else}
      <!-- Header Info Glass Card -->
      <GlassCard hoverEffect={false}>
        <div class="card-header">
          <ObjectAvatar icon={details.object.name.split(' ')[0] || '✨'} size="lg" />
          <div class="info-block">
            <h2>{details.object.name}</h2>
            {#if details.object.description}
              <p class="desc">{details.object.description}</p>
            {/if}
          </div>
        </div>

        <div class="stats-row">
          <div class="stat-badge">
            <span class="stat-val">{details.entries_count}</span>
            <span class="stat-lbl">Записей в хронологии</span>
          </div>
          <div class="stat-badge">
            <span class="stat-val">{details.photos_count}</span>
            <span class="stat-lbl">Прикрепленных фото</span>
          </div>
        </div>
      </GlassCard>

      <!-- History Timeline -->
      <section class="history-section">
        <h3>История Владения и Развития</h3>

        {#if details.entries.length === 0}
          <p class="no-history">К этому объекту пока не привязано хронологических событий.</p>
        {:else}
          <div class="timeline-stack">
            {#each details.entries as item (item.id)}
              <div class="timeline-node-wrapper">
                <TimelineLine active={true} />
                <div class="history-node-card">
                  <GlassCard hoverEffect={true}>
                    <DateChip
                      dateStr={new Date(item.occurred_at).toLocaleDateString('ru-RU', {
                        day: 'numeric',
                        month: 'long',
                        year: 'numeric'
                      })}
                    />
                    <h4 class="item-title">{item.title}</h4>
                    {#if item.description}
                      <p class="item-desc">{item.description}</p>
                    {/if}
                  </GlassCard>
                </div>
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
    padding-bottom: 3rem;
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
    color: var(--accent-primary);
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
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 1.25rem;
    margin-bottom: 1.25rem;
  }

  .info-block h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    color: var(--text-main);
  }

  .desc {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-top: 0.25rem;
  }

  .stats-row {
    display: flex;
    gap: 2rem;
    border-top: 1px solid var(--border-subtle);
    padding-top: 1rem;
  }

  .stat-badge {
    display: flex;
    flex-direction: column;
  }

  .stat-val {
    font-size: 1.3rem;
    font-weight: 700;
    color: var(--accent-primary);
  }

  .stat-lbl {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .history-section h3 {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    margin-bottom: 1rem;
    color: var(--text-main);
  }

  .timeline-stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .timeline-node-wrapper {
    display: flex;
    align-items: stretch;
  }

  .history-node-card {
    flex: 1;
  }

  .item-title {
    font-family: var(--font-heading);
    font-size: 1.05rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .item-desc {
    font-size: 0.875rem;
    color: var(--text-muted);
    margin-top: 0.35rem;
    line-height: 1.4;
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
    border-top-color: var(--accent-primary);
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
    background-color: var(--accent-primary);
    border: none;
    color: #FFF;
    font-weight: 600;
    padding: 0.6rem 1.2rem;
    border-radius: var(--radius-pill);
    cursor: pointer;
  }
</style>
