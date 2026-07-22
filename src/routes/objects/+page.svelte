<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import MemoryCounter from '$lib/design/MemoryCounter.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';

  // Component can receive openCreateObject from layout via slot props
  // but since SvelteKit doesn't easily pass props to pages from layout
  // we'll use an event listener or a store. Let's use an event for now.
  export let openCreateObject: () => void = () => {};

  interface ObjectDto {
    id: string;
    category_id: string;
    name: string;
    description: string | null;
    created_at: string;
  }

  let objectsList: ObjectDto[] = [];
  let loading = true;

  async function loadObjects() {
    loading = true;
    try {
      objectsList = await invoke<ObjectDto[]>('get_objects');
    } catch {
      objectsList = [];
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadObjects();

    // Refresh list when object is created
    const handleCreated = () => loadObjects();
    window.addEventListener('object-created', handleCreated);
    return () => window.removeEventListener('object-created', handleCreated);
  });

  function handleOpenObject(id: string) {
    goto(`/object/${id}`);
  }

  function handleCreateClick() {
    // In Svelte 5 with Layout slots, we can call the prop
    if (openCreateObject) {
       openCreateObject();
    } else {
       // Fallback: search for layout's open function or use dispatch
       window.dispatchEvent(new CustomEvent('request-create-object'));
    }
  }
</script>

<div class="objects-page">
  <main class="page-content">
    <div class="section-title">
      <div class="title-row">
        <h2>Все Объекты Хроники</h2>
        <button class="add-obj-btn" on:click={handleCreateClick}>+</button>
      </div>
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
        description="Создайте свой первый жизненный объект (автомобиль, дом, путешествие), чтобы добавлять к нему воспоминания."
        buttonText="+ Создать объект"
        on:action={handleCreateClick}
      />
    {:else}
      <div class="objects-grid">
        {#each objectsList as obj (obj.id)}
          <div class="grid-card-wrapper" on:click={() => handleOpenObject(obj.id)}>
            <GlassCard hoverEffect={true}>
              <div class="object-card-content">
                <ObjectAvatar icon={obj.name.split(' ')[0] || '✨'} size="md" />
                <div class="object-info">
                  <h3 class="obj-name">{obj.name}</h3>
                  {#if obj.description}
                    <p class="obj-desc">{obj.description}</p>
                  {/if}
                  <!-- Real entries count could be fetched per object if needed -->
                  <MemoryCounter entriesCount={0} photosCount={0} />
                </div>
              </div>
            </GlassCard>
          </div>
        {/each}
      </div>
    {/if}
  </main>
</div>

<style>
  .objects-page {
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .section-title {
    margin-bottom: 1.5rem;
  }

  .title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .section-title h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .add-obj-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--accent-primary);
    color: white;
    border: none;
    font-size: 1.5rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
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
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 240px;
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

  @keyframes spin { to { transform: rotate(360deg); } }
</style>
