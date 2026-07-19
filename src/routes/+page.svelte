<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import { entriesStore, entriesByDate } from '$lib/stores/entries';
  import { sessionStore } from '$lib/stores/session';
  import { platformStore } from '$lib/stores/platform';
  import EntryCard from '$lib/components/EntryCard.svelte';
  import CreateEntryModal from '$lib/components/CreateEntryModal.svelte';
  import PINLockModal from '$lib/components/PINLockModal.svelte';

  let showCreateModal = false;

  onMount(async () => {
    await sessionStore.init();
    await platformStore.loadCapabilities();
    await entriesStore.loadEntries();
  });

  function handleLockApp() {
    sessionStore.lock();
  }
</script>

<div class="journal-app">
  <!-- Top Mobile Bar -->
  <header class="app-bar">
    <div class="bar-title">
      <span class="brand-icon">📖</span>
      <div class="brand-text">
        <h1>ХРОНИКИ</h1>
        <span class="sub-text">Личный Дневник Воспоминаний</span>
      </div>
    </div>

    <div class="bar-actions">
      <button class="lock-action-btn" title="Заблокировать" on:click={handleLockApp}>
        🔒
      </button>
    </div>
  </header>

  <!-- Main Timeline Container -->
  <main class="timeline-content">
    {#if $entriesStore.loading && $entriesStore.entries.length === 0}
      <div class="state-container">
        <div class="spinner"></div>
        <p>Загрузка хроник...</p>
      </div>
    {:else if $entriesByDate.length === 0}
      <div class="state-container empty-state">
        <div class="empty-icon">✨</div>
        <h3>Ваш Дневник Пок Пуст</h3>
        <p>Сохраняйте ценные моменты и фотографии. Нажмите <strong>+</strong>, чтобы добавить первое воспоминание.</p>
        <button class="first-entry-btn" on:click={() => (showCreateModal = true)}>
          + Создать Запись
        </button>
      </div>
    {:else}
      <div class="timeline-list">
        {#each $entriesByDate as group (group.date)}
          <section class="date-group">
            <h2 class="date-header">
              <span class="calendar-icon">📅</span>
              {group.date}
            </h2>

            <div class="entries-stack">
              {#each group.items as entry (entry.id)}
                <EntryCard {entry} />
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {/if}
  </main>

  <!-- Floating Action Button (FAB) -->
  <button
    class="fab-btn"
    aria-label="Создать новое воспоминание"
    on:click={() => (showCreateModal = true)}
  >
    <span class="fab-icon">+</span>
  </button>

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
    gap: 0.75rem;
  }

  .brand-icon {
    font-size: 1.6rem;
  }

  .brand-text h1 {
    font-family: var(--font-heading);
    font-size: 1.2rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--text-main);
    line-height: 1.1;
  }

  .sub-text {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .bar-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .lock-action-btn {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    color: var(--text-main);
    width: 40px;
    height: 40px;
    border-radius: var(--radius-pill);
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .timeline-content {
    flex: 1;
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.25rem 1rem;
  }

  .state-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 4rem 1.5rem;
    color: var(--text-muted);
  }

  .empty-state {
    background-color: var(--bg-surface);
    border: 1px dashed var(--border-subtle);
    border-radius: var(--radius-lg);
    margin-top: 2rem;
  }

  .empty-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .empty-state h3 {
    font-family: var(--font-heading);
    font-size: 1.25rem;
    color: var(--text-main);
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    font-size: 0.9rem;
    line-height: 1.5;
    margin-bottom: 1.5rem;
    max-width: 380px;
  }

  .first-entry-btn {
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    font-weight: 600;
    padding: 0.75rem 1.5rem;
    border-radius: var(--radius-pill);
    cursor: pointer;
  }

  .date-group {
    margin-bottom: 2rem;
  }

  .date-header {
    font-family: var(--font-heading);
    font-size: 1rem;
    font-weight: 600;
    color: var(--accent-amber);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.85rem;
    text-transform: capitalize;
  }

  .calendar-icon {
    font-size: 0.95rem;
  }

  .entries-stack {
    display: flex;
    flex-direction: column;
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

  /* Floating Action Button */
  .fab-btn {
    position: fixed;
    bottom: 2rem;
    right: 1.5rem;
    z-index: 90;
    width: 60px;
    height: 60px;
    border-radius: var(--radius-pill);
    background: linear-gradient(135deg, var(--accent-amber), #d97706);
    border: none;
    color: #000;
    box-shadow: var(--shadow-card), var(--shadow-glow);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: transform 0.2s ease;
  }

  .fab-btn:hover {
    transform: scale(1.06);
  }

  .fab-icon {
    font-size: 2rem;
    font-weight: 300;
    line-height: 1;
  }
</style>
