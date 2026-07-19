<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import { entriesStore, entriesByDate } from '$lib/stores/entries';
  import { sessionStore } from '$lib/stores/session';
  import { platformStore } from '$lib/stores/platform';
  import { categoriesStore } from '$lib/stores/categories';
  import CreateEntryModal from '$lib/components/CreateEntryModal.svelte';
  import PINLockModal from '$lib/components/PINLockModal.svelte';
  import FilterBar from '$lib/components/FilterBar.svelte';
  import MemoryRepeatBanner from '$lib/components/MemoryRepeatBanner.svelte';
  import DateChip from '$lib/design/DateChip.svelte';
  import TimelineEntry from '$lib/design/TimelineEntry.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';
  import FloatingActionButton from '$lib/design/FloatingActionButton.svelte';

  let showCreateModal = false;

  onMount(async () => {
    await sessionStore.init();
    await platformStore.loadCapabilities();
    await categoriesStore.loadCategories();
    await entriesStore.loadEntries();
  });

  function handleLockApp() {
    sessionStore.lock();
  }
</script>

<div class="journal-app">
  <!-- Top Bar -->
  <header class="app-bar">
    <div class="bar-title">
      <span class="brand-icon">📖</span>
      <div class="brand-text">
        <h1>ХРОНИКИ</h1>
        <span class="sub-text">Архив Воспоминаний</span>
      </div>
      <span class="version-chip">Beta 0.2.0</span>
    </div>

    <!-- Navigation Tabs -->
    <nav class="nav-tabs">
      <a href="/" class="tab-link active">Лента</a>
      <a href="/objects" class="tab-link">Объекты</a>
      <a href="/settings" class="tab-link">Настройки</a>
    </nav>
  </header>

  <!-- Main Timeline Container -->
  <main class="timeline-content">
    <FilterBar />

    <!-- Memory Repeat Highlight Banner -->
    {#if $entriesStore.entries.length > 0}
      <MemoryRepeatBanner
        title="Поездка в горы Алтая"
        objectName="✈️ Путешествия"
        yearsAgo={1}
      />
    {/if}

    {#if $entriesStore.loading && $entriesStore.entries.length === 0}
      <div class="state-container">
        <div class="spinner"></div>
        <p>Загрузка хроник...</p>
      </div>
    {:else if $entriesByDate.length === 0}
      <EmptyState
        title="Ваш Дневник Пока Пуст"
        description="Сохраняйте ценные моменты, жизненные объекты и фотографии."
        on:action={() => (showCreateModal = true)}
      />
    {:else}
      <div class="timeline-list">
        {#each $entriesByDate as group (group.date)}
          <section class="date-group">
            <DateChip dateStr={group.date} />

            <div class="entries-stack">
              {#each group.items as entry (entry.id)}
                <TimelineEntry {entry} />
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {/if}
  </main>

  <FloatingActionButton on:click={() => (showCreateModal = true)} />

  <!-- Modals -->
  {#if showCreateModal}
    <CreateEntryModal on:close={() => (showCreateModal = false)} />
  {/if}

  {#if $sessionStore.isLocked}
    <PINLockModal />
  {/if}
</div>

<style>
  .journal-app {
    min-height: 100vh;
    background-color: var(--bg-app);
    color: var(--text-main);
    display: flex;
    flex-direction: column;
    position: relative;
    padding-bottom: 5rem;
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

  .bar-title {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .brand-icon {
    font-size: 1.5rem;
  }

  .brand-text h1 {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-main);
    line-height: 1.1;
  }

  .sub-text {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .version-chip {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--accent-primary);
    background-color: rgba(124, 58, 237, 0.1);
    border: 1px solid var(--border-accent);
    padding: 0.15rem 0.5rem;
    border-radius: var(--radius-pill);
    margin-left: 0.4rem;
  }

  .nav-tabs {
    display: flex;
    gap: 0.4rem;
    background-color: rgba(23, 23, 23, 0.05);
    padding: 0.2rem;
    border-radius: var(--radius-pill);
  }

  .tab-link {
    text-decoration: none;
    font-size: 0.825rem;
    font-weight: 500;
    color: var(--text-muted);
    padding: 0.35rem 0.85rem;
    border-radius: var(--radius-pill);
    transition: all 0.2s ease;
  }

  .tab-link.active {
    background-color: #FFF;
    color: var(--accent-primary);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .timeline-content {
    flex: 1;
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.25rem 1rem;
  }

  .date-group {
    margin-bottom: 1.75rem;
  }

  .entries-stack {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  .state-container {
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

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
