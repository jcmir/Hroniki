<script lang="ts">
  import { settingsStore } from '$lib/stores/settings';
  import GlassCard from '$lib/design/GlassCard.svelte';
  import '../../app.css';

  let profileName = $settingsStore.settings.profile_name || '';
  let backupPassword = '';
  let statusMsg = '';
  let errorMsg = '';
  let isWorking = false;

  function handleSaveProfile() {
    settingsStore.updateProfileName(profileName);
    statusMsg = 'Профиль успешно сохранен';
  }

  async function handleExport() {
    if (!backupPassword) {
      errorMsg = 'Введите пароль для шифрования бэкапа';
      return;
    }
    errorMsg = '';
    statusMsg = '';
    isWorking = true;
    try {
      const msg = await settingsStore.exportBackup(backupPassword);
      statusMsg = msg || 'Резервная копия сохранена';
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : 'Ошибка экспорта';
    } finally {
      isWorking = false;
    }
  }

  async function handleImport() {
    if (!backupPassword) {
      errorMsg = 'Введите пароль для расшифровки бэкапа';
      return;
    }
    errorMsg = '';
    statusMsg = '';
    isWorking = true;
    try {
      await settingsStore.importBackup(backupPassword);
      statusMsg = 'Данные успешно восстановлены!';
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : 'Ошибка импорта';
    } finally {
      isWorking = false;
    }
  }
</script>

<div class="settings-page">
  <header class="app-bar">
    <div class="bar-title">
      <span class="brand-icon">📖</span>
      <div class="brand-text">
        <h1>ХРОНИКИ</h1>
        <span class="sub-text">Настройки и Бэкап</span>
      </div>
    </div>

    <nav class="nav-tabs">
      <a href="/" class="tab-link">Лента</a>
      <a href="/objects" class="tab-link">Объекты</a>
      <a href="/reminders" class="tab-link">Напоминания</a>
      <a href="/settings" class="tab-link active">Настройки</a>
    </nav>
  </header>

  <main class="page-content">
    <div class="section-title">
      <h2>Параметры и Защита</h2>
      <p class="subtitle">Управление профилем, безопасностью и резервным копированием.</p>
    </div>

    <div class="settings-stack">
      <!-- Profile Glass Card -->
      <GlassCard hoverEffect={false}>
        <div class="settings-section">
          <h3>Профиль Хранителя</h3>
          <div class="form-group">
            <label for="profile-name">Имя или Псевдоним</label>
            <div class="input-row">
              <input
                id="profile-name"
                type="text"
                bind:value={profileName}
                placeholder="Ваше имя..."
              />
              <button class="save-btn" on:click={handleSaveProfile}>Сохранить</button>
            </div>
          </div>
        </div>
      </GlassCard>

      <!-- Encrypted Backup Glass Card -->
      <GlassCard hoverEffect={false}>
        <div class="settings-section">
          <h3>Зашифрованный Бэкап</h3>
          <p class="desc-text">
            Полный бэкап всех объектов, хроник и фото в зашифрованный архив <code>.hroniki</code> с манифестом версии.
          </p>

          <div class="form-group">
            <label for="pwd-input">Пароль шифрования</label>
            <input
              id="pwd-input"
              type="password"
              bind:value={backupPassword}
              placeholder="Пароль бэкапа..."
            />
          </div>

          <div class="btn-row">
            <button class="export-btn" disabled={isWorking} on:click={handleExport}>
              📤 Экспортировать Бэкап
            </button>

            <button class="import-btn" disabled={isWorking} on:click={handleImport}>
              📥 Импортировать Бэкап
            </button>
          </div>
        </div>
      </GlassCard>

      {#if statusMsg}
        <div class="status-msg success">{statusMsg}</div>
      {/if}

      {#if errorMsg}
        <div class="status-msg error">{errorMsg}</div>
      {/if}

      <div class="app-version-card">
        <span>Версия приложения:</span>
        <strong class="ver-badge">Beta 0.2.0</strong>
      </div>
    </div>
  </main>
</div>

<style>
  .settings-page {
    min-height: 100vh;
    background-color: var(--bg-app);
    color: var(--text-main);
    display: flex;
    flex-direction: column;
    padding-bottom: 5rem;
  }

  .app-bar {
    position: sticky;
    top: 0;
    z-index: 100;
    background-color: var(--bg-glass);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    padding: 1rem 1.25rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .bar-title {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .brand-icon {
    font-size: 1.5rem;
  }

  .brand-text h1 {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .sub-text {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .nav-tabs {
    display: flex;
    gap: 0.4rem;
    background-color: rgba(23, 23, 23, 0.05);
    padding: 0.2rem;
    border-radius: var(--radius-pill);
  }

  .tab-link {
    text-decoration: none;
    font-size: 0.825rem;
    font-weight: 500;
    color: var(--text-muted);
    padding: 0.35rem 0.85rem;
    border-radius: var(--radius-pill);
    transition: all 0.2s ease;
  }

  .tab-link.active {
    background-color: #FFF;
    color: var(--accent-primary);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .page-content {
    flex: 1;
    max-width: 640px;
    width: 100%;
    margin: 0 auto;
    padding: 1.5rem 1rem;
  }

  .section-title {
    margin-bottom: 1.25rem;
  }

  .section-title h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .subtitle {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-top: 0.2rem;
  }

  .settings-stack {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .settings-section h3 {
    font-family: var(--font-heading);
    font-size: 1.1rem;
    color: var(--accent-primary);
    margin-bottom: 0.6rem;
  }

  .desc-text {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-bottom: 1rem;
    line-height: 1.4;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .form-group label {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .input-row {
    display: flex;
    gap: 0.5rem;
  }

  .input-row input,
  .form-group input {
    flex: 1;
    background-color: #FFF;
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 0.6rem 0.85rem;
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .save-btn {
    background-color: var(--accent-primary);
    border: none;
    color: #FFF;
    font-weight: 600;
    padding: 0 1.2rem;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .btn-row {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .export-btn,
  .import-btn {
    flex: 1;
    padding: 0.7rem 0.85rem;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-subtle);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    background-color: #FFF;
    color: var(--text-main);
    transition: all 0.2s ease;
  }

  .export-btn:hover {
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .import-btn:hover {
    border-color: var(--accent-blue);
    color: var(--accent-blue);
  }

  .status-msg {
    padding: 0.75rem 1rem;
    border-radius: var(--radius-sm);
    font-size: 0.85rem;
  }

  .status-msg.success {
    background-color: rgba(16, 185, 129, 0.15);
    border: 1px solid var(--accent-green);
    color: var(--accent-green);
  }

  .status-msg.error {
    background-color: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #ef4444;
  }

  .app-version-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .ver-badge {
    color: var(--accent-primary);
  }
</style>
