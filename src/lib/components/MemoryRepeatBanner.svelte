<script lang="ts">
  export let title: string = 'Первая поездка в Алтай';
  export let objectName: string = '✈️ Путешествие';
  export let yearsAgo: number = 1;

  let dismissed = false;
</script>

{#if !dismissed}
  <div class="memory-banner">
    <div class="banner-header">
      <div class="banner-tag">
        <span class="sparkle">✨</span>
        <span>В этот день {yearsAgo} {yearsAgo === 1 ? 'год' : 'года'} назад</span>
      </div>
      <button class="dismiss-btn" on:click={() => (dismissed = true)}>✕</button>
    </div>

    <div class="banner-body">
      <span class="object-badge">{objectName}</span>
      <h4 class="memory-title">{title}</h4>
      <p class="memory-hint">Нажмите, чтобы вспомнить подробности и просмотреть фотографии того дня.</p>
    </div>
  </div>
{/if}

<style>
  .memory-banner {
    background: linear-gradient(135deg, rgba(245, 158, 11, 0.15), rgba(59, 130, 246, 0.12));
    border: 1px solid var(--border-accent);
    border-radius: var(--radius-md);
    padding: 1rem 1.25rem;
    margin-bottom: 1.5rem;
    box-shadow: var(--shadow-glow);
    position: relative;
    overflow: hidden;
  }

  .banner-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .banner-tag {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--accent-amber);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .sparkle {
    font-size: 0.95rem;
  }

  .dismiss-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 0.9rem;
    cursor: pointer;
  }

  .banner-body {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .object-badge {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .memory-title {
    font-family: var(--font-heading);
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .memory-hint {
    font-size: 0.825rem;
    color: var(--text-muted);
    line-height: 1.4;
  }
</style>
