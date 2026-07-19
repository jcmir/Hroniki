<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import MemoryCounter from '$lib/design/MemoryCounter.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';
  import FloatingActionButton from '$lib/design/FloatingActionButton.svelte';
  import CreateEntryModal from '$lib/components/CreateEntryModal.svelte';
  import '../../app.css';

  interface ObjectDto {
    id: string;
    category_id: string;
    name: string;
    description: string | null;
    created_at: string;
  }

  let objectsList: ObjectDto[] = [];
  let loading = true;
  let showCreateModal = false;

  onMount(async () => {
    try {
      objectsList = await invoke<ObjectDto[]>('get_objects');
    } catch {
      // Fallback local objects
      objectsList = [
        { id: '1', category_id: 'v', name: '🚗 BMW X5', description: 'Семейная машина', created_at: new Date().toISOString() },
        { id: '2', category_id: 'h', name: '🏡 Дом в Завидово', description: 'Дача и ремонт', created_at: new Date().toISOString() },
        { id: '3', category_id: 't', name: '✈️ Поездка на Алтай', description: 'Горный трекинг 2026', created_at: new Date().toISOString() },
      ];
    } finally {
      loading = false;
    }
  });

  function handleOpenObject(id: string) {
    window.location.href = `/object/${id}`;
  }
</script>

<div class="objects-page">
  <header class="app-bar">
    <div class="bar-title">
      <span class="brand-icon">📖</span>
      <div class="brand-text">
        <h1>ХРОНИКИ</h1>
        <span class="sub-text">Жизненные Объекты</span>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <nav class="nav-tabs">
      <a href="/" class="tab-link">Лента</a>
      <a href="/objects" class="tab-link active">Объекты</a>
      <a href="/settings" class="tab-link">Настройки</a>
    </nav>
  </header>

  <main class="page-content">
    <div class="section-title">
      <h2>Все Объекты Хроники</h2>
      <p class="subtitle">Ваши машины, недвижимость, путешествия и близкие люди.</p>
    </div>

    {#if loading}
      <div class="state-box">
        <div class="spinner"></div>
        <p>Загрузка объектов...</p>
      </div>
    {:else if objectsList.length === 0}
      <EmptyState
        title="Нет Созданных Объектов"
        description="Создайте свой первый жизненный объект (автомобиль, дом, путешествие)."
        on:action={() => (showCreateModal = true)}
      />
    {:else}
      <div class="objects-grid">
        {#each objectsList as obj (obj.id)}
          <!-- clickable object card -->
          <div class="grid-card-wrapper" on:click={() => handleOpenObject(obj.id)}>
            <GlassCard hoverEffect={true}>
              <div class="object-card-content">
                <ObjectAvatar icon={obj.name.split(' ')[0] || '✨'} size="md" />
                <div class="object-info">
                  <h3 class="obj-name">{obj.name}</h3>
                  {#if obj.description}
                    <p class="obj-desc">{obj.description}</p>
                  {/if}
                  <MemoryCounter entriesCount={4} photosCount={8} />
                </div>
              </div>
            </GlassCard>
          </div>
        {/each}
      </div>
    {/if}
  </main>

  <FloatingActionButton on:click={() => (showCreateModal = true)} />

  {#if showCreateModal}
    <CreateEntryModal on:close={() => (showCreateModal = false)} />
  {/if}
</div>

<style>
  .objects-page {
    min-height: 100vh;
    background-color: var(--bg-app);
    color: var(--text-main);
    display: flex;
    flex-direction: column;
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
  }

  .sub-text {
    font-size: 0.75rem;
    color: var(--text-muted);
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

  .page-content {
    flex: 1;
    max-width: 640px;
    width: 100%;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .section-title {
    margin-bottom: 1.25rem;
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

  .objects-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .grid-card-wrapper {
    cursor: pointer;
  }

  .object-card-content {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .object-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .obj-name {
    font-family: var(--font-heading);
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .obj-desc {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

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

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
