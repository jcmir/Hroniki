<script lang="ts">
  export let entriesCount: number = 0;
  export let photosCount: number = 0;
</script>

<div class="memory-counter">
  <span class="count-chip">📝 {entriesCount} {entriesCount === 1 ? 'запись' : 'записей'}</span>
  <span class="count-chip">📷 {photosCount} {photosCount === 1 ? 'фото' : 'фотографий'}</span>
</div>

<style>
  .memory-counter {
    display: flex;
    gap: 0.5rem;
  }

  .count-chip {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--text-muted);
    background-color: rgba(23, 23, 23, 0.04);
    padding: 0.25rem 0.6rem;
    border-radius: var(--radius-pill);
  }
</style>
