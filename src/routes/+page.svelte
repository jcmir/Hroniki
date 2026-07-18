<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import '../app.css';
  import TimelineCard from '$lib/components/TimelineCard.svelte';
  import BottomNav from '$lib/components/BottomNav.svelte';
  import Card from '$lib/components/Card.svelte';
  import EntryDetail from '$lib/components/EntryDetail.svelte';

  // State variables from DB
  let categories = $state<any[]>([]);
  let objects = $state<any[]>([]);
  let entries = $state<any[]>([]);

  // Browser-only mockup fallback database
  let mockCategories = [
    { id: '1', name: 'Сад' },
    { id: '2', name: 'Здоровье' },
    { id: '3', name: 'Авто' }
  ];
  let mockObjects = [
    { id: 'obj1', category_id: '1', name: 'Яблоня', description: 'Дерево в саду' }
  ];
  let mockEntries = $state<any[]>([
    {
      id: 'e1',
      object_id: 'obj1',
      occurred_at: new Date(Date.now() - 3600000).toISOString(),
      title: 'Опрыскала томаты Фитовермом от тли',
      description: 'Листья выглядят намного лучше.'
    },
    {
      id: 'e2',
      object_id: 'obj1',
      occurred_at: new Date(Date.now() - 7200000).toISOString(),
      title: 'Сдал анализы',
      description: 'Самочувствие стало лучше. Продолжаю бег по утрам.'
    }
  ]);
  
  // Mapping of mock entry IDs to mock photos
  let mockPhotosMap = new Map<string, any[]>([
    ['e1', [{ id: 'p1', entry_id: 'e1', path: '/garden_tomatoes.png', thumbnail: '/garden_tomatoes.png' }]],
    ['e2', [{ id: 'p2', entry_id: 'e2', path: '/running_shoes.png', thumbnail: '/running_shoes.png' }]]
  ]);

  // Navigation state
  let activeTab = $state<'feed' | 'objects' | 'reminders' | 'settings'>('feed');
  let showAddModal = $state(false);
  let showDetail = $state(false);
  let selectedEntry = $state<any>(null);

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

  onMount(async () => {
    await refreshData();
  });

  // Safe wrapper around Tauri invoke to run gracefully in standard browsers
  async function safeInvoke<T>(cmd: string, args?: any): Promise<T> {
    const isTauri = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
    if (isTauri) {
      return await invoke<T>(cmd, args);
    } else {
      console.warn(`[Tauri Mock] Invoke '${cmd}'`, args);
      if (cmd === 'get_categories') {
        return mockCategories as T;
      }
      if (cmd === 'get_objects') {
        return mockObjects as T;
      }
      if (cmd === 'get_entries') {
        return mockEntries as T;
      }
      if (cmd === 'get_entry_photos') {
        const customPhotos = mockPhotosMap.get(args.entryId) || [];
        return customPhotos as T;
      }
      if (cmd === 'create_object') {
        const newObj = {
          id: `obj-${Math.random()}`,
          category_id: args.categoryId,
          name: args.name,
          description: args.description || null
        };
        mockObjects.push(newObj);
        return newObj.id as T;
      }
      if (cmd === 'create_entry') {
        const newEnt = {
          id: `ent-${Math.random()}`,
          object_id: args.objectId,
          occurred_at: new Date().toISOString(),
          title: args.title,
          description: args.description || null
        };
        mockEntries.unshift(newEnt);
        
        // Save mock photos
        if (args.imageFilenames && args.imageFilenames.length > 0) {
          const mockPhotosList = args.imageFilenames.map((f: string) => ({
            id: `p-${Math.random()}`,
            entry_id: newEnt.id,
            path: f,
            thumbnail: f
          }));
          mockPhotosMap.set(newEnt.id, mockPhotosList);
        }
        
        return newEnt.id as T;
      }
      if (cmd === 'select_images') {
        // Return a mock local image file path for browser environment demo
        return ['/garden_tomatoes.png'] as T;
      }
      if (cmd === 'save_media') {
        // Return the source path directly as the filename in web fallback mode
        return args.sourcePath as T;
      }
      if (cmd === 'delete_entry') {
        mockEntries = mockEntries.filter(e => e.id !== args.entryId);
        mockPhotosMap.delete(args.entryId);
        return null as T;
      }
      if (cmd === 'update_entry') {
        const ent = mockEntries.find(e => e.id === args.entryId);
        if (ent) {
          ent.title = args.title;
          ent.description = args.description;
        }
        return null as T;
      }
      return null as T;
    }
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

      const rawEntries = await safeInvoke<any[]>('get_entries');
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

      await safeInvoke('create_entry', {
        objectId: selectedObject,
        title: newEntryTitle,
        description: newEntryDesc || null,
        imageFilenames: savedFilenames
      });

      showAddModal = false;
      newEntryTitle = '';
      newEntryDesc = '';
      selectedPhotoPaths = [];
      await refreshData();
    } catch (e) {
      console.error('Failed to save entry:', e);
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
      <span class="logo-spark">✨</span>
      <h1>ХРОНИКИ</h1>
    </div>
    <div class="header-actions">
      <button type="button" class="icon-btn" aria-label="Search">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
      </button>
      <button type="button" class="icon-btn" aria-label="Filters">
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

  <!-- Content view -->
  <div class="content-viewport">
    {#if activeTab === 'feed'}
      <!-- Feed Tab: Date Selector & Timeline -->
      <section class="feed-section" in:fade={{ duration: 200 }}>
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
              <span class="empty-icon">📖</span>
              <p>Хроника пуста. Нажмите на плюс внизу, чтобы добавить событие.</p>
            </div>
          {:else}
            {#each entries as item (item.id)}
              <TimelineCard {...item} onClick={handleCardClick} />
            {/each}
          {/if}
        </div>
      </section>
    {:else if activeTab === 'objects'}
      <!-- Objects Tab: Object details view -->
      <section class="objects-section" in:fade={{ duration: 200 }}>
        {#if objects.length > 0}
          <div class="object-hero">
            <div class="object-avatar-wrapper">
              <img src="/garden_tomatoes.png" alt="Яблоня" class="object-avatar" />
            </div>
            <h2 class="object-title">{objects[0].name}</h2>
            <span class="object-category-badge">🌱 Сад</span>
            <span class="object-meta">Успешно подключен к SQLite</span>
          </div>

          <!-- Statistics -->
          <div class="stats-row">
            <div class="stat-card">
              <span class="stat-value">{entries.length}</span>
              <span class="stat-label">Записей</span>
            </div>
            <div class="stat-card">
              <span class="stat-value">{entries.filter(e => e.images.length > 0).length}</span>
              <span class="stat-label">Фото</span>
            </div>
            <div class="stat-card">
              <span class="stat-value">0</span>
              <span class="stat-label">Напоминаний</span>
            </div>
          </div>

          <h3 class="section-title">История ухода</h3>
          <div class="timeline-container">
            <div class="timeline-axis"></div>
            {#if entries.length === 0}
              <p class="empty-label">Нет записей для этого объекта.</p>
            {:else}
              {#each entries as item (item.id)}
                <TimelineCard {...item} onClick={handleCardClick} />
              {/each}
            {/if}
          </div>
        {:else}
          <div class="empty-tab">
            <h3>Нет объектов</h3>
          </div>
        {/if}
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
              <span class="setting-name">База данных</span>
              <span class="setting-value">SQLite (активна)</span>
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
</main>

<style>
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

  .empty-label {
    text-align: center;
    color: var(--muted);
    padding: 20px;
    font-size: 0.9rem;
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
</style>
