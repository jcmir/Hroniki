<script lang="ts">
  import { entriesStore, entriesByDate } from '$lib/stores/entries';
  import FilterBar from '$lib/components/FilterBar.svelte';
  import MemoryRepeatBanner from '$lib/components/MemoryRepeatBanner.svelte';
  import DateChip from '$lib/design/DateChip.svelte';
  import TimelineEntry from '$lib/design/TimelineEntry.svelte';
  import EmptyState from '$lib/design/EmptyState.svelte';

  // No need for local modal state or store init, handled by +layout.svelte
</script>

<div class="timeline-page">
  <main class="timeline-content">
    <FilterBar />

    <!-- Memory Repeat Highlight Banner -->
    {#if $entriesStore.entries.length > 0}
      <MemoryRepeatBanner
        title="Поездка в горы Алтая"
        objectName="✈️ Путешествия"
        yearsAgo={1}
      />
    {/if}

    {#if $entriesStore.loading && $entriesStore.entries.length === 0}
      <div class="state-container">
        <div class="spinner"></div>
        <p>Загрузка хроник...</p>
      </div>
    {:else if $entriesByDate.length === 0}
      <EmptyState
        title="Ваш Дневник Пока Пуст"
        description="Сохраняйте ценные моменты, жизненные объекты и фотографии."
      />
    {:else}
      <div class="timeline-list">
        {#each $entriesByDate as group (group.date)}
          <section class="date-group">
            <DateChip dateStr={group.date} />

            <div class="entries-stack">
              {#each group.items as entry (entry.id)}
                <TimelineEntry {entry} />
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {/if}
  </main>
</div>

<style>
  .timeline-page {
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 1.25rem 1rem;
  }

  .timeline-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .date-group {
    margin-bottom: 1.75rem;
  }

  .entries-stack {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
    margin-top: 1rem;
  }

  .state-container {
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
