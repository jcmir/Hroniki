<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { settingsStore } from '$lib/stores/settings';

  const dispatch = createEventDispatcher();

  let profileName = $settingsStore.settings.profile_name || '';
  let backupPassword = '';
  let statusMsg = '';
  let errorMsg = '';
  let isWorking = false;

  function handleSaveProfile() {
    settingsStore.updateProfileName(profileName);
    statusMsg = 'Профиль обновлен';
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

<div class="modal-backdrop" on:click|self={() => dispatch('close')}>
  <div class="modal-card">
    <header class="modal-header">
      <h2>Настройки и Резервные Копии</h2>
      <button class="close-btn" on:click={() => dispatch('close')}>✕</button>
    </header>

    <div class="modal-body">
      <!-- Profile Section -->
      <div class="section-group">
        <h3>Профиль Хранителя</h3>
        <div class="input-row">
          <input
            type="text"
            bind:value={profileName}
            placeholder="Ваше Имя / Псевдоним..."
          />
          <button class="save-profile-btn" on:click={handleSaveProfile}>Сохранить</button>
        </div>
      </div>

      <!-- Backup Export & Import Section -->
      <div class="section-group">
        <h3>Зашифрованная Резервная Копия</h3>
        <p class="section-desc">
          Полный бэкап всех объектов, записей и оригиналов фото в зашифрованный архив <code>.hroniki</code>.
        </p>

        <div class="form-group">
          <label for="backup-pwd">Пароль шифрования бэкапа</label>
          <input
            id="backup-pwd"
            type="password"
            bind:value={backupPassword}
            placeholder="Введите надежный пароль..."
          />
        </div>

        <div class="backup-actions">
          <button class="export-btn" disabled={isWorking} on:click={handleExport}>
            📤 Экспорт Бэкапа
          </button>

          <button class="import-btn" disabled={isWorking} on:click={handleImport}>
            📥 Импорт Бэкапа
          </button>
        </div>
      </div>

      {#if statusMsg}
        <div class="msg-box success">{statusMsg}</div>
      {/if}

      {#if errorMsg}
        <div class="msg-box error">{errorMsg}</div>
      {/if}
    </div>

    <footer class="modal-footer">
      <div class="version-info">Версия ХРОНИКИ: <strong>Beta 0.2.0</strong></div>
      <button class="done-btn" on:click={() => dispatch('close')}>Готово</button>
    </footer>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: var(--bg-backdrop);
    backdrop-filter: blur(8px);
    z-index: 200;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 1rem;
  }

  .modal-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    width: 100%;
    max-width: 500px;
    box-shadow: var(--shadow-glow);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .modal-header h2 {
    font-family: var(--font-heading);
    font-size: 1.15rem;
    color: var(--text-main);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.1rem;
    cursor: pointer;
  }

  .modal-body {
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .section-group h3 {
    font-family: var(--font-heading);
    font-size: 0.95rem;
    color: var(--accent-amber);
    margin-bottom: 0.5rem;
  }

  .section-desc {
    font-size: 0.825rem;
    color: var(--text-muted);
    line-height: 1.4;
    margin-bottom: 0.75rem;
  }

  .input-row {
    display: flex;
    gap: 0.5rem;
  }

  .input-row input,
  .form-group input {
    flex: 1;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 0.6rem 0.85rem;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 0.9rem;
  }

  .save-profile-btn {
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    font-weight: 600;
    padding: 0 1rem;
    border-radius: var(--radius-md);
    cursor: pointer;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    margin-bottom: 0.75rem;
  }

  .form-group label {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .backup-actions {
    display: flex;
    gap: 0.75rem;
  }

  .export-btn,
  .import-btn {
    flex: 1;
    padding: 0.65rem 0.85rem;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    background-color: var(--bg-app);
    color: var(--text-main);
    transition: all 0.2s ease;
  }

  .export-btn:hover {
    border-color: var(--accent-amber);
    color: var(--accent-amber);
  }

  .import-btn:hover {
    border-color: var(--accent-blue);
    color: var(--accent-blue);
  }

  .msg-box {
    padding: 0.65rem 0.85rem;
    border-radius: var(--radius-md);
    font-size: 0.85rem;
  }

  .msg-box.success {
    background-color: rgba(16, 185, 129, 0.15);
    border: 1px solid var(--accent-green);
    color: var(--accent-green);
  }

  .msg-box.error {
    background-color: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #ef4444;
  }

  .modal-footer {
    padding: 1rem 1.25rem;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .version-info {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .done-btn {
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    font-weight: 600;
    padding: 0.5rem 1.2rem;
    border-radius: var(--radius-pill);
    cursor: pointer;
  }
</style>
