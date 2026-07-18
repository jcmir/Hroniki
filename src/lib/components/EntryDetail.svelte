<script lang="ts">
  import { fade, slide } from 'svelte/transition';

  interface Photo {
    id: string;
    entry_id: string;
    path: string;
    thumbnail: string;
  }

  interface Props {
    show: boolean;
    entryId: string;
    categoryName: string;
    categoryIcon: string;
    categoryTheme: 'green' | 'blue' | 'pink' | 'orange' | 'purple';
    time: string;
    content: string;
    images: string[];
    tags: string[];
    onClose: () => void;
    onDelete: (entryId: string) => Promise<void>;
    onSave: (entryId: string, updatedTitle: string, updatedDesc: string) => Promise<void>;
  }

  let {
    show = false,
    entryId,
    categoryName,
    categoryIcon,
    categoryTheme,
    time,
    content,
    images = [],
    tags = [],
    onClose,
    onDelete,
    onSave
  }: Props = $props();

  // Parsing title and description from content
  let contentLines = $derived(content.split('\n'));
  let initialTitle = $derived(contentLines[0] || '');
  let initialDesc = $derived(contentLines.slice(1).join('\n') || '');

  // Edit mode state
  let isEditing = $state(false);
  let editTitle = $state('');
  let editDesc = $state('');

  // Delete confirmation dialog state
  let showDeleteConfirm = $state(false);

  // Initialize edit fields when edit mode is opened
  function toggleEdit() {
    if (!isEditing) {
      editTitle = initialTitle;
      editDesc = initialDesc;
    }
    isEditing = !isEditing;
  }

  async function handleSave() {
    if (!editTitle.trim()) return;
    await onSave(entryId, editTitle, editDesc);
    isEditing = false;
  }

  async function handleDelete() {
    showDeleteConfirm = false;
    await onDelete(entryId);
    onClose();
  }
</script>

{#if show}
  <div
    class="detail-overlay"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
    role="button"
    tabindex="-1"
    aria-label="Close details"
  >
    <div
      class="detail-sheet"
      transition:slide={{ duration: 300 }}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="presentation"
    >
      <!-- Sheet Drag Handle / Top Indicator -->
      <div class="bottom-sheet-handle"></div>

      <!-- Header -->
      <header class="detail-header">
        <button type="button" class="back-btn" onclick={onClose} aria-label="Go back">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/>
          </svg>
        </button>
        <div class="header-info">
          <span class="category-indicator theme-{categoryTheme}">
            {categoryIcon} {categoryName}
          </span>
          <span class="header-time">18 июля • {time}</span>
        </div>
        <div style="width: 24px;"></div> <!-- Spacer -->
      </header>

      <div class="detail-scrollable">
        {#if !isEditing}
          <!-- View Mode -->
          <div class="detail-content">
            <h2 class="entry-title">{initialTitle}</h2>
            
            {#if initialDesc}
              <p class="entry-description">{initialDesc}</p>
            {/if}

            <!-- Tags -->
            {#if tags.length > 0}
              <div class="tags-row">
                {#each tags as tag}
                  <span class="tag-pill">#{tag}</span>
                {/each}
              </div>
            {/if}

            <!-- Image Gallery -->
            {#if images.length > 0}
              <div class="gallery-container">
                {#each images as img}
                  <div class="gallery-item">
                    <img src={img} alt="Gallery attachment" class="gallery-img" />
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {:else}
          <!-- Edit Mode -->
          <div class="edit-form">
            <div class="form-group">
              <label class="form-label" for="edit-title-input">Событие</label>
              <input
                id="edit-title-input"
                type="text"
                class="form-input"
                bind:value={editTitle}
              />
            </div>

            <div class="form-group">
              <label class="form-label" for="edit-desc-textarea">Детали</label>
              <textarea
                id="edit-desc-textarea"
                class="form-textarea"
                bind:value={editDesc}
              ></textarea>
            </div>

            <div class="edit-actions-row">
              <button type="button" class="action-btn cancel" onclick={toggleEdit}>
                Отмена
              </button>
              <button type="button" class="action-btn save" onclick={handleSave}>
                Сохранить
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Action Footer -->
      {#if !isEditing}
        <footer class="detail-footer">
          <button type="button" class="footer-action-btn delete" onclick={() => showDeleteConfirm = true}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
            Удалить
          </button>
          <button type="button" class="footer-action-btn edit" onclick={toggleEdit}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 1 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
            Редактировать
          </button>
        </footer>
      {/if}
    </div>
  </div>
{/if}

<!-- Delete Confirmation Dialog -->
{#if showDeleteConfirm}
  <div
    class="confirm-overlay"
    transition:fade={{ duration: 150 }}
    onclick={() => showDeleteConfirm = false}
    onkeydown={(e) => e.key === 'Escape' && (showDeleteConfirm = false)}
    role="button"
    tabindex="-1"
    aria-label="Cancel deletion"
  >
    <div
      class="confirm-dialog"
      transition:slide={{ duration: 200 }}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="presentation"
    >
      <h3>Удалить эту запись?</h3>
      <p>Это действие нельзя отменить. Фотографии будут безвозвратно удалены из хранилища.</p>
      <div class="confirm-actions">
        <button type="button" class="confirm-btn cancel" onclick={() => showDeleteConfirm = false}>
          Отмена
        </button>
        <button type="button" class="confirm-btn confirm" onclick={handleDelete}>
          Удалить
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .detail-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(24, 24, 27, 0.4);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    z-index: 1100;
    display: flex;
    justify-content: center;
    align-items: flex-end;
  }

  .detail-sheet {
    background: var(--background);
    width: 100%;
    max-width: 480px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    max-height: 92vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 -12px 48px rgba(0, 0, 0, 0.15);
  }

  .bottom-sheet-handle {
    width: 36px;
    height: 5px;
    background-color: var(--light-gray);
    border-radius: 3px;
    margin: 8px auto 0;
    flex-shrink: 0;
  }

  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(0,0,0,0.03);
    flex-shrink: 0;
  }

  .back-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
  }

  .back-btn svg {
    width: 20px;
    height: 20px;
  }

  .header-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .category-indicator {
    font-size: 0.8rem;
    font-weight: 700;
    padding: 3px 10px;
    border-radius: 12px;
  }

  .category-indicator.theme-green {
    background: var(--accent-green-bg);
    color: var(--accent-green);
  }
  .category-indicator.theme-pink {
    background: var(--accent-pink-bg);
    color: var(--accent-pink);
  }
  .category-indicator.theme-blue {
    background: var(--accent-blue-bg);
    color: var(--accent-blue);
  }
  .category-indicator.theme-purple {
    background: rgba(96, 37, 255, 0.05);
    color: var(--primary-purple);
  }

  .header-time {
    font-size: 0.75rem;
    color: var(--muted);
    font-weight: 500;
  }

  .detail-scrollable {
    overflow-y: auto;
    flex-grow: 1;
    padding: 24px 20px;
  }

  .detail-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .entry-title {
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text);
    line-height: 1.3;
  }

  .entry-description {
    font-size: 0.98rem;
    color: var(--text);
    line-height: 1.5;
    white-space: pre-wrap;
  }

  .tags-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 4px;
  }

  .tag-pill {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--primary-purple);
    background: rgba(96, 37, 255, 0.05);
    padding: 4px 10px;
    border-radius: 8px;
  }

  /* Gallery Grid */
  .gallery-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 8px;
  }

  .gallery-item {
    width: 100%;
    border-radius: var(--radius-lg);
    overflow: hidden;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
  }

  .gallery-img {
    width: 100%;
    height: auto;
    display: block;
    object-fit: cover;
  }

  /* Action Footer */
  .detail-footer {
    display: flex;
    border-top: 1px solid var(--light-gray);
    background: var(--background);
    padding: 16px;
    gap: 12px;
    flex-shrink: 0;
  }

  .footer-action-btn {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
    padding: 14px;
    border-radius: var(--radius-md);
    border: none;
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
    font-family: inherit;
    transition: transform 0.2s ease, opacity 0.2s;
  }

  .footer-action-btn:active {
    transform: scale(0.98);
  }

  .footer-action-btn.delete {
    background: var(--accent-pink-bg);
    color: var(--accent-pink);
  }

  .footer-action-btn.edit {
    background: var(--primary-purple);
    color: white;
    box-shadow: 0 4px 12px rgba(96, 37, 255, 0.2);
  }

  .footer-action-btn svg {
    width: 18px;
    height: 18px;
  }

  /* Edit Form */
  .edit-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text);
  }

  .form-input,
  .form-textarea {
    width: 100%;
    background: var(--surface-opaque);
    border: 1px solid var(--light-gray);
    border-radius: var(--radius-md);
    padding: 14px;
    font-size: 0.95rem;
    color: var(--text);
    font-family: inherit;
    outline: none;
  }

  .form-textarea {
    min-height: 160px;
    resize: none;
  }

  .edit-actions-row {
    display: flex;
    gap: 12px;
    margin-top: 8px;
  }

  .action-btn {
    flex: 1;
    padding: 14px;
    border-radius: var(--radius-md);
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
    border: none;
    font-family: inherit;
  }

  .action-btn.cancel {
    background: var(--light-gray);
    color: var(--text);
  }

  .action-btn.save {
    background: var(--primary-purple);
    color: white;
  }

  /* Confirm Dialog */
  .confirm-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(24, 24, 27, 0.5);
    backdrop-filter: blur(4px);
    z-index: 1200;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
  }

  .confirm-dialog {
    background: var(--background);
    width: 100%;
    max-width: 340px;
    border-radius: var(--radius-lg);
    padding: 24px;
    box-shadow: 0 10px 30px rgba(0,0,0,0.15);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .confirm-dialog h3 {
    font-size: 1.15rem;
    font-weight: 700;
  }

  .confirm-dialog p {
    font-size: 0.9rem;
    color: var(--muted);
    line-height: 1.4;
  }

  .confirm-actions {
    display: flex;
    gap: 12px;
    margin-top: 8px;
  }

  .confirm-btn {
    flex: 1;
    padding: 12px;
    border-radius: var(--radius-md);
    font-size: 0.9rem;
    font-weight: 700;
    cursor: pointer;
    border: none;
    font-family: inherit;
  }

  .confirm-btn.cancel {
    background: var(--light-gray);
    color: var(--text);
  }

  .confirm-btn.confirm {
    background: var(--accent-pink);
    color: white;
  }
</style>
