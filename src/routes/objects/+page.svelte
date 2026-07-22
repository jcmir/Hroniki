<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { objectsStore } from '$lib/stores/objects';
  import CreateObjectModal from '$lib/components/CreateObjectModal.svelte';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import ObjectAvatar from '$lib/design/ObjectAvatar.svelte';
  import MemoryCounter from '$lib/design/MemoryCounter.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';

  let showCreateObject = false;

  onMount(() => {
    objectsStore.loadObjects();
  });

  function handleOpenObject(id: string) {
    goto(`/object/${id}`);
  }

  function handleCreateClick() {
    objectsStore.clearError();
    showCreateObject = true;
  }

  function handleCreated() {
    showCreateObject = false;
  }
</script>

<div class="objects-page">
  <main class="page-content">
    <div class="section-title">
      <div class="title-row">
        <h2>Все Объекты Хроники</h2>
        <button class="add-obj-btn" type="button" on:click={handleCreateClick} aria-label="Создать объект">+</button>
      </div>
      <p class="subtitle">Ваши машины, недвижимость, путешествия и близкие люди.</p>
    </div>

    {#if $objectsStore.loading}
      <div class="state-box">
        <div class="spinner"></div>
        <p>Загрузка объектов...</p>
      </div>
    {:else if $objectsStore.error && $objectsStore.objects.length === 0}
      <div class="state-box error-state" role="alert">
        <p>{$objectsStore.error}</p>
        <button type="button" class="retry-btn" on:click={() => objectsStore.loadObjects()}>Повторить</button>
      </div>
    {:else if $objectsStore.objects.length === 0}
      <EmptyState
        title="Нет Созданных Объектов"
        description="Создайте свой первый жизненный объект, чтобы добавлять к нему воспоминания."
        buttonText="+ Создать объект"
        on:action={handleCreateClick}
      />
    {:else}
      <div class="objects-grid">
        {#each $objectsStore.objects as object (object.id)}
          <button class="grid-card-wrapper" type="button" on:click={() => handleOpenObject(object.id)}>
            <GlassCard hoverEffect={true}>
              <div class="object-card-content">
                <ObjectAvatar icon="✨" size="md" />
                <div class="object-info">
                  <h3 class="obj-name">{object.name}</h3>
                  {#if object.description}
                    <p class="obj-desc">{object.description}</p>
                  {/if}
                  <MemoryCounter entriesCount={0} photosCount={0} />
                </div>
              </div>
            </GlassCard>
          </button>
        {/each}
      </div>
    {/if}
  </main>
</div>

{#if showCreateObject}
  <CreateObjectModal
    on:close={() => (showCreateObject = false)}
    on:success={handleCreated}
  />
{/if}

<style>
  .objects-page {
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .section-title { margin-bottom: 1.5rem; }
  .title-row { display: flex; justify-content: space-between; align-items: center; }

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

  .subtitle { font-size: 0.85rem; color: var(--text-muted); margin-top: 0.2rem; }
  .objects-grid { display: grid; grid-template-columns: 1fr; gap: 1rem; }

  .grid-card-wrapper {
    display: block;
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    text-align: left;
    cursor: pointer;
  }

  .object-card-content { display: flex; align-items: center; gap: 1rem; }
  .object-info { flex: 1; display: flex; flex-direction: column; gap: 0.25rem; min-width: 0; }
  .obj-name { font-family: var(--font-heading); font-size: 1.1rem; font-weight: 600; color: var(--text-main); }
  .obj-desc { font-size: 0.85rem; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

  .state-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 4rem 1rem;
    color: var(--text-muted);
    text-align: center;
  }

  .error-state { color: #b91c1c; }
  .retry-btn { border: 0; border-radius: var(--radius-pill); padding: 0.7rem 1.2rem; background: var(--accent-primary); color: white; font-weight: 700; }
  .spinner { width: 32px; height: 32px; border: 3px solid var(--border-subtle); border-top-color: var(--accent-primary); border-radius: 50%; animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
