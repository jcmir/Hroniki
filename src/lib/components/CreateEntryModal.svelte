<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { entriesStore } from '$lib/stores/entries';

  const dispatch = createEventDispatcher();

  let title = '';
  let content = '';
  let photoPathInput = '';
  let photoPaths: string[] = [];
  let isSaving = false;

  function handleAddPhoto() {
    if (photoPathInput.trim()) {
      photoPaths = [...photoPaths, photoPathInput.trim()];
      photoPathInput = '';
    }
  }

  function handleRemovePhoto(index: number) {
    photoPaths = photoPaths.filter((_, i) => i !== index);
  }

  async function handleSave() {
    if (!title.trim()) return;

    isSaving = true;
    const success = await entriesStore.createEntry(title.trim(), content.trim(), undefined, photoPaths);
    isSaving = false;

    if (success) {
      dispatch('close');
    }
  }
</script>

<div class="modal-backdrop" on:click|self={() => dispatch('close')}>
  <div class="modal-card">
    <header class="modal-header">
      <h2>Новое Воспоминание</h2>
      <button class="close-btn" on:click={() => dispatch('close')}>✕</button>
    </header>

    <div class="modal-body">
      <div class="form-group">
        <label for="title">Название записи *</label>
        <input
          id="title"
          type="text"
          bind:value={title}
          placeholder="Например: Поездка в горы или ДЕНЬ РОЖДЕНИЯ..."
          required
        />
      </div>

      <div class="form-group">
        <label for="content">История / Заметка</label>
        <textarea
          id="content"
          bind:value={content}
          rows="4"
          placeholder="Что особенного произошло в этот момент? Опишите подробности..."
        ></textarea>
      </div>

      <div class="form-group">
        <label for="photos">Фотография / Вложение</label>
        <div class="photo-input-row">
          <input
            id="photos"
            type="text"
            bind:value={photoPathInput}
            placeholder="Путь к фото или URL..."
          />
          <button class="add-photo-btn" type="button" on:click={handleAddPhoto}>
            + Фото
          </button>
        </div>

        {#if photoPaths.length > 0}
          <div class="attached-photos">
            {#each photoPaths as path, index}
              <div class="attached-chip">
                <span>{path.split('/').pop()}</span>
                <button type="button" class="remove-chip" on:click={() => handleRemovePhoto(index)}>✕</button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <footer class="modal-footer">
      <button class="cancel-btn" type="button" on:click={() => dispatch('close')}>Отмена</button>
      <button class="save-btn" type="button" disabled={!title.trim() || isSaving} on:click={handleSave}>
        {isSaving ? 'Сохранение...' : 'Сохранить'}
      </button>
    </footer>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(8px);
    display: flex;
    justify-content: center;
    align-items: flex-end;
    z-index: 1000;
  }

  @media (min-width: 640px) {
    .modal-backdrop {
      align-items: center;
    }
  }

  .modal-card {
    background-color: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-top-left-radius: var(--radius-lg);
    border-top-right-radius: var(--radius-lg);
    width: 100%;
    max-width: 520px;
    padding: 1.5rem;
    box-shadow: var(--shadow-card);
    animation: slideUp 0.25s ease-out;
  }

  @media (min-width: 640px) {
    .modal-card {
      border-radius: var(--radius-lg);
    }
  }

  @keyframes slideUp {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.25rem;
  }

  .modal-header h2 {
    font-family: var(--font-heading);
    font-size: 1.35rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0.25rem;
  }

  .modal-body {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .form-group label {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  input[type='text'],
  textarea {
    width: 100%;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 0.85rem 1rem;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.2s ease;
  }

  input[type='text']:focus,
  textarea:focus {
    border-color: var(--accent-amber);
  }

  .photo-input-row {
    display: flex;
    gap: 0.5rem;
  }

  .add-photo-btn {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-accent);
    color: var(--accent-amber);
    padding: 0 1rem;
    border-radius: var(--radius-md);
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
  }

  .attached-photos {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-top: 0.4rem;
  }

  .attached-chip {
    background-color: var(--accent-amber-glow);
    border: 1px solid var(--border-accent);
    color: var(--accent-amber);
    font-size: 0.8rem;
    padding: 0.25rem 0.6rem;
    border-radius: var(--radius-pill);
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .remove-chip {
    background: none;
    border: none;
    color: var(--accent-amber);
    cursor: pointer;
    font-size: 0.8rem;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .cancel-btn {
    background: none;
    border: 1px solid var(--border-subtle);
    color: var(--text-muted);
    padding: 0.75rem 1.25rem;
    border-radius: var(--radius-pill);
    font-weight: 500;
    cursor: pointer;
  }

  .save-btn {
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    padding: 0.75rem 1.5rem;
    border-radius: var(--radius-pill);
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s ease;
  }

  .save-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
