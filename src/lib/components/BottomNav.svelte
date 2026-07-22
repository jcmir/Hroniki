<script lang="ts">
  import { page } from '$app/stores';

  $: currentPath = $page.url.pathname;

  // Logic to determine active tab based on path
  $: isFeed = currentPath === '/';
  $: isObjects = currentPath.startsWith('/objects') || currentPath.startsWith('/object/');
  $: isReminders = currentPath === '/reminders';
  $: isSettings = currentPath === '/settings';
</script>

<div class="bottom-nav-safe-area">
  <nav class="bottom-nav">
    <!-- Feed -->
    <a href="/" class="nav-item" class:active={isFeed} aria-label="Лента">
      <div class="icon-wrapper">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
          <polyline points="9 22 9 12 15 12 15 22"/>
        </svg>
      </div>
      <span class="label">Лента</span>
    </a>

    <!-- Objects -->
    <a href="/objects" class="nav-item" class:active={isObjects} aria-label="Объекты">
      <div class="icon-wrapper">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="7" height="7"/>
          <rect x="14" y="3" width="7" height="7"/>
          <rect x="14" y="14" width="7" height="7"/>
          <rect x="3" y="14" width="7" height="7"/>
        </svg>
      </div>
      <span class="label">Объекты</span>
    </a>

    <!-- Reminders -->
    <a href="/reminders" class="nav-item" class:active={isReminders} aria-label="Напоминания">
      <div class="icon-wrapper">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
          <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
        </svg>
      </div>
      <span class="label">События</span>
    </a>

    <!-- Settings -->
    <a href="/settings" class="nav-item" class:active={isSettings} aria-label="Настройки">
      <div class="icon-wrapper">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </div>
      <span class="label">Профиль</span>
    </a>
  </nav>
</div>

<style>
  .bottom-nav-safe-area {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 500;
    padding-bottom: var(--safe-area-bottom);
    background-color: var(--bg-glass);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border-top: 1px solid var(--border-subtle);
    box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.03);
  }

  .bottom-nav {
    height: var(--bottom-nav-height);
    display: flex;
    align-items: center;
    justify-content: space-around;
    padding: 0 1rem;
  }

  .nav-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    color: var(--text-muted);
    gap: 0.25rem;
    transition: all 0.2s ease;
    height: 100%;
    -webkit-tap-highlight-color: transparent;
  }

  .nav-item:active {
    transform: scale(0.92);
    color: var(--accent-primary);
  }

  .nav-item.active {
    color: var(--accent-primary);
  }

  .icon-wrapper {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nav-item svg {
    width: 22px;
    height: 22px;
    transition: transform 0.2s ease;
  }

  .nav-item.active svg {
    transform: translateY(-2px);
  }

  .label {
    font-size: 0.65rem;
    font-weight: 600;
    letter-spacing: 0.01em;
  }
</style>
