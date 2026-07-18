<script lang="ts">
  interface Props {
    activeTab: 'feed' | 'objects' | 'reminders' | 'settings';
    onTabChange: (tab: 'feed' | 'objects' | 'reminders' | 'settings') => void;
    onAddClick: () => void;
  }

  let { activeTab, onTabChange, onAddClick }: Props = $props();
</script>

<div class="bottom-nav-container">
  <div class="nav-bar">
    <!-- Feed Tab -->
    <button type="button" class="nav-item {activeTab === 'feed' ? 'active' : ''}" onclick={() => onTabChange('feed')}>
      <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
        <polyline points="9 22 9 12 15 12 15 22"/>
      </svg>
      <span class="nav-label">Хроника</span>
    </button>

    <!-- Objects Tab -->
    <button type="button" class="nav-item {activeTab === 'objects' ? 'active' : ''}" onclick={() => onTabChange('objects')}>
      <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="7" height="7"/>
        <rect x="14" y="3" width="7" height="7"/>
        <rect x="14" y="14" width="7" height="7"/>
        <rect x="3" y="14" width="7" height="7"/>
      </svg>
      <span class="nav-label">Объекты</span>
    </button>

    <!-- Reminders Tab -->
    <button type="button" class="nav-item {activeTab === 'reminders' ? 'active' : ''}" onclick={() => onTabChange('reminders')}>
      <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
        <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
      </svg>
      <span class="nav-label">Напоминания</span>
    </button>

    <!-- Settings Tab -->
    <button type="button" class="nav-item {activeTab === 'settings' ? 'active' : ''}" onclick={() => onTabChange('settings')}>
      <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
      <span class="nav-label">Настройки</span>
    </button>
  </div>

  <!-- Floating Add Button -->
  <button type="button" class="fab-btn" onclick={onAddClick} aria-label="Add Chronicle">
    <svg class="fab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
      <line x1="12" y1="5" x2="12" y2="19"/>
      <line x1="5" y1="12" x2="19" y2="12"/>
    </svg>
  </button>
</div>

<style>
  .bottom-nav-container {
    position: fixed;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    max-width: 480px;
    z-index: 100;
    padding: 0 16px 20px;
    pointer-events: none;
  }

  .nav-bar {
    display: flex;
    justify-content: space-around;
    align-items: center;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border-radius: 24px;
    padding: 12px 8px;
    box-shadow: 0 12px 32px rgba(96, 37, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.5);
    pointer-events: auto;
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--muted);
    transition: color 0.2s ease, transform 0.2s ease;
    flex-grow: 1;
  }

  .nav-item:hover {
    color: var(--primary-purple);
  }

  .nav-item.active {
    color: var(--primary-purple);
    transform: translateY(-2px);
  }

  .nav-icon {
    width: 22px;
    height: 22px;
    transition: transform 0.2s ease;
  }

  .nav-item.active .nav-icon {
    transform: scale(1.1);
  }

  .nav-label {
    font-size: 0.72rem;
    font-weight: 600;
  }

  /* Floating Add Button */
  .fab-btn {
    position: absolute;
    right: 28px;
    bottom: 74px;
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: var(--gradient-main);
    color: white;
    border: none;
    cursor: pointer;
    box-shadow: 0 8px 24px rgba(96, 37, 255, 0.4);
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: auto;
    transition: transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.2s ease;
  }

  .fab-btn:hover {
    transform: scale(1.1) rotate(90deg);
    box-shadow: 0 12px 28px rgba(96, 37, 255, 0.5);
  }

  .fab-btn:active {
    transform: scale(0.95);
  }

  .fab-icon {
    width: 22px;
    height: 22px;
  }
</style>
