<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { categoriesStore } from '$lib/stores/categories';
  import { objectsStore } from '$lib/stores/objects';

  const dispatch = createEventDispatcher<{ close: void; success: { objectId: string } }>();

  let name = '';
  let description = '';
  let selectedCategoryId = '';
  let errorMsg = '';

  $: availableCategories = $categoriesStore.categories.filter((category) => category.id !== 'all');
  $: isSaving = $objectsStore.saving;

  async function handleSave() {
    errorMsg = '';
    objectsStore.clearError();

    if (!name.trim()) {
      errorMsg = 'Пожалуйста, введите название объекта';
      return;
    }

    if (!selectedCategoryId) {
      errorMsg = 'Выберите категорию';
      return;
    }

    const objectId = await objectsStore.createObject(
      selectedCategoryId,
      name,
      description,
    );

    if (!objectId) {
      errorMsg = $objectsStore.error || 'Ошибка создания объекта';
      return;
    }

    dispatch('success', { objectId });
  }
</script>

<div class="modal-root" role="presentation">
  <div class="modal-backdrop" on:click|self={() => dispatch('close')}></div>

  <div class="modal-sheet" role="dialog" aria-modal="true" aria-labelledby="create-object-title">
    <form on:submit|preventDefault={handleSave}>
      <header class="modal-header">
        <div class="header-indicator"></div>
        <div class="header-row">
          <h2 id="create-object-title">Новый объект</h2>
          <button class="close-btn" type="button" on:click={() => dispatch('close')} aria-label="Закрыть">✕</button>
        </div>
      </header>

      <div class="modal-body">
        {#if errorMsg}
          <div class="error-banner" role="alert">{errorMsg}</div>
        {/if}

        <div class="form-group">
          <label for="obj-name">Название</label>
          <input
            id="obj-name"
            type="text"
            bind:value={name}
            autocomplete="off"
            required
          />
        </div>

        <div class="form-group">
          <label for="obj-cat">Категория</label>
          <select id="obj-cat" bind:value={selectedCategoryId} required>
            <option value="" disabled>Выберите категорию...</option>
            {#each availableCategories as category}
              <option value={category.id}>{category.icon} {category.name}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="obj-desc">Описание (необязательно)</label>
          <textarea id="obj-desc" bind:value={description} rows="3"></textarea>
        </div>

        <div class="bottom-spacer"></div>
      </div>

      <footer class="modal-footer">
        <button class="btn-secondary" type="button" on:click={() => dispatch('close')} disabled={isSaving}>Отмена</button>
        <button class="btn-primary" type="submit" disabled={isSaving}>
          {isSaving ? 'Создание...' : 'Создать объект'}
        </button>
      </footer>
    </form>
  </div>
</div>

<style>
  .modal-root {
    position: fixed;
    inset: 0;
    z-index: 2500;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
  }

  .modal-backdrop {
    position: absolute;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
  }

  .modal-sheet {
    position: relative;
    background-color: var(--bg-surface-elevated);
    border-top-left-radius: 32px;
    border-top-right-radius: 32px;
    padding-bottom: env(safe-area-inset-bottom);
    max-height: 90dvh;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideIn {
    from { transform: translateY(100%); }
    to { transform: translateY(0); }
  }

  form { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
  .modal-header { padding: 0.75rem 1.5rem 1rem; }
  .header-indicator { width: 40px; height: 5px; background: var(--border-subtle); margin: 0 auto 1.25rem; border-radius: 10px; }
  .header-row { display: flex; justify-content: space-between; align-items: center; }
  .header-row h2 { font-size: 1.4rem; font-weight: 700; color: var(--text-main); }
  .close-btn { background: var(--bg-app); border: none; width: 36px; height: 36px; border-radius: 50%; color: var(--text-muted); cursor: pointer; }
  .modal-body { padding: 0 1.5rem; overflow-y: auto; display: flex; flex-direction: column; gap: 1.5rem; }
  .form-group { display: flex; flex-direction: column; gap: 0.5rem; }
  .form-group label { font-size: 0.7rem; font-weight: 800; color: var(--text-muted); text-transform: uppercase; }

  input, textarea, select {
    background: #f9fafb;
    border: 2px solid var(--border-subtle);
    border-radius: 16px;
    padding: 1rem;
    font-size: 1rem;
    color: var(--text-main);
    outline: none;
  }

  .error-banner { background: #fef2f2; color: #b91c1c; padding: 1rem; border-radius: 12px; font-weight: 600; margin-bottom: 1rem; }
  .modal-footer { padding: 1.5rem; display: flex; gap: 1rem; border-top: 1px solid var(--border-subtle); }
  .btn-primary, .btn-secondary { flex: 1; padding: 1.1rem; border-radius: 50px; font-weight: 700; cursor: pointer; }
  .btn-primary { background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink)); color: white; border: none; }
  .btn-secondary { background: var(--bg-app); border: none; color: var(--text-muted); }
  .btn-primary:disabled, .btn-secondary:disabled { opacity: 0.6; cursor: default; }
  .bottom-spacer { height: 2rem; }
</style>
