<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, slide, scale } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/core';
  import '../app.css';
  import TimelineCard from '$lib/components/TimelineCard.svelte';
  import BottomNav from '$lib/components/BottomNav.svelte';
  import Card from '$lib/components/Card.svelte';

  // Navigation state
  let activeTab = $state<'feed' | 'objects' | 'reminders' | 'settings'>('feed');
  let showAddModal = $state(false);

  // Selected date state
  let selectedDateIndex = $state(0);
  const dates = [
    { day: '16', month: 'Июл', weekday: 'Среда' },
    { day: '15', month: 'Июл', weekday: 'Вторник' },
    { day: '14', month: 'Июл', weekday: 'Понедельник' },
    { day: '13', month: 'Июл', weekday: 'Воскресенье' },
    { day: '12', month: 'Июл', weekday: 'Суббота' }
  ];

  // Forms state
  let newEntryTitle = $state('');
  let newEntryDesc = $state('');
  let newEntryCategory = $state('Сад');
  let newEntryPhotos = $state<string[]>([]);
  let notifyToggle = $state(false);

  // Categories & Mock data
  const feedItems = [
    {
      categoryName: 'Сад',
      categoryIcon: '🌱',
      categoryTheme: 'green' as const,
      time: '08:45',
      content: 'Опрыскала томаты Фитовермом от тли. Листья выглядят намного лучше.',
      images: ['/garden_tomatoes.png'],
      tags: ['томаты', 'обработка', 'сад'],
      commentsCount: 2,
      likesCount: 4
    },
    {
      categoryName: 'Здоровье',
      categoryIcon: '❤️',
      categoryTheme: 'pink' as const,
      time: '07:30',
      content: 'Сдал анализы, самочувствие стало лучше. Продолжаю бег по утрам.',
      images: ['/running_shoes.png'],
      tags: ['бег', 'здоровье', 'утро'],
      commentsCount: 1,
      likesCount: 6
    },
    {
      categoryName: 'Авто',
      categoryIcon: '🚗',
      categoryTheme: 'blue' as const,
      time: '18:15',
      content: 'Заменил масло и фильтры. Машина чувствует себя отлично.',
      images: ['/car_maintenance.png'],
      tags: ['авто', 'масло', 'обслуживание'],
      commentsCount: 3,
      likesCount: 9
    }
  ];

  // Handle add category through Tauri commands (demonstration)
  async function testTauriCommand() {
    try {
      const categoryId = await invoke('create_category', { name: newEntryCategory });
      console.log('Created category with ID:', categoryId);
    } catch (e) {
      console.error('Error invoking Tauri command:', e);
    }
  }

  function handleSaveEntry() {
    // Add mock entry to feed logic or just close modal
    showAddModal = false;
    testTauriCommand();
    newEntryTitle = '';
    newEntryDesc = '';
  }
</script>

<main class="app-shell">
  <!-- Status bar spacer for mobile/desktop app header look -->
  <header class="app-header">
    <div class="header-logo">
      <span class="logo-spark">✨</span>
      <h1>ХРОНИКИ</h1>
    </div>
    <div class="header-actions">
      <button class="icon-btn" aria-label="Search">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
      </button>
      <button class="icon-btn" aria-label="Filters">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <line x1="4" y1="21" x2="4" y2="14"/><line x1="4" y1="10" x2="4" y2="3"/>
          <line x1="12" y1="21" x2="12" y2="12"/><line x1="12" y1="8" x2="12" y2="3"/>
          <line x1="20" y1="21" x2="20" y2="16"/><line x1="20" y1="12" x2="20" y2="3"/>
          <line x1="1" y1="14" x2="7" y2="14"/><line x1="9" y1="8" x2="15" y2="8"/>
          <line x1="17" y1="16" x2="23" y2="16"/>
        </svg>
      </button>
    </div>
  </header>

  <!-- Main Content Container with max width of mobile device -->
  <div class="content-viewport">
    {#if activeTab === 'feed'}
      <!-- Feed Tab: Date Selector & Timeline -->
      <section class="feed-section" in:fade={{ duration: 200 }}>
        <!-- Horizontal Scrollable Date selector -->
        <div class="date-selector">
          {#each dates as d, idx}
            <button
              class="date-pill {selectedDateIndex === idx ? 'active' : ''}"
              onclick={() => selectedDateIndex = idx}
            >
              <span class="date-day">{d.day}</span>
              <span class="date-month">{d.month}</span>
            </button>
          {/each}
        </div>

        <div class="selected-day-banner">
          <span class="selected-day-num">{dates[selectedDateIndex].day}</span>
          <div class="selected-day-details">
            <span class="selected-day-month">{dates[selectedDateIndex].month} Июля</span>
            <span class="selected-day-name">{dates[selectedDateIndex].weekday}</span>
          </div>
        </div>

        <!-- Vertical Timeline Feed -->
        <div class="timeline-container">
          <div class="timeline-axis"></div>
          {#each feedItems as item}
            <TimelineCard {...item} />
          {/each}
        </div>
      </section>
    {:else if activeTab === 'objects'}
      <!-- Objects Tab: Mock Details view (Right Screen in Design) -->
      <section class="objects-section" in:fade={{ duration: 200 }}>
        <!-- Object Main Card -->
        <div class="object-hero">
          <div class="object-avatar-wrapper">
            <img src="/garden_tomatoes.png" alt="Яблоня" class="object-avatar" />
          </div>
          <h2 class="object-title">Яблоня</h2>
          <span class="object-category-badge">🌱 Дерево</span>
          <span class="object-meta">Создан 30 марта 2025</span>
        </div>

        <!-- Object Statistics -->
        <div class="stats-row">
          <div class="stat-card">
            <span class="stat-value">8</span>
            <span class="stat-label">Записей</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">12</span>
            <span class="stat-label">Фото</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">2</span>
            <span class="stat-label">Напоминания</span>
          </div>
        </div>

        <!-- Specific Timeline -->
        <h3 class="section-title">История ухода</h3>
        <div class="timeline-container">
          <div class="timeline-axis"></div>
          <TimelineCard
            categoryName="Обработка"
            categoryIcon="🌱"
            categoryTheme="green"
            time="30 Июля"
            content="Повторная обработка Скором. Листья чистые."
            images={['/garden_tomatoes.png']}
            tags={['скор', 'грибок']}
          />
          <TimelineCard
            categoryName="Обработка"
            categoryIcon="pink"
            categoryTheme="pink"
            time="16 Июля"
            content="Первая обработка Скором. Обнаружен грибок."
            images={['/garden_tomatoes.png']}
            tags={['грибок', 'скор']}
          />
        </div>
      </section>
    {:else if activeTab === 'reminders'}
      <section class="empty-tab" in:fade={{ duration: 200 }}>
        <div class="empty-illustration">🔔</div>
        <h3>Нет активных напоминаний</h3>
        <p>Вы можете добавить напоминание при создании новой записи.</p>
      </section>
    {:else if activeTab === 'settings'}
      <section class="settings-section" in:fade={{ duration: 200 }}>
        <h2 class="section-title">Настройки</h2>
        <Card>
          <div class="settings-list">
            <div class="settings-item">
              <span class="setting-name">Тема оформления</span>
              <span class="setting-value">Светлая уютная</span>
            </div>
            <div class="settings-item">
              <span class="setting-name">Резервное копирование</span>
              <span class="setting-value">Включено (локально)</span>
            </div>
            <div class="settings-item">
              <span class="setting-name">Версия приложения</span>
              <span class="setting-value">0.1.0 (Tauri 2)</span>
            </div>
          </div>
        </Card>
      </section>
    {/if}
  </div>

  <!-- Bottom Navigation Bar -->
  <BottomNav
    {activeTab}
    onTabChange={(tab) => activeTab = tab}
    onAddClick={() => showAddModal = true}
  />

  <!-- Add New Entry Dialog (Middle Screen Mockup) -->
  {#if showAddModal}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }} onclick={() => showAddModal = false}>
      <div class="modal-card" transition:slide={{ duration: 300 }} onclick={(e) => e.stopPropagation()}>
        <!-- Modal Header -->
        <header class="modal-header">
          <button class="text-btn" onclick={() => showAddModal = false}>Отмена</button>
          <h2>Новая запись</h2>
          <button class="text-btn primary" onclick={handleSaveEntry}>Сохранить</button>
        </header>

        <!-- Form fields -->
        <div class="modal-form">
          <div class="form-group">
            <label class="form-label" for="category-select">Для объекта</label>
            <div class="select-wrapper">
              <select id="category-select" class="form-select" bind:value={newEntryCategory}>
                <option value="Сад">🌱 Сад</option>
                <option value="Здоровье">❤️ Здоровье</option>
                <option value="Авто">🚗 Авто</option>
              </select>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label" for="desc-input">Что произошло?</label>
            <textarea
              id="desc-input"
              class="form-textarea"
              placeholder="Опишите событие..."
              bind:value={newEntryDesc}
            ></textarea>
          </div>

          <div class="form-group">
            <label class="form-label">Фото</label>
            <div class="photo-upload-row">
              <div class="photo-uploader">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="uploader-icon">
                  <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
                  <circle cx="12" cy="13" r="4"/>
                </svg>
              </div>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">Теги</label>
            <div class="tags-picker">
              <span class="tag-pill active">#сад</span>
              <span class="tag-pill">#томаты</span>
              <span class="tag-pill">#обработка</span>
              <span class="add-tag-btn">+</span>
            </div>
          </div>

          <!-- Reminder Row -->
          <div class="form-row-toggle">
            <div class="toggle-details">
              <span class="toggle-title">Напомнить</span>
              <span class="toggle-desc">Создать задачу в напоминаниях</span>
            </div>
            <label class="switch">
              <input type="checkbox" bind:checked={notifyToggle}>
              <span class="slider round"></span>
            </label>
          </div>

          {#if notifyToggle}
            <div class="form-group" transition:slide>
              <div class="select-wrapper">
                <select class="form-select">
                  <option>Через 14 дней</option>
                  <option>Через 7 дней</option>
                  <option>Завтра</option>
                </select>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    width: 100%;
    min-height: 100vh;
    align-items: center;
    background-color: var(--background);
    padding-bottom: 120px; /* Space for BottomNav */
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 480px;
    padding: 24px 20px 12px;
    background: var(--background);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .header-logo {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .logo-spark {
    font-size: 1.4rem;
  }

  .app-header h1 {
    font-size: 1.25rem;
    font-weight: 700;
    letter-spacing: 1px;
    background: var(--gradient-main);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .header-actions {
    display: flex;
    gap: 12px;
  }

  .icon-btn {
    background: rgba(96, 37, 255, 0.05);
    border: none;
    width: 38px;
    height: 38px;
    border-radius: 50%;
    cursor: pointer;
    color: var(--primary-purple);
    display: flex;
    justify-content: center;
    align-items: center;
    transition: transform 0.2s ease, background 0.2s ease;
  }

  .icon-btn:hover {
    transform: scale(1.05);
    background: rgba(96, 37, 255, 0.08);
  }

  .icon-btn svg {
    width: 18px;
    height: 18px;
  }

  .content-viewport {
    width: 100%;
    max-width: 480px;
    padding: 0 16px;
  }

  /* Date selector */
  .date-selector {
    display: flex;
    gap: 12px;
    overflow-x: auto;
    padding: 8px 4px 16px;
    scrollbar-width: none;
  }
  .date-selector::-webkit-scrollbar {
    display: none;
  }

  .date-pill {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 56px;
    height: 74px;
    border-radius: 20px;
    border: none;
    background: var(--surface-opaque);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.02);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .date-pill.active {
    background: var(--primary-purple);
    color: white;
    box-shadow: 0 8px 20px rgba(96, 37, 255, 0.3);
    transform: scale(1.05);
  }

  .date-day {
    font-size: 1.15rem;
    font-weight: 700;
  }

  .date-month {
    font-size: 0.72rem;
    font-weight: 500;
    margin-top: 4px;
    opacity: 0.8;
  }

  .selected-day-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    margin: 16px 4px 24px;
  }

  .selected-day-num {
    font-size: 2.2rem;
    font-weight: 700;
    color: var(--text);
  }

  .selected-day-details {
    display: flex;
    flex-direction: column;
  }

  .selected-day-month {
    font-size: 0.95rem;
    font-weight: 600;
  }

  .selected-day-name {
    font-size: 0.82rem;
    color: var(--muted);
    font-weight: 500;
  }

  /* Timeline container */
  .timeline-container {
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .timeline-axis {
    position: absolute;
    left: 15px;
    top: 30px;
    bottom: 30px;
    width: 2px;
    background: linear-gradient(to bottom, var(--light-gray), rgba(0,0,0,0.01));
    z-index: 1;
  }

  /* Objects Section */
  .object-hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 28px 0;
    text-align: center;
  }

  .object-avatar-wrapper {
    width: 96px;
    height: 96px;
    border-radius: 50%;
    overflow: hidden;
    border: 4px solid white;
    box-shadow: 0 8px 24px rgba(96, 37, 255, 0.15);
    margin-bottom: 16px;
  }

  .object-avatar {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .object-title {
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 6px;
  }

  .object-category-badge {
    padding: 4px 12px;
    border-radius: 12px;
    background: var(--accent-green-bg);
    color: var(--accent-green);
    font-size: 0.8rem;
    font-weight: 600;
    margin-bottom: 8px;
  }

  .object-meta {
    font-size: 0.78rem;
    color: var(--muted);
  }

  .stats-row {
    display: flex;
    gap: 12px;
    margin-bottom: 28px;
  }

  .stat-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    background: var(--surface-opaque);
    border-radius: var(--radius-lg);
    padding: 16px;
    box-shadow: var(--card-shadow);
  }

  .stat-value {
    font-size: 1.35rem;
    font-weight: 700;
    color: var(--primary-purple);
  }

  .stat-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--muted);
    margin-top: 4px;
  }

  .section-title {
    font-size: 1.1rem;
    font-weight: 700;
    margin-bottom: 16px;
    color: var(--text);
  }

  /* Settings */
  .settings-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .settings-item {
    display: flex;
    justify-content: space-between;
    font-size: 0.92rem;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--light-gray);
  }

  .settings-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .setting-name {
    font-weight: 500;
  }

  .setting-value {
    color: var(--muted);
    font-weight: 600;
  }

  /* Empty State */
  .empty-tab {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 24px;
    text-align: center;
  }

  .empty-illustration {
    font-size: 3rem;
    margin-bottom: 16px;
  }

  .empty-tab h3 {
    font-size: 1.1rem;
    font-weight: 700;
    margin-bottom: 6px;
  }

  .empty-tab p {
    font-size: 0.88rem;
    color: var(--muted);
  }

  /* Modal Add New Entry */
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(24, 24, 27, 0.4);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: flex-end;
  }

  .modal-card {
    background: var(--background);
    width: 100%;
    max-width: 480px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    max-height: 88vh;
    overflow-y: auto;
    box-shadow: 0 -12px 48px rgba(0, 0, 0, 0.15);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px;
    border-bottom: 1px solid rgba(0,0,0,0.03);
    position: sticky;
    top: 0;
    background: var(--background);
    z-index: 2;
  }

  .modal-header h2 {
    font-size: 1.05rem;
    font-weight: 700;
  }

  .text-btn {
    background: none;
    border: none;
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--muted);
    cursor: pointer;
    font-family: inherit;
  }

  .text-btn.primary {
    color: var(--primary-purple);
    font-weight: 700;
  }

  .modal-form {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text);
  }

  .form-select,
  .form-textarea {
    width: 100%;
    background: var(--surface-opaque);
    border: 1px solid var(--light-gray);
    border-radius: var(--radius-md);
    padding: 14px;
    font-size: 0.95rem;
    color: var(--text);
    font-family: inherit;
    outline: none;
    transition: border-color 0.2s ease;
  }

  .form-select:focus,
  .form-textarea:focus {
    border-color: var(--primary-purple);
  }

  .form-textarea {
    min-height: 120px;
    resize: none;
  }

  .select-wrapper {
    position: relative;
  }

  .photo-upload-row {
    display: flex;
    gap: 12px;
  }

  .photo-uploader {
    width: 72px;
    height: 72px;
    border-radius: var(--radius-md);
    border: 2px dashed var(--muted);
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    opacity: 0.6;
    transition: opacity 0.2s ease;
  }

  .photo-uploader:hover {
    opacity: 1;
  }

  .uploader-icon {
    width: 24px;
    height: 24px;
    color: var(--muted);
  }

  .tags-picker {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }

  .tags-picker .tag-pill {
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tags-picker .tag-pill.active {
    background-color: var(--primary-purple);
    color: white;
  }

  .add-tag-btn {
    width: 28px;
    height: 28px;
    border-radius: 8px;
    background-color: var(--light-gray);
    color: var(--muted);
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 1.1rem;
    cursor: pointer;
    font-weight: 500;
  }

  /* Toggle Switch */
  .form-row-toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-top: 1px solid var(--light-gray);
    border-bottom: 1px solid var(--light-gray);
  }

  .toggle-details {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle-title {
    font-size: 0.95rem;
    font-weight: 600;
  }

  .toggle-desc {
    font-size: 0.78rem;
    color: var(--muted);
  }

  /* Toggle Switch slider */
  .switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 28px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--light-gray);
    transition: .4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .4s;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  input:checked + .slider {
    background-color: var(--primary-purple);
  }

  input:checked + .slider:before {
    transform: translateX(22px);
  }

  .slider.round {
    border-radius: 34px;
  }

  .slider.round:before {
    border-radius: 50%;
  }
</style>
