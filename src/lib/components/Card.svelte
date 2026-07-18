<script lang="ts">
  import { scale } from 'svelte/transition';
</script>

<div class="card" transition:scale={{ duration: 300, start: 0.95 }}>
  <slot />
</div>

<style>
  .card {
    background: var(--surface-opaque);
    border-radius: var(--radius-xl);
    box-shadow: var(--card-shadow);
    padding: 24px;
    border: 1px solid rgba(96, 37, 255, 0.03);
    transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.3s ease;
    width: 100%;
    margin-bottom: 20px;
    position: relative;
    overflow: hidden;
  }

  .card:hover {
    transform: translateY(-4px);
    box-shadow: 0 20px 48px rgba(96, 37, 255, 0.12);
  }
</style>
