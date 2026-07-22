<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { entriesStore } from '$lib/stores/entries';

  const dispatch = createEventDispatcher();

  export let objectId: string | undefined = undefined;

  let title = '';
  let content = '';
  let photoPathInput = '';
  let photoPaths: string[] = [];
  let isSaving = false;
  let errorMsg = '';

  onMount(() => {
    console.log('[CreateEntryModal] Open with objectId:', objectId);
  });

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
    if (!title.trim()) {
      errorMsg = 'Пожалуйста, введите название воспоминания';
      return;
    }

    isSaving = true;
    errorMsg = '';

    try {
      let targetObjectId = objectId;

      // Temporary: fetch any object if none provided (backend requirement)
      if (!targetObjectId) {
        const { invoke } = await import('@tauri-apps/api/core');
        const objects = await invoke<any[]>('get_objects');
        if (objects && objects.length > 0) {
          targetObjectId = objects[0].id;
        } else {
          throw new Error('Сначала создайте объект (напр. Машина) в разделе Объекты');
        }
      }

      const success = await entriesStore.createEntry(
        title.trim(),
        content.trim(),
        targetObjectId,
        photoPaths
      );

      if (success) {
        dispatch('close');
      } else {
        errorMsg = 'Не удалось сохранить запись. Попробуйте еще раз.';
      }
    } catch (err) {
      console.error('[CreateEntryModal] Save failed:', err);
      errorMsg = typeof err === 'string' ? err : (err as Error).message || 'Ошибка сохранения';
    } finally {
      isSaving = false;
    }
  }

  function handleCancel() {
    dispatch('close');
  }
</script>

<div class="modal-root">
  <div class="modal-backdrop" on:click|self={handleCancel}></div>

  <div class="modal-sheet">
    <form on:submit|preventDefault={handleSave}>
      <header class="modal-header">
        <div class="header-indicator"></div>
        <div class="header-row">
          <h2>Новое воспоминание</h2>
          <button class="close-btn" type="button" on:click={handleCancel} aria-label="Закрыть">✕</button>
        </div>
      </header>

      <div class="modal-body">
        {#if errorMsg}
          <div class="error-banner">{errorMsg}</div>
        {/if}

        <div class="form-group">
          <label for="entry-title">Заголовок</label>
          <input
            id="entry-title"
            type="text"
            bind:value={title}
            placeholder="Название воспоминания"
            required
            autocomplete="off"
          />
        </div>

        <div class="form-group">
          <label for="entry-content">Подробности</label>
          <textarea
            id="entry-content"
            bind:value={content}
            rows="5"
            placeholder="Что произошло? Опишите детали..."
          ></textarea>
        </div>

        <div class="form-group">
          <label for="photos">Фотографии</label>
          <div class="photo-input-row">
            <button class="add-photo-btn disabled-btn" type="button" disabled>
              <span>📸 Выбор фото скоро появится</span>
            </button>
          </div>
        </div>

        <div class="bottom-spacer"></div>
      </div>

      <footer class="modal-footer">
        <button class="btn-secondary" type="button" on:click={handleCancel}>Отмена</button>
        <button class="btn-primary" type="submit" disabled={!title.trim() || isSaving}>
          {isSaving ? 'Сохранение...' : 'Сохранить'}
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
    -webkit-backdrop-filter: blur(4px);
  }

  .modal-sheet {
    position: relative;
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    background-color: var(--bg-surface-elevated);
    border-top-left-radius: 32px;
    border-top-right-radius: 32px;
    box-shadow: 0 -12px 40px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    max-height: calc(100dvh - var(--safe-area-top) - 24px);
    animation: slideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideIn {
    from { transform: translateY(100%); }
    to { transform: translateY(0); }
  }

  form {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .modal-header {
    padding: 0.75rem 1.5rem 1rem;
    flex-shrink: 0;
  }

  .header-indicator {
    width: 40px;
    height: 5px;
    background-color: var(--border-subtle);
    border-radius: 10px;
    margin: 0 auto 1.25rem;
  }

  .header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-row h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .close-btn {
    background: var(--bg-app);
    border: none;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    color: var(--text-muted);
    cursor: pointer;
  }

  .modal-body {
    padding: 0 1.5rem;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1.75rem;
    flex: 1;
    -webkit-overflow-scrolling: touch;
  }

  .error-banner {
    background-color: #fef2f2;
    color: #b91c1c;
    padding: 1rem;
    border-radius: 16px;
    font-size: 0.9rem;
    border: 1px solid #fee2e2;
    font-weight: 600;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .form-group label {
    font-size: 0.7rem;
    font-weight: 800;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  input[type='text'],
  textarea {
    width: 100%;
    background-color: #f9fafb;
    border: 2px solid var(--border-subtle);
    border-radius: 18px;
    padding: 1rem 1.25rem;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 1rem;
    outline: none;
    transition: all 0.2s ease;
  }

  input[type='text']:focus,
  textarea:focus {
    border-color: var(--accent-primary);
    background-color: #FFF;
    box-shadow: 0 0 0 4px rgba(124, 58, 237, 0.1);
  }

  .photo-input-row {
    display: flex;
    gap: 0.75rem;
  }

  .add-photo-btn {
    background-color: var(--accent-primary);
    color: white;
    border: none;
    width: 100%;
    height: 52px;
    border-radius: 16px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .add-photo-btn.disabled-btn {
    background-color: #f3f4f6;
    color: var(--text-muted);
    border: 1px dashed var(--border-subtle);
    cursor: not-allowed;
  }

  .attached-photos {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .attached-chip {
    background-color: rgba(124, 58, 237, 0.08);
    border: 1px solid var(--border-accent);
    color: var(--accent-primary);
    padding: 0.5rem 1rem;
    border-radius: var(--radius-pill);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .chip-text {
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .remove-chip {
    background: none;
    border: none;
    color: var(--accent-primary);
    cursor: pointer;
    font-weight: 800;
    padding: 0 4px;
  }

  .modal-footer {
    padding: 1.5rem 1.5rem calc(1.5rem + var(--safe-area-bottom));
    display: flex;
    gap: 1rem;
    background-color: var(--bg-surface-elevated);
    border-top: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .btn-primary, .btn-secondary {
    flex: 1;
    padding: 1.25rem;
    border-radius: var(--radius-pill);
    font-size: 1.05rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .btn-primary {
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink));
    border: none;
    color: white;
    box-shadow: 0 8px 24px rgba(124, 58, 237, 0.35);
  }

  .btn-primary:active {
    transform: scale(0.96);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    box-shadow: none;
    filter: grayscale(1);
  }

  .btn-secondary {
    background-color: var(--bg-app);
    border: none;
    color: var(--text-muted);
  }

  .bottom-spacer {
    height: 2rem;
    flex-shrink: 0;
  }
</style>
