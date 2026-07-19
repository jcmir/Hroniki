<script lang="ts">
  import type { EntryDto } from '$lib/stores/entries';

  export let entry: EntryDto;

  $: formattedTime = new Date(entry.created_at).toLocaleTimeString('ru-RU', {
    hour: '2-digit',
    minute: '2-digit',
  });
</script>

<article class="entry-card">
  <header class="card-header">
    <div class="header-left">
      <span class="time-badge">{formattedTime}</span>
      {#if entry.category_name}
        <span class="category-tag">{entry.category_name}</span>
      {/if}
    </div>
  </header>

  <h3 class="entry-title">{entry.title}</h3>

  {#if entry.content}
    <p class="entry-content">{entry.content}</p>
  {/if}

  {#if entry.photos && entry.photos.length > 0}
    <div class="photo-grid" class:multi={entry.photos.length > 1}>
      {#each entry.photos as photo}
        <div class="photo-container">
          <img src={photo.file_path} alt="Вложенное фото" class="photo-img" loading="lazy" />
        </div>
      {/each}
    </div>
  {/if}
</article>

<style>
  .entry-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 1.25rem;
    margin-bottom: 1rem;
    box-shadow: var(--shadow-card);
    transition: transform 0.2s ease, border-color 0.2s ease;
  }

  .entry-card:hover {
    border-color: var(--border-accent);
    transform: translateY(-2px);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .time-badge {
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--accent-amber);
    background-color: var(--accent-amber-glow);
    padding: 0.2rem 0.6rem;
    border-radius: var(--radius-pill);
  }

  .category-tag {
    font-size: 0.8rem;
    color: var(--text-muted);
    background: rgba(255, 255, 255, 0.05);
    padding: 0.2rem 0.6rem;
    border-radius: var(--radius-pill);
  }

  .entry-title {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    font-weight: 600;
    color: var(--text-main);
    line-height: 1.35;
    margin-bottom: 0.5rem;
  }

  .entry-content {
    font-size: 0.95rem;
    color: var(--text-muted);
    line-height: 1.55;
    white-space: pre-wrap;
    margin-bottom: 0.75rem;
  }

  .photo-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
    border-radius: var(--radius-sm);
    overflow: hidden;
    margin-top: 0.75rem;
  }

  .photo-grid.multi {
    grid-template-columns: repeat(2, 1fr);
  }

  .photo-container {
    width: 100%;
    max-height: 240px;
    overflow: hidden;
    border-radius: var(--radius-sm);
    background-color: var(--bg-surface-elevated);
  }

  .photo-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
</style>
