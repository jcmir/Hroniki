<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import '../app.css';
  import TimelineCard from '$lib/components/TimelineCard.svelte';
  import BottomNav from '$lib/components/BottomNav.svelte';
  import Card from '$lib/components/Card.svelte';
  import EntryDetail from '$lib/components/EntryDetail.svelte';
  import PinLockScreen from '$lib/components/PinLockScreen.svelte';
  import OnboardingScreen from '$lib/components/OnboardingScreen.svelte';
  import ObjectCard from '$lib/components/ObjectCard.svelte';
  import ObjectChronicle from '$lib/components/ObjectChronicle.svelte';
  import Logo from '$lib/components/Logo.svelte';
  import { mockInvoke } from '$lib/mock/mockRepository';

  // Explicit Types
  import type { Category, ChronicleObject, Entry, TimelineEntry, ObjectStats, Reminder } from '$lib/types';

  // State variables from DB
  let categories = $state<Category[]>([]);
  let objects = $state<ChronicleObject[]>([]);
  let entries = $state<TimelineEntry[]>([]);
  let reminders = $state<Reminder[]>([]);

  // Navigation state
  let activeTab = $state<'feed' | 'objects' | 'reminders' | 'settings'>('feed');
  let showAddModal = $state(false);
  let showDetail = $state(false);
  let selectedEntry = $state<TimelineEntry | null>(null);

  // Selected date state
  let selectedDateIndex = $state(0);
  const dates = [
    { day: '18', month: 'Июл', weekday: 'Суббота' },
    { day: '17', month: 'Июл', weekday: 'Пятница' },
    { day: '16', month: 'Июл', weekday: 'Четверг' },
    { day: '15', month: 'Июл', weekday: 'Среда' },
    { day: '14', month: 'Июл', weekday: 'Вторник' }
  ];

  // Forms state
  let selectedObject = $state('');
  let newEntryTitle = $state('');
  let newEntryDesc = $state('');
  let notifyToggle = $state(false);
  let reminderPeriod = $state('Через 14 дней');
  
  // Selected photos paths (absolute paths before saving)
  let selectedPhotoPaths = $state<string[]>([]);

  // Search & Filter state
  let searchQueryText = $state('');
  let searchCategory = $state('');
  let searchObject = $state('');
  let searchStartDate = $state('');
  let searchEndDate = $state('');
  let showFiltersPanel = $state(false);

  // Security PIN and Backup states
  let isAppLocked = $state(false);
  let showSetPinModal = $state(false);
  let showBackupPasswordModal = $state(false);
  let showImportPasswordModal = $state(false);

  let tempPinSetup = $state('');
  let backupPasswordInput = $state('');
  let importPasswordInput = $state('');
  let isPinEnabled = $state(false);

  // Onboarding & Splash states
  let showOnboarding = $state(false);
  let currentUsername = $state('Пользователь');
  let isSplashing = $state(true);

  // Selected object chronicle view state and scroll preservation
  let selectedObjectForChronicle = $state<ChronicleObject | null>(null);
  let selectedObjectStats = $state<ObjectStats | null>(null);
  let selectedObjectEntries = $state<TimelineEntry[]>([]);
  let objectsScrollY = 0;

  onMount(async () => {
    try {
      const pinConfigured = await safeInvoke<boolean>('is_pin_configured');
      if (pinConfigured) {
        isAppLocked = true;
      }
      isPinEnabled = pinConfigured;

      const onboardingDone = await safeInvoke<boolean>('is_onboarding_completed');
      if (!onboardingDone) {
        showOnboarding = true;
      } else {
        currentUsername = await safeInvoke<string>('get_username');
      }
    } catch (e) {
      console.error('Failed to query startup configurations:', e);
    }
    await refreshData();
    setTimeout(() => {
      isSplashing = false;
    }, 2000);
  });

  async function handleOnboardingComplete(name: string) {
    try {
      await safeInvoke('complete_onboarding', { username: name });
      currentUsername = name;
      showOnboarding = false;
      await refreshData();
    } catch (e) {
      console.error('Failed to complete onboarding:', e);
    }
  }

  async function handleSeedDemoData() {
    try {
      await safeInvoke('seed_demo_data');
    } catch (e) {
      console.error('Failed to seed demo data:', e);
    }
  }

  async function handleVerifyPin(pin: string): Promise<boolean> {
    try {
      return await safeInvoke<boolean>('verify_pin', { pin });
    } catch (e) {
      console.error(e);
      return false;
    }
  }

  async function handleSetupNewPin() {
    if (tempPinSetup.length < 4) return;
    try {
      await safeInvoke('set_pin', { pin: tempPinSetup });
      isPinEnabled = true;
      showSetPinModal = false;
      tempPinSetup = '';
    } catch (e) {
      console.error('Failed to set PIN:', e);
    }
  }

  async function handleTogglePin() {
    if (isPinEnabled) {
      try {
        await safeInvoke('disable_pin');
        isPinEnabled = false;
      } catch (e) {
        console.error('Failed to disable PIN:', e);
      }
    } else {
      showSetPinModal = true;
    }
  }

  async function handleExportBackup() {
    if (!backupPasswordInput) return;
    try {
      const msg = await safeInvoke<string>('export_archive', { password: backupPasswordInput });
      alert(msg);
      showBackupPasswordModal = false;
      backupPasswordInput = '';
    } catch (e) {
      alert(`Ошибка экспорта: ${e}`);
    }
  }

  async function handleImportBackup() {
    if (!importPasswordInput) return;
    try {
      await safeInvoke('import_archive', { password: importPasswordInput });
      alert("Архив успешно восстановлен. Приложение будет перезапущено для обновления данных.");
      showImportPasswordModal = false;
      importPasswordInput = '';
      // Force reload page to reload SQLite pool and refresh everything
      if (typeof window !== 'undefined') {
        window.location.reload();
      }
    } catch (e) {
      alert(`Ошибка импорта: ${e}`);
    }
  }

  // Safe wrapper around Tauri invoke to run gracefully in standard browsers
  async function safeInvoke<T>(cmd: string, args?: any): Promise<T> {
    const isTauri = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
    if (isTauri) {
      return await invoke<T>(cmd, args);
    } else {
      return mockInvoke(cmd, args) as T;
    }
  }

  async function selectObjectForChronicle(obj: ChronicleObject) {
    objectsScrollY = window.scrollY;
    selectedObjectForChronicle = obj;
    try {
      selectedObjectStats = await safeInvoke<ObjectStats>('get_object_stats', { objectId: obj.id });
      selectedObjectEntries = entries.filter(e => e.object_id === obj.id);
    } catch (e) {
      console.error('Failed to get object stats:', e);
    }
    setTimeout(() => window.scrollTo(0, 0), 10);
  }

  function clearObjectChronicle() {
    selectedObjectForChronicle = null;
    selectedObjectStats = null;
    selectedObjectEntries = [];
    setTimeout(() => window.scrollTo(0, objectsScrollY), 10);
  }

  async function refreshData() {
    try {
      categories = await safeInvoke<any[]>('get_categories');
      objects = await safeInvoke<any[]>('get_objects');

      // Setup default mock object if empty
      if (categories.length > 0 && objects.length === 0) {
        const gardenCat = categories.find(c => c.name === 'Сад') || categories[0];
        await safeInvoke('create_object', {
          categoryId: gardenCat.id,
          name: 'Яблоня',
          description: 'Дерево в саду'
        });
        objects = await safeInvoke<any[]>('get_objects');
      }

      // Pre-select first object in form
      if (objects.length > 0 && !selectedObject) {
        selectedObject = objects[0].id;
      }

      // Query entries using search command or standard get_entries
      let rawEntries = [];
      if (searchQueryText || searchCategory || searchObject || searchStartDate || searchEndDate) {
        rawEntries = await safeInvoke<any[]>('search_entries', {
          queryText: searchQueryText || null,
          categoryId: searchCategory || null,
          objectId: searchObject || null,
          startDate: searchStartDate ? new Date(searchStartDate).toISOString() : null,
          endDate: searchEndDate ? new Date(searchEndDate).toISOString() : null
        });
      } else {
        rawEntries = await safeInvoke<any[]>('get_entries');
      }

      const loadedEntries = [];

      for (const e of rawEntries) {
        const obj = objects.find(o => o.id === e.object_id);
        const cat = obj ? categories.find(c => c.id === obj.category_id) : null;
        
        const themeMap: Record<string, 'green' | 'blue' | 'pink' | 'orange' | 'purple'> = {
          'Сад': 'green',
          'Здоровье': 'pink',
          'Авто': 'blue'
        };
        const catName = cat ? cat.name : 'Сад';

        const dateObj = new Date(e.occurred_at);
        const timeStr = dateObj.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

        // Retrieve photos from DB
        const rawPhotos = await safeInvoke<any[]>('get_entry_photos', { entryId: e.id });
        const imageUrls = [];
        for (const p of rawPhotos) {
          const isTauri = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
          if (isTauri) {
            const absolutePath = await invoke<string>('get_media_path', { filename: p.path });
            imageUrls.push(convertFileSrc(absolutePath));
          } else {
            imageUrls.push(p.path);
          }
        }

        loadedEntries.push({
          id: e.id,
          object_id: e.object_id,
          occurred_at: e.occurred_at,
          categoryName: catName,
          categoryIcon: catName === 'Сад' ? '🌱' : catName === 'Здоровье' ? '❤️' : catName === 'Авто' ? '🚗' : '✨',
          categoryTheme: themeMap[catName] || 'purple',
          time: timeStr,
          content: `${e.title}${e.description ? '\n' + e.description : ''}`,
          images: imageUrls,
          tags: catName === 'Сад' ? ['яблоня', 'уход'] : catName === 'Здоровье' ? ['здоровье'] : ['обслуживание'],
          reminderText: ''
        });
      }
      entries = loadedEntries;

      // Load reminders
      try {
        const rawReminders = await safeInvoke<any[]>('get_reminders');
        reminders = rawReminders;
      } catch (_) {
        reminders = [];
      }

      if (selectedObjectForChronicle) {
        selectedObjectEntries = entries.filter(e => e.object_id === selectedObjectForChronicle!.id);
      }
    } catch (e) {
      console.error('Failed to load data from database:', e);
    }
  }

  async function handleAddPhoto() {
    try {
      const paths = await safeInvoke<string[] | null>('select_images');
      if (paths) {
        selectedPhotoPaths = [...selectedPhotoPaths, ...paths];
      }
    } catch (e) {
      console.error('Failed to select images:', e);
    }
  }

  function handleRemovePhoto(index: number) {
    selectedPhotoPaths = selectedPhotoPaths.filter((_, i) => i !== index);
  }

  async function handleSaveEntry() {
    if (!selectedObject || !newEntryTitle) return;

    try {
      // Copy and save all picked photos to app data media originals directory
      const savedFilenames: string[] = [];
      for (const path of selectedPhotoPaths) {
        const filename = await safeInvoke<string>('save_media', { sourcePath: path });
        if (filename) {
          savedFilenames.push(filename);
        }
      }

      const entryId = await safeInvoke<string>('create_entry', {
        objectId: selectedObject,
        title: newEntryTitle,
        description: newEntryDesc || null,
        imageFilenames: savedFilenames
      });

      // Schedule reminder if toggle is enabled
      if (notifyToggle && entryId) {
        const daysMap: Record<string, number> = {
          'Через 14 дней': 14,
          'Через 7 дней': 7,
          'Завтра': 1
        };
        const days = daysMap[reminderPeriod] ?? 14;
        const triggerAt = new Date();
        triggerAt.setDate(triggerAt.getDate() + days);

        await safeInvoke('create_reminder', {
          entryId,
          triggerAt: triggerAt.toISOString(),
          repeatDays: days
        });
      }

      showAddModal = false;
      newEntryTitle = '';
      newEntryDesc = '';
      notifyToggle = false;
      selectedPhotoPaths = [];
      await refreshData();
    } catch (e) {
      console.error('Failed to save entry:', e);
    }
  }

  async function handleCompleteReminder(reminderId: string) {
    try {
      await safeInvoke('complete_reminder', { reminderId });
      await refreshData();
    } catch (e) {
      console.error('Failed to complete reminder:', e);
    }
  }

  async function handleSnoozeReminder(reminderId: string) {
    try {
      await safeInvoke('snooze_reminder', { reminderId, days: 3 });
      await refreshData();
    } catch (e) {
      console.error('Failed to snooze reminder:', e);
    }
  }
  function handleCardClick(id: string) {
    const found = entries.find(e => e.id === id);
    if (found) {
      selectedEntry = found;
      showDetail = true;
    }
  }

  async function handleDeleteEntry(id: string) {
    try {
      await safeInvoke('delete_entry', { entryId: id });
      await refreshData();
    } catch (e) {
      console.error('Failed to delete entry:', e);
    }
  }

  async function handleSaveEditedEntry(id: string, title: string, desc: string) {
    try {
      await safeInvoke('update_entry', { entryId: id, title, description: desc || null });
      await refreshData();
      
      // Update selectedEntry details in view
      const updated = entries.find(e => e.id === id);
      if (updated) {
        selectedEntry = updated;
      }
    } catch (e) {
      console.error('Failed to update entry:', e);
    }
  }
</script>

<main class="app-shell">
  <!-- Header bar -->
  <header class="app-header">
    <div class="header-logo">
      <Logo size={24} />
      <h1>Хроника {currentUsername}</h1>
    </div>
    <div class="header-actions">
      <button
        type="button"
        class="icon-btn {showFiltersPanel ? 'active' : ''}"
        aria-label="Search and filters"
        onclick={() => showFiltersPanel = !showFiltersPanel}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 20px; height: 20px;">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
      </button>
    </div>
  </header>

  <!-- Content view -->
  <div class="content-viewport">
    {#if activeTab === 'feed'}
      <!-- Feed Tab: Search, Date Selector & Timeline -->
      <section class="feed-section" in:fade={{ duration: 200 }}>
        
        <!-- Interactive Search & Filters Panel -->
        {#if showFiltersPanel}
          <div class="search-filters-panel" transition:slide={{ duration: 200 }}>
            <div class="search-input-wrapper">
              <input
                type="text"
                class="search-text-input"
                placeholder="Поиск по событиям и описанию..."
                bind:value={searchQueryText}
                oninput={refreshData}
              />
            </div>
            <div class="filter-dropdowns-row">
              <div class="filter-col">
                <label class="filter-label" for="filter-category">Категория</label>
                <select id="filter-category" class="filter-select" bind:value={searchCategory} onchange={refreshData}>
                  <option value="">Все категории</option>
                  {#each categories as cat}
                    <option value={cat.id}>{cat.name}</option>
                  {/each}
                </select>
              </div>
              <div class="filter-col">
                <label class="filter-label" for="filter-object">Объект</label>
                <select id="filter-object" class="filter-select" bind:value={searchObject} onchange={refreshData}>
                  <option value="">Все объекты</option>
                  {#each objects as obj}
                    <option value={obj.id}>{obj.name}</option>
                  {/each}
                </select>
              </div>
            </div>
            <div class="filter-dates-row">
              <div class="filter-col">
                <label class="filter-label" for="filter-start">С даты</label>
                <input id="filter-start" type="date" class="filter-date-input" bind:value={searchStartDate} onchange={refreshData} />
              </div>
              <div class="filter-col">
                <label class="filter-label" for="filter-end">По дату</label>
                <input id="filter-end" type="date" class="filter-date-input" bind:value={searchEndDate} onchange={refreshData} />
              </div>
            </div>
            <button
              type="button"
              class="clear-filters-btn"
              onclick={() => {
                searchQueryText = '';
                searchCategory = '';
                searchObject = '';
                searchStartDate = '';
                searchEndDate = '';
                refreshData();
              }}
            >Очистить фильтры</button>
          </div>
        {/if}

        <!-- Horizontal Scrollable Date selector -->
        <div class="date-selector">
          {#each dates as d, idx}
            <button
              type="button"
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
          {#if entries.length === 0}
            <div class="empty-feed">
              <span class="empty-icon">🌿</span>
              <h3>Ваша история ещё не началась</h3>
              <p>Создайте первую запись: сад, авто, дом или другое важное событие.</p>
            </div>
          {:else}
            {#each entries as item, i (item.id)}
              <TimelineCard {...item} index={i} onClick={handleCardClick} />
            {/each}
          {/if}
        </div>
      </section>
    {:else if activeTab === 'objects'}
      <!-- Objects Tab -->
      <section class="objects-section" in:fade={{ duration: 200 }}>
        {#if selectedObjectForChronicle}
          <ObjectChronicle
            object={selectedObjectForChronicle}
            stats={selectedObjectStats}
            entries={selectedObjectEntries}
            {categories}
            onBack={clearObjectChronicle}
            onCardClick={handleCardClick}
          />
        {:else}
          <!-- Objects List -->
          <h2 class="section-title">Ваши объекты</h2>
          {#if objects.length === 0}
            <div class="empty-tab">
              <div class="empty-illustration">🌱</div>
              <h3>Создайте первый объект</h3>
              <p>Например: Яблоня, Автомобиль, Дом, Питомец.</p>
            </div>
          {:else}
            <div class="objects-grid">
              {#each objects as obj, i}
                <ObjectCard
                  object={obj}
                  categoryName={categories.find((c: any) => c.id === obj.category_id)?.name ?? ''}
                  entryCount={entries.filter((e: any) => e.object_id === obj.id).length}
                  index={i}
                  onSelect={selectObjectForChronicle}
                />
              {/each}
            </div>
          {/if}
        {/if}
      </section>
    {:else if activeTab === 'reminders'}
      <section class="reminders-section" in:fade={{ duration: 200 }}>
        <h2 class="section-title">Напоминания</h2>
        {#if reminders.length === 0}
          <div class="empty-tab">
            <div class="empty-illustration">🔔</div>
            <h3>Пока ничего не нужно повторять</h3>
            <p>Когда появятся запланированные действия — мы напомним.</p>
          </div>
        {:else}
          <div class="reminders-list">
            {#each reminders as reminder (reminder.id)}
              {@const entry = entries.find(e => e.id === reminder.entry_id)}
              {@const triggerDate = new Date(reminder.trigger_at)}
              {@const isOverdue = triggerDate < new Date() && reminder.status === 'Scheduled'}
              <div class="reminder-card {reminder.status === 'Completed' ? 'completed' : ''} {isOverdue ? 'overdue' : ''}">
                <div class="reminder-icon">
                  {#if reminder.status === 'Completed'}
                    ✅
                  {:else if isOverdue}
                    🔴
                  {:else}
                    🔔
                  {/if}
                </div>
                <div class="reminder-body">
                  <span class="reminder-title">{entry ? entry.content.split('\n')[0] : 'Запись удалена'}</span>
                  <span class="reminder-meta">
                    {#if reminder.status === 'Completed'}
                      Выполнено
                    {:else if reminder.status === 'Snoozed'}
                      Отложено до {triggerDate.toLocaleDateString('ru-RU', { day: 'numeric', month: 'long' })}
                    {:else}
                      {isOverdue ? 'Просрочено: ' : 'Напомнить '}{triggerDate.toLocaleDateString('ru-RU', { day: 'numeric', month: 'long' })}
                    {/if}
                    {#if reminder.repeat_days}
                      · Повтор каждые {reminder.repeat_days} дней
                    {/if}
                  </span>
                </div>
                {#if reminder.status !== 'Completed'}
                  <div class="reminder-actions">
                    <button
                      type="button"
                      class="reminder-btn done"
                      onclick={() => handleCompleteReminder(reminder.id)}
                    >Выполнено</button>
                    <button
                      type="button"
                      class="reminder-btn snooze"
                      onclick={() => handleSnoozeReminder(reminder.id)}
                    >Отложить</button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
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
              <span class="setting-name">База данных</span>
              <span class="setting-value">SQLite (активна)</span>
            </div>
            <div class="settings-item">
              <span class="setting-name">Версия приложения</span>
              <span class="setting-value">0.1.0 (Tauri 2)</span>
            </div>
          </div>
        </Card>

        <h3 class="section-title" style="margin-top: 24px;">Безопасность</h3>
        <Card>
          <div class="settings-list">
            <div class="settings-item" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
              <span class="setting-name">Защита PIN-кодом</span>
              <label class="switch">
                <input type="checkbox" checked={isPinEnabled} onchange={handleTogglePin} />
                <span class="slider round"></span>
              </label>
            </div>
          </div>
        </Card>

        <h3 class="section-title" style="margin-top: 24px;">Резервное копирование</h3>
        <Card>
          <div class="settings-list">
            <button type="button" class="settings-action-btn" onclick={() => showBackupPasswordModal = true}>
              🛡️ Экспортировать зашифрованный архив
            </button>
            <button type="button" class="settings-action-btn" onclick={() => showImportPasswordModal = true}>
              🔑 Восстановить данные из файла
            </button>
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

  <!-- Add New Entry Dialog (Bottom Sheet Style) -->
  {#if showAddModal}
    <div
      class="modal-backdrop"
      transition:fade={{ duration: 200 }}
      onclick={() => showAddModal = false}
      onkeydown={(e) => e.key === 'Escape' && (showAddModal = false)}
      role="button"
      tabindex="-1"
      aria-label="Close sheet"
    >
      <div
        class="bottom-sheet"
        transition:slide={{ duration: 300 }}
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => e.stopPropagation()}
        role="presentation"
      >
        <!-- Drag Handle Indicator -->
        <div class="bottom-sheet-handle"></div>

        <!-- Sheet Header -->
        <header class="sheet-header">
          <button type="button" class="text-btn" onclick={() => showAddModal = false}>Отмена</button>
          <h2>Новая запись</h2>
          <button type="button" class="text-btn primary" onclick={handleSaveEntry}>Сохранить</button>
        </header>

        <!-- Form Fields -->
        <div class="sheet-form">
          <div class="form-group">
            <label class="form-label" for="object-select">Для объекта</label>
            <div class="select-wrapper">
              <select id="object-select" class="form-select" bind:value={selectedObject}>
                {#each objects as obj}
                  <option value={obj.id}>🌱 {obj.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label" for="title-input">Событие</label>
            <input
              id="title-input"
              type="text"
              class="form-input"
              placeholder="Что произошло? (например, Полила яблоню)"
              bind:value={newEntryTitle}
            />
          </div>

          <div class="form-group">
            <label class="form-label" for="desc-input">Детали (необязательно)</label>
            <textarea
              id="desc-input"
              class="form-textarea"
              placeholder="Дополнительные подробности..."
              bind:value={newEntryDesc}
            ></textarea>
          </div>

          <!-- Photo Selection and Preview Area -->
          <div class="form-group">
            <span class="form-label">Фотографии</span>
            <div class="photo-upload-row">
              <button
                type="button"
                id="photo-upload-btn"
                class="photo-uploader"
                onclick={handleAddPhoto}
                aria-label="Add photo"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="uploader-icon">
                  <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
                  <circle cx="12" cy="13" r="4"/>
                </svg>
              </button>

              <!-- Image Previews -->
              {#if selectedPhotoPaths.length > 0}
                <div class="previews-container">
                  {#each selectedPhotoPaths as path, index}
                    {@const srcUrl = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ ? convertFileSrc(path) : path}
                    <div class="preview-item">
                      <img src={srcUrl} alt="Preview" class="preview-img" />
                      <button
                        type="button"
                        class="remove-photo-btn"
                        onclick={() => handleRemovePhoto(index)}
                        aria-label="Remove photo"
                      >
                        ✕
                      </button>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </div>

          <div class="form-group">
            <label class="form-label" for="tags-container">Теги</label>
            <div id="tags-container" class="tags-picker">
              <span class="tag-pill active">#уход</span>
              <span class="tag-pill">#яблоня</span>
              <span class="tag-pill">#сад</span>
              <button type="button" class="add-tag-btn" aria-label="Add tag">+</button>
            </div>
          </div>

          <!-- Reminder Row -->
          <div class="form-row-toggle">
            <div class="toggle-details">
              <span class="toggle-title">Напомнить</span>
              <span class="toggle-desc">Создать напоминание</span>
            </div>
            <label class="switch" for="notify-toggle-cb">
              <input type="checkbox" id="notify-toggle-cb" bind:checked={notifyToggle}>
              <span class="slider round"></span>
            </label>
          </div>

          {#if notifyToggle}
            <div class="form-group" transition:slide>
              <label class="form-label" for="reminder-period-select">Периодичность</label>
              <div class="select-wrapper">
                <select id="reminder-period-select" class="form-select" bind:value={reminderPeriod}>
                  <option value="Через 14 дней">Через 14 дней</option>
                  <option value="Через 7 дней">Через 7 дней</option>
                  <option value="Завтра">Завтра</option>
                </select>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Entry Detail View Panel -->
  {#if showDetail && selectedEntry}
    <EntryDetail
      show={showDetail}
      entryId={selectedEntry.id}
      categoryName={selectedEntry.categoryName}
      categoryIcon={selectedEntry.categoryIcon}
      categoryTheme={selectedEntry.categoryTheme}
      time={selectedEntry.time}
      content={selectedEntry.content}
      images={selectedEntry.images}
      tags={selectedEntry.tags}
      onClose={() => showDetail = false}
      onDelete={handleDeleteEntry}
      onSave={handleSaveEditedEntry}
    />
  {/if}
  <!-- Modal: Setup PIN -->
  {#if showSetPinModal}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }} onclick={() => showSetPinModal = false} onkeydown={() => {}} role="presentation">
      <div class="bottom-sheet" transition:slide={{ duration: 300 }} onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="presentation">
        <div class="bottom-sheet-handle"></div>
        <header class="sheet-header">
          <button type="button" class="text-btn" onclick={() => showSetPinModal = false}>Отмена</button>
          <h2>Установить PIN-код</h2>
          <button type="button" class="text-btn primary" onclick={handleSetupNewPin}>Готово</button>
        </header>
        <div class="sheet-form">
          <div class="form-group">
            <label class="form-label" for="pin-input">Введите 4-значный PIN</label>
            <input
              id="pin-input"
              type="password"
              maxlength="4"
              pattern="[0-9]*"
              inputmode="numeric"
              class="form-input"
              style="text-align: center; font-size: 1.5rem; letter-spacing: 12px;"
              placeholder="••••"
              bind:value={tempPinSetup}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Modal: Password for Backup -->
  {#if showBackupPasswordModal}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }} onclick={() => showBackupPasswordModal = false} onkeydown={() => {}} role="presentation">
      <div class="bottom-sheet" transition:slide={{ duration: 300 }} onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="presentation">
        <div class="bottom-sheet-handle"></div>
        <header class="sheet-header">
          <button type="button" class="text-btn" onclick={() => showBackupPasswordModal = false}>Отмена</button>
          <h2>Пароль для архива</h2>
          <button type="button" class="text-btn primary" onclick={handleExportBackup}>Экспорт</button>
        </header>
        <div class="sheet-form">
          <div class="form-group">
            <label class="form-label" for="backup-pw-input">Задайте пароль для шифрования архива</label>
            <input
              id="backup-pw-input"
              type="password"
              class="form-input"
              placeholder="Пароль бэкапа"
              bind:value={backupPasswordInput}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Modal: Password for Import -->
  {#if showImportPasswordModal}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }} onclick={() => showImportPasswordModal = false} onkeydown={() => {}} role="presentation">
      <div class="bottom-sheet" transition:slide={{ duration: 300 }} onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="presentation">
        <div class="bottom-sheet-handle"></div>
        <header class="sheet-header">
          <button type="button" class="text-btn" onclick={() => showImportPasswordModal = false}>Отмена</button>
          <h2>Пароль импорта</h2>
          <button type="button" class="text-btn primary" onclick={handleImportBackup}>Импорт</button>
        </header>
        <div class="sheet-form">
          <div class="form-group">
            <label class="form-label" for="import-pw-input">Введите пароль для расшифровки бэкапа</label>
            <input
              id="import-pw-input"
              type="password"
              class="form-input"
              placeholder="Пароль расшифровки"
              bind:value={importPasswordInput}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Pin Lock Screen Overlay -->
  {#if isAppLocked && !isSplashing}
    <PinLockScreen onVerify={handleVerifyPin} onSuccess={() => isAppLocked = false} />
  {/if}
  <!-- Onboarding Screen Overlay -->
  {#if showOnboarding && !isSplashing}
    <OnboardingScreen
      onComplete={handleOnboardingComplete}
      onSeedDemo={handleSeedDemoData}
      onSetPin={handleSetupNewPin}
    />
  {/if}
  <!-- Splash Screen Overlay -->
  {#if isSplashing}
    <div class="splash-screen" transition:fade={{ duration: 400 }}>
      <div class="splash-content">
        <Logo size={110} />
        <h1 class="splash-title">ХРОНИКИ</h1>
        <p class="splash-subtitle">Ваша личная история</p>
      </div>
    </div>
  {/if}
</main>

<style>
  .splash-screen {
    position: fixed;
    inset: 0;
    background: #090810;
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
    color: white;
  }

  .splash-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    text-align: center;
  }

  .splash-title {
    font-size: 2.2rem;
    font-weight: 800;
    letter-spacing: 3px;
    background: linear-gradient(135deg, #a855f7, #6366f1);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 2px;
  }

  .splash-subtitle {
    font-size: 1rem;
    color: var(--muted);
    font-weight: 500;
    letter-spacing: 1px;
  }

  .app-shell {
    display: flex;
    flex-direction: column;
    width: 100%;
    min-height: 100vh;
    align-items: center;
    background-color: var(--background);
    padding-bottom: 120px;
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

  .empty-feed {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 60px 20px;
    text-align: center;
    color: var(--muted);
  }

  .empty-icon {
    font-size: 2.5rem;
    margin-bottom: 12px;
  }

  .empty-feed p {
    font-size: 0.9rem;
    line-height: 1.4;
  }


  .section-title {
    font-size: 1.1rem;
    font-weight: 700;
    margin-bottom: 16px;
    color: var(--text);
  }

  /* Objects Grid wrapper (layout only) */
  .objects-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 8px;
  }


  .reminders-section {
    width: 100%;
    max-width: 480px;
    padding: 16px 20px;
  }

  .reminders-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .reminder-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    background: var(--surface-opaque);
    border-radius: var(--radius-lg);
    padding: 16px;
    box-shadow: var(--card-shadow);
    border-left: 4px solid var(--primary-purple);
    transition: opacity 0.2s;
  }

  .reminder-card.completed {
    border-left-color: #a3d9a5;
    opacity: 0.65;
  }

  .reminder-card.overdue {
    border-left-color: #e05c5c;
  }

  .reminder-icon {
    font-size: 1.4rem;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .reminder-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .reminder-title {
    font-size: 0.92rem;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .reminder-meta {
    font-size: 0.78rem;
    color: var(--muted);
  }

  .reminder-actions {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex-shrink: 0;
  }

  .reminder-btn {
    border: none;
    border-radius: var(--radius-sm);
    padding: 6px 12px;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.15s, opacity 0.15s;
  }

  .reminder-btn:hover {
    transform: scale(1.04);
  }

  .reminder-btn.done {
    background: var(--primary-purple);
    color: #fff;
  }

  .reminder-btn.snooze {
    background: rgba(96, 37, 255, 0.08);
    color: var(--primary-purple);
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

  /* Modal bottom sheet container */
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

  /* Bottom sheet styling */
  .bottom-sheet {
    background: var(--background);
    width: 100%;
    max-width: 480px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    max-height: 88vh;
    overflow-y: auto;
    box-shadow: 0 -12px 48px rgba(0, 0, 0, 0.15);
    padding-bottom: 30px;
  }

  .bottom-sheet-handle {
    width: 36px;
    height: 5px;
    background-color: var(--light-gray);
    border-radius: 3px;
    margin: 8px auto 0;
  }

  .sheet-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px 20px;
    border-bottom: 1px solid rgba(0,0,0,0.03);
    position: sticky;
    top: 0;
    background: var(--background);
    z-index: 2;
  }

  .sheet-header h2 {
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

  .sheet-form {
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
  .form-input,
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
  .form-input:focus,
  .form-textarea:focus {
    border-color: var(--primary-purple);
  }

  .form-textarea {
    min-height: 100px;
    resize: none;
  }

  .select-wrapper {
    position: relative;
  }

  /* Photo upload row & horizontal previews */
  .photo-upload-row {
    display: flex;
    gap: 12px;
    align-items: center;
    overflow-x: auto;
    padding-bottom: 8px;
    scrollbar-width: none;
  }
  
  .photo-upload-row::-webkit-scrollbar {
    display: none;
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
    background: none;
    opacity: 0.6;
    transition: opacity 0.2s ease;
    flex-shrink: 0;
  }

  .photo-uploader:hover {
    opacity: 1;
  }

  .uploader-icon {
    width: 24px;
    height: 24px;
    color: var(--muted);
  }

  .previews-container {
    display: flex;
    gap: 10px;
  }

  .preview-item {
    position: relative;
    width: 72px;
    height: 72px;
    border-radius: var(--radius-md);
    overflow: hidden;
    flex-shrink: 0;
  }

  .preview-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .remove-photo-btn {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: rgba(24, 24, 27, 0.6);
    color: white;
    border: none;
    font-size: 10px;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    transition: background-color 0.2s;
  }

  .remove-photo-btn:hover {
    background: rgba(24, 24, 27, 0.9);
  }

  .tags-picker {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }

  .tags-picker .tag-pill {
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
    border: none;
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

  /* Search & Filter Panel */
  .search-filters-panel {
    background: var(--surface-opaque);
    border-radius: var(--radius-lg);
    padding: 16px;
    box-shadow: var(--card-shadow);
    margin: 8px 4px 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .search-input-wrapper {
    position: relative;
    width: 100%;
  }
  .search-text-input {
    width: 100%;
    border: 1.5px solid var(--light-gray);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    font-size: 0.9rem;
    font-family: var(--font-main);
    outline: none;
    transition: border-color 0.2s;
  }
  .search-text-input:focus {
    border-color: var(--primary-purple);
  }
  .filter-dropdowns-row, .filter-dates-row {
    display: flex;
    gap: 12px;
  }
  .filter-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .filter-label {
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .filter-select, .filter-date-input {
    width: 100%;
    border: 1.5px solid var(--light-gray);
    border-radius: 10px;
    padding: 8px;
    font-size: 0.85rem;
    font-family: var(--font-main);
    background: white;
    outline: none;
  }
  .clear-filters-btn {
    border: none;
    background: rgba(96, 37, 255, 0.05);
    color: var(--primary-purple);
    border-radius: 10px;
    padding: 8px;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  .clear-filters-btn:hover {
    background: rgba(96, 37, 255, 0.1);
  }


  .settings-action-btn {
    border: none;
    background: none;
    color: var(--primary-purple);
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    text-align: left;
    width: 100%;
    padding: 12px 0;
    display: flex;
    align-items: center;
    gap: 8px;
    border-bottom: 1px solid var(--light-gray);
  }
  .settings-action-btn:last-child {
    border-bottom: none;
  }
</style>
