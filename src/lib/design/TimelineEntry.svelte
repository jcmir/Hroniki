<script lang="ts">
  import GlassCard from './GlassCard.svelte';
  import CategoryBadge from './CategoryBadge.svelte';
  import PhotoGrid from './PhotoGrid.svelte';
  import type { EntryDto } from '$lib/stores/entries';

  export let entry: EntryDto;
</script>

<GlassCard>
  <article class="timeline-entry">
    <header class="entry-header">
      <CategoryBadge
        name={entry.category_name || 'Заметка'}
        icon="🌱"
      />
      <time class="entry-time">
        {new Date(entry.created_at).toLocaleTimeString('ru-RU', { hour: '2-digit', minute: '2-digit' })}
      </time>
    </header>

    <h3 class="entry-title">{entry.title}</h3>

    {#if entry.content}
      <p class="entry-content">{entry.content}</p>
    {/if}

    {#if entry.photos && entry.photos.length > 0}
      <PhotoGrid photos={entry.photos} />
    {/if}
  </article>
</GlassCard>

<style>
  .timeline-entry {
    display: flex;
    flex-direction: column;
  }

  .entry-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.6rem;
  }

  .entry-time {
    font-size: 0.75rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  .entry-title {
    font-family: var(--font-heading);
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
    line-height: 1.3;
    margin-bottom: 0.35rem;
  }

  .entry-content {
    font-size: 0.9rem;
    color: var(--text-muted);
    line-height: 1.5;
  }
</style>
