<script lang="ts">
  export let hoverEffect: boolean = true;
  export let padding: string = '1.25rem';
</script>

<div
  class="glass-card"
  class:hoverable={hoverEffect}
  style:padding={padding}
>
  <slot />
</div>

<style>
  .glass-card {
    background-color: var(--bg-surface);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-card);
    box-shadow: var(--shadow-soft);
    transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .glass-card.hoverable:hover {
    transform: translateY(-2px);
    border-color: var(--border-accent);
    box-shadow: 0 12px 36px rgba(124, 58, 237, 0.12);
  }
</style>
