<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';
  import '../../app.css';

  interface MemoryCenterItem {
    id: string;
    title: string;
    object_name: string | null;
    trigger_at: string;
    section: 'today' | 'on_this_day' | 'upcoming';
    years_ago: number | null;
    days_until: number | null;
  }

  let allItems: MemoryCenterItem[] = [];
  let loading = true;

  $: todayItems = allItems.filter(i => i.section === 'today');
  $: onThisDayItems = allItems.filter(i => i.section === 'on_this_day');
  $: upcomingItems = allItems.filter(i => i.section === 'upcoming');

  onMount(async () => {
    try {
      allItems = await invoke<MemoryCenterItem[]>('get_memory_center');
    } catch {
      allItems = [];
    } finally {
      loading = false;
    }
  });

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('ru-RU', {
      day: 'numeric',
      month: 'long',
    });
  }
</script>

<div class="reminders-page">
  <header class="app-bar">
    <div class="bar-title">
      <span class="brand-icon">📖</span>
      <div class="brand-text">
        <h1>ХРОНИКИ</h1>
        <span class="sub-text">Центр Памяти</span>
      </div>
    </div>

    <nav class="nav-tabs">
      <a href="/" class="tab-link">Лента</a>
      <a href="/objects" class="tab-link">Объекты</a>
      <a href="/reminders" class="tab-link active">Напоминания</a>
      <a href="/settings" class="tab-link">Настройки</a>
    </nav>
  </header>

  <main class="page-content">
    {#if loading}
      <div class="state-box">
        <div class="spinner"></div>
        <p>Загрузка центра памяти...</p>
      </div>

    {:else if allItems.length === 0}
      <EmptyState
        title="Центр Памяти Пуст"
        description="Добавьте напоминания к жизненным событиям — они появятся здесь."
      />

    {:else}
      <!-- Section: Today -->
      {#if todayItems.length > 0}
        <section class="memory-section">
          <h2 class="section-label today-label">📅 Сегодня</h2>
          <div class="items-stack">
            {#each todayItems as item (item.id)}
              <GlassCard hoverEffect={true}>
                <div class="memory-row">
                  <ObjectAvatar icon="⏰" size="sm" />
                  <div class="memory-info">
                    <p class="memory-title">{item.title}</p>
                    {#if item.object_name}
                      <p class="memory-meta">{item.object_name}</p>
                    {/if}
                  </div>
                </div>
              </GlassCard>
            {/each}
          </div>
        </section>
      {/if}

      <!-- Section: On This Day -->
      {#if onThisDayItems.length > 0}
        <section class="memory-section">
          <h2 class="section-label memory-label">🕰️ В этот день 1 год назад</h2>
          <div class="items-stack">
            {#each onThisDayItems as item (item.id)}
              <GlassCard hoverEffect={true}>
                <div class="memory-row">
                  <ObjectAvatar icon="🌿" size="sm" />
                  <div class="memory-info">
                    <p class="memory-title">{item.title}</p>
                    {#if item.object_name}
                      <p class="memory-meta">{item.object_name}</p>
                    {/if}
                    <p class="memory-date">{formatDate(item.trigger_at)}</p>
                  </div>
                </div>
              </GlassCard>
            {/each}
          </div>
        </section>
      {/if}

      <!-- Section: Upcoming -->
      {#if upcomingItems.length > 0}
        <section class="memory-section">
          <h2 class="section-label upcoming-label">🔔 Будущие события</h2>
          <div class="items-stack">
            {#each upcomingItems as item (item.id)}
              <GlassCard hoverEffect={true}>
                <div class="memory-row">
                  <ObjectAvatar icon="📅" size="sm" />
                  <div class="memory-info">
                    <p class="memory-title">{item.title}</p>
                    {#if item.days_until !== null}
                      <p class="memory-meta">
                        Через {item.days_until} {item.days_until === 1 ? 'день' : 'дней'}
                      </p>
                    {/if}
                    <p class="memory-date">{formatDate(item.trigger_at)}</p>
                  </div>
                </div>
              </GlassCard>
            {/each}
          </div>
        </section>
      {/if}
    {/if}
  </main>
</div>

<style>
  .reminders-page {
    min-height: 100vh;
    background-color: var(--bg-app);
    color: var(--text-main);
    display: flex;
    flex-direction: column;
    padding-bottom: 4rem;
  }

  .app-bar {
    position: sticky;
    top: 0;
    z-index: 100;
    background-color: var(--bg-glass);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    padding: 1rem 1.25rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .bar-title { display: flex; align-items: center; gap: 0.6rem; }
  .brand-icon { font-size: 1.5rem; }

  .brand-text h1 {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-main);
    line-height: 1.1;
  }

  .sub-text { font-size: 0.75rem; color: var(--text-muted); }

  .nav-tabs {
    display: flex;
    gap: 0.3rem;
    background-color: rgba(23, 23, 23, 0.05);
    padding: 0.2rem;
    border-radius: var(--radius-pill);
    flex-wrap: wrap;
  }

  .tab-link {
    text-decoration: none;
    font-size: 0.775rem;
    font-weight: 500;
    color: var(--text-muted);
    padding: 0.3rem 0.7rem;
    border-radius: var(--radius-pill);
    transition: all 0.2s ease;
  }

  .tab-link.active {
    background-color: #FFF;
    color: var(--accent-primary);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .page-content {
    flex: 1;
    max-width: 640px;
    width: 100%;
    margin: 0 auto;
    padding: 1.25rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .memory-section { display: flex; flex-direction: column; gap: 0.75rem; }

  .section-label {
    font-family: var(--font-heading);
    font-size: 1rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }

  .today-label { color: var(--accent-primary); }
  .memory-label { color: var(--accent-pink); }
  .upcoming-label { color: var(--accent-green); }

  .items-stack { display: flex; flex-direction: column; gap: 0.75rem; }

  .memory-row {
    display: flex;
    align-items: center;
    gap: 0.85rem;
  }

  .memory-info { display: flex; flex-direction: column; gap: 0.15rem; flex: 1; }

  .memory-title {
    font-family: var(--font-heading);
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .memory-meta { font-size: 0.8rem; color: var(--accent-primary); font-weight: 500; }
  .memory-date { font-size: 0.78rem; color: var(--text-muted); }

  .state-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 4rem 1rem;
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

  @keyframes spin { to { transform: rotate(360deg); } }
</style>
