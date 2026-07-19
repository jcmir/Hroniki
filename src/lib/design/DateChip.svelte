<script lang="ts">
  export let dateStr: string = '';
</script>

<div class="date-chip">
  <span class="cal-icon">📅</span>
  <span class="date-text">{dateStr}</span>
</div>

<style>
  .date-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    font-family: var(--font-heading);
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--accent-primary);
    margin-bottom: 0.75rem;
    text-transform: capitalize;
  }

  .cal-icon {
    font-size: 0.9rem;
  }
</style>
