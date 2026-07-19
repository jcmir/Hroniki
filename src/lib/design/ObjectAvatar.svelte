<script lang="ts">
  export let icon: string = '✨';
  export let size: 'sm' | 'md' | 'lg' = 'md';
</script>

<div class="object-avatar size-{size}">
  <span class="avatar-icon">{icon}</span>
</div>

<style>
  .object-avatar {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    background: linear-gradient(135deg, rgba(124, 58, 237, 0.15), rgba(236, 72, 153, 0.15));
    border: 1px solid var(--border-accent);
    box-shadow: 0 4px 12px rgba(124, 58, 237, 0.1);
  }

  .size-sm {
    width: 36px;
    height: 36px;
    font-size: 1.1rem;
  }

  .size-md {
    width: 48px;
    height: 48px;
    font-size: 1.5rem;
  }

  .size-lg {
    width: 64px;
    height: 64px;
    font-size: 2.2rem;
  }
</style>
