<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';

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
  <main class="page-content">
    <div class="section-title">
      <h2>Центр Памяти</h2>
      <p class="subtitle">Ваши напоминания и важные даты в одном месте.</p>
    </div>

    {#if loading}
      <div class="state-box">
        <div class="spinner"></div>
        <p>Загрузка центра памяти...</p>
      </div>

    {:else if allItems.length === 0}
      <EmptyState
        title="Центр Памяти Пуст"
        description="Здесь появятся напоминания и исторические события, когда вы начнете вести Хроники."
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
          <h2 class="section-label memory-label">🕰️ В этот день {onThisDayItems[0].years_ago} год назад</h2>
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
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .section-title {
    margin-bottom: 1.5rem;
  }

  .section-title h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .subtitle {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-top: 0.2rem;
  }

  .memory-section { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 2rem; }

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
