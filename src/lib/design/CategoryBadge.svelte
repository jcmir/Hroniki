<script lang="ts">
  export let name: string = 'Личное';
  export let icon: string = '✨';
  export let color: string = '#7C3AED';
</script>

<div class="category-badge" style="--badge-color: {color}">
  <span class="badge-icon">{icon}</span>
  <span class="badge-name">{name}</span>
</div>

<style>
  .category-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.3rem 0.75rem;
    border-radius: var(--radius-pill);
    font-size: 0.75rem;
    font-weight: 600;
    background-color: rgba(124, 58, 237, 0.08);
    border: 1px solid var(--border-subtle);
    color: var(--accent-primary);
    white-space: nowrap;
  }

  .badge-icon {
    font-size: 0.85rem;
  }
</style>
