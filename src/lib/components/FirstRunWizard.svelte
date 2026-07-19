<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import GlassCard from '$lib/design/GlassCard.svelte';

  const dispatch = createEventDispatcher();

  let step = 0;
  let selectedCategories: string[] = [];
  let wantDemo = false;
  let isSeeding = false;

  const categoryOptions = [
    { id: 'vehicle', name: 'Автомобиль', icon: '🚗', description: 'ТО, страховка, ремонт' },
    { id: 'home', name: 'Дом / Дача', icon: '🏡', description: 'Ремонт, сад, хозяйство' },
    { id: 'travel', name: 'Путешествия', icon: '✈️', description: 'Поездки и маршруты' },
    { id: 'family', name: 'Близкие', icon: '👨‍👩‍👧', description: 'Дети, родители, питомцы' },
  ];

  function toggleCategory(id: string) {
    if (selectedCategories.includes(id)) {
      selectedCategories = selectedCategories.filter(c => c !== id);
    } else {
      selectedCategories = [...selectedCategories, id];
    }
  }

  async function handleSeed() {
    isSeeding = true;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('seed_demo_dataset');
    } catch {
      // Seed can fail if user already has data — fine to ignore
    } finally {
      isSeeding = false;
      dispatch('complete');
    }
  }

  function handleSkip() {
    dispatch('complete');
  }
</script>

<div class="wizard-overlay">
  {#if step === 0}
    <!-- Step 0: Brand Welcome -->
    <div class="wizard-card">
      <GlassCard hoverEffect={false}>
        <div class="welcome-content">
          <div class="brand-sparkle">✦</div>
          <h2 class="brand-title">ХРОНИКИ</h2>
          <p class="brand-tagline">Архив жизненных объектов</p>

          <p class="welcome-desc">
            Сохраняйте историю своих автомобилей, дома, путешествий и любимых людей.
          </p>

          <button class="primary-btn" on:click={() => (step = 1)}>
            Начать →
          </button>

          <button class="skip-link" on:click={handleSkip}>Пропустить</button>
        </div>
      </GlassCard>
    </div>
  {:else if step === 1}
    <!-- Step 1: Category Selection -->
    <div class="wizard-card">
      <GlassCard hoverEffect={false}>
        <h3 class="step-title">Что хотите сохранить?</h3>
        <p class="step-desc">Выберите области жизни, которые важны для вас.</p>

        <div class="category-grid">
          {#each categoryOptions as cat}
            <button
              class="cat-option"
              class:selected={selectedCategories.includes(cat.id)}
              on:click={() => toggleCategory(cat.id)}
            >
              <span class="cat-icon">{cat.icon}</span>
              <span class="cat-name">{cat.name}</span>
              <span class="cat-desc">{cat.description}</span>
            </button>
          {/each}
        </div>

        <div class="step-actions">
          <button class="primary-btn" on:click={() => (step = 2)}>Далее →</button>
          <button class="skip-link" on:click={handleSkip}>Пропустить</button>
        </div>
      </GlassCard>
    </div>
  {:else if step === 2}
    <!-- Step 2: Demo Data Offer -->
    <div class="wizard-card">
      <GlassCard hoverEffect={false}>
        <div class="demo-offer">
          <div class="demo-icon">📖</div>
          <h3>Загрузить примеры хроник?</h3>
          <p>
            Заполним приложение примерными объектами, чтобы сразу было понятно, как работает ХРОНИКИ.
          </p>

          <div class="demo-preview">
            <div class="demo-row">🚗 BMW X5 &mdash; Замена масла 5w30</div>
            <div class="demo-row">🏡 Дом в Завидово &mdash; Обработка сада</div>
          </div>

          {#if isSeeding}
            <div class="seeding-spinner">
              <div class="spinner"></div>
              <span>Создаём хроники...</span>
            </div>
          {:else}
            <div class="step-actions">
              <button class="primary-btn" on:click={handleSeed}>
                Да, загрузить примеры
              </button>
              <button class="skip-link" on:click={handleSkip}>
                Начну сам
              </button>
            </div>
          {/if}
        </div>
      </GlassCard>
    </div>
  {/if}
</div>

<style>
  .wizard-overlay {
    position: fixed;
    inset: 0;
    background-color: var(--bg-app);
    z-index: 200;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
  }

  .wizard-card {
    width: 100%;
    max-width: 420px;
    animation: fadeUp 0.35s cubic-bezier(0.16, 1, 0.3, 1) both;
  }

  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(24px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .welcome-content {
    text-align: center;
    padding: 0.5rem;
  }

  .brand-sparkle {
    font-size: 2.8rem;
    color: var(--accent-primary);
    margin-bottom: 0.5rem;
  }

  .brand-title {
    font-family: var(--font-heading);
    font-size: 2rem;
    font-weight: 700;
    color: var(--text-main);
    letter-spacing: 0.04em;
  }

  .brand-tagline {
    font-size: 0.9rem;
    color: var(--accent-primary);
    font-weight: 600;
    margin-bottom: 1.25rem;
  }

  .welcome-desc {
    font-size: 0.95rem;
    color: var(--text-muted);
    line-height: 1.5;
    margin-bottom: 2rem;
  }

  .step-title {
    font-family: var(--font-heading);
    font-size: 1.3rem;
    font-weight: 700;
    color: var(--text-main);
    margin-bottom: 0.4rem;
  }

  .step-desc {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-bottom: 1.25rem;
  }

  .category-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }

  .cat-option {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 1rem 0.5rem;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .cat-option.selected {
    border-color: var(--accent-primary);
    background-color: rgba(124, 58, 237, 0.06);
    box-shadow: 0 0 0 2px rgba(124, 58, 237, 0.15);
  }

  .cat-icon { font-size: 1.7rem; }
  .cat-name { font-size: 0.9rem; font-weight: 600; color: var(--text-main); }
  .cat-desc { font-size: 0.72rem; color: var(--text-muted); text-align: center; }

  .demo-offer {
    text-align: center;
  }

  .demo-icon { font-size: 2.5rem; margin-bottom: 0.75rem; }

  .demo-offer h3 {
    font-family: var(--font-heading);
    font-size: 1.2rem;
    color: var(--text-main);
    margin-bottom: 0.6rem;
  }

  .demo-offer p {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-bottom: 1rem;
    line-height: 1.5;
  }

  .demo-preview {
    background-color: rgba(23, 23, 23, 0.04);
    border-radius: var(--radius-md);
    padding: 0.75rem 1rem;
    margin-bottom: 1.5rem;
    text-align: left;
  }

  .demo-row {
    font-size: 0.85rem;
    color: var(--text-muted);
    padding: 0.3rem 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .demo-row:last-child { border-bottom: none; }

  .step-actions {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    align-items: center;
  }

  .primary-btn {
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink));
    border: none;
    color: #FFF;
    font-weight: 600;
    font-size: 1rem;
    padding: 0.85rem 2.5rem;
    border-radius: var(--radius-pill);
    box-shadow: 0 4px 16px rgba(124, 58, 237, 0.3);
    cursor: pointer;
    width: 100%;
  }

  .skip-link {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 0.875rem;
    cursor: pointer;
    text-decoration: underline;
  }

  .seeding-spinner {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    color: var(--text-muted);
    font-size: 0.9rem;
    margin-top: 1rem;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 3px solid var(--border-subtle);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
</style>
