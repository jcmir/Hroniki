<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import TimelineCard from './TimelineCard.svelte';
  import { getCategoryIcon } from '../utils/categoryIcons';
  import type { ChronicleObject } from '../types/ChronicleObject';
  import type { ObjectStats } from '../types/ObjectStats';
  import type { Entry } from '../types/Entry';
  import type { Category } from '../types/Category';

  interface Props {
    object: ChronicleObject;
    stats: ObjectStats | null;
    entries: Entry[];
    categories: Category[];
    onBack: () => void;
    onCardClick: (id: string) => void;
  }

  let { object, stats, entries, categories, onBack, onCardClick }: Props = $props();

  function pluralRu(n: number, one: string, few: string, many: string): string {
    const mod10 = n % 10, mod100 = n % 100;
    if (mod10 === 1 && mod100 !== 11) return one;
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 10 || mod100 >= 20)) return few;
    return many;
  }

  function calcAge(createdAt: string | undefined): string {
    if (!createdAt) return '';
    const ms = Date.now() - new Date(createdAt).getTime();
    const totalDays = Math.floor(ms / 86_400_000);
    if (totalDays < 1) return 'только что';
    const years = Math.floor(totalDays / 365);
    const months = Math.floor((totalDays % 365) / 30);
    const days = totalDays % 30;
    const parts: string[] = [];
    if (years > 0) parts.push(`${years} ${pluralRu(years, 'год', 'года', 'лет')}`);
    if (months > 0) parts.push(`${months} ${pluralRu(months, 'месяц', 'месяца', 'месяцев')}`);
    if (years === 0 && months === 0 && days > 0) parts.push(`${days} ${pluralRu(days, 'день', 'дня', 'дней')}`);
    return parts.join(' ');
  }

  // Group entries by year of occurred_at, sorted ascending
  function groupByYear(list: Entry[]): [number, Entry[]][] {
    const map: Record<number, Entry[]> = {};
    for (const e of list) {
      const year = new Date(e.occurred_at ?? e.created_at).getFullYear();
      (map[year] ??= []).push(e);
    }
    return Object.entries(map)
      .map(([y, es]) => [Number(y), es] as [number, Entry[]])
      .sort((a, b) => a[0] - b[0]);
  }

  function formatDayMonth(dateStr: string): string {
    return new Date(dateStr).toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' });
  }

  const categoryName = $derived(categories.find((c: Category) => c.id === object.category_id)?.name ?? 'Объект');
  const icon = $derived(getCategoryIcon(categoryName));
  const age = $derived(calcAge(object.created_at));
  const yearGroups = $derived(groupByYear(entries));

  // First ever entry by occurred_at
  const firstEntry = $derived(
    entries.length > 0
      ? [...entries].sort((a, b) => new Date(a.occurred_at ?? a.created_at).getTime() - new Date(b.occurred_at ?? b.created_at).getTime())[0]
      : null
  );
</script>

<div class="chronicle-view" in:fade={{ duration: 200 }}>
  <!-- Back link -->
  <button type="button" class="back-link-btn" onclick={onBack}>
    ← К объектам
  </button>

  <!-- Hero -->
  <div class="object-hero" in:fly={{ y: 12, duration: 300, delay: 60 }}>
    <div class="object-avatar-wrapper">
      <span class="object-avatar-emoji">{icon}</span>
    </div>
    <h2 class="object-title">{object.name}</h2>
    <span class="object-category-badge">{icon} {categoryName}</span>
    {#if age}
      <span class="object-age">В архиве: {age}</span>
    {/if}
    {#if stats && stats.last_event_title}
      <div class="object-last-event">
        <span class="last-event-label">Последнее:</span>
        <span class="last-event-value">{stats.last_event_title}</span>
        {#if stats.last_event_date}
          <span class="last-event-time">({formatDayMonth(stats.last_event_date)})</span>
        {/if}
      </div>
    {/if}
    {#if object.description}
      <p class="object-description">{object.description}</p>
    {/if}
  </div>

  <!-- Stats row -->
  {#if stats}
    <div class="stats-row" in:fly={{ y: 8, duration: 280, delay: 120 }}>
      <div class="stat-card">
        <span class="stat-value">{stats.total_entries}</span>
        <span class="stat-label">Событий</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{stats.total_photos}</span>
        <span class="stat-label">Фото</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">
          {#if stats.next_reminder_date}
            {new Date(stats.next_reminder_date).toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })}
          {:else}
            —
          {/if}
        </span>
        <span class="stat-label">Напомн.</span>
      </div>
    </div>
  {/if}

  <!-- Timeline -->
  <div class="chronicle-timeline" in:fly={{ y: 8, duration: 280, delay: 180 }}>
    {#if entries.length === 0}
      <div class="chronicle-empty">
        <span class="chronicle-empty-icon">{icon}</span>
        <p>Пока нет событий для этого объекта.</p>
        <p class="chronicle-empty-hint">Добавьте первую запись из ленты.</p>
      </div>
    {:else}
      <!-- First event callout -->
      {#if firstEntry}
        <div class="first-event-callout">
          <span class="first-event-dot">🌱</span>
          <div class="first-event-body">
            <span class="first-event-label">Первое событие</span>
            <span class="first-event-title">{firstEntry.title || firstEntry.content?.split('\n')[0] || 'Запись'}</span>
            <span class="first-event-date">{formatDayMonth(firstEntry.occurred_at ?? firstEntry.created_at)}</span>
          </div>
        </div>
      {/if}

      <!-- Continuous Timeline Container -->
      <div class="chronicle-timeline-container">
        <div class="timeline-axis"></div>

        {#each yearGroups as [year, yearEntries]}
          <div class="year-header">
            <div class="year-left">
              <div class="year-circle"></div>
            </div>
            <span class="year-num">{year}</span>
          </div>

          <div class="year-entries">
            {#each yearEntries as item, i (item.id)}
              <TimelineCard {...item} index={i} onClick={onCardClick} />
            {/each}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .chronicle-view {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .back-link-btn {
    border: none;
    background: none;
    color: var(--primary-purple);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    text-align: left;
    margin-bottom: 8px;
    padding: 4px 0;
    align-self: flex-start;
    font-family: inherit;
  }

  /* Hero */
  .object-hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px 0 24px;
    text-align: center;
    gap: 6px;
  }

  .object-avatar-wrapper {
    width: 80px;
    height: 80px;
    border-radius: 22px;
    background: var(--light-gray);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 6px 20px rgba(96, 37, 255, 0.12);
    margin-bottom: 10px;
  }

  .object-avatar-emoji { font-size: 2.6rem; line-height: 1; }

  .object-title {
    font-size: 1.45rem;
    font-weight: 700;
    margin: 0;
    color: var(--text);
  }

  .object-category-badge {
    padding: 4px 12px;
    border-radius: 12px;
    background: var(--accent-green-bg);
    color: var(--accent-green);
    font-size: 0.78rem;
    font-weight: 600;
  }

  .object-age {
    font-size: 0.82rem;
    color: var(--muted);
    font-weight: 500;
  }

  .object-last-event {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 0.8rem;
    background: rgba(96, 37, 255, 0.05);
    color: var(--primary-purple);
    padding: 4px 10px;
    border-radius: 8px;
    margin-top: 4px;
    margin-bottom: 2px;
    font-weight: 500;
  }

  .last-event-label {
    opacity: 0.7;
    font-weight: 600;
  }

  .last-event-value {
    font-weight: 600;
    max-width: 140px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .last-event-time {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  .object-description {
    font-size: 0.88rem;
    color: var(--muted);
    max-width: 280px;
    line-height: 1.4;
    margin: 2px 0 0;
  }

  /* Stats */
  .stats-row {
    display: flex;
    gap: 10px;
    margin-bottom: 24px;
  }

  .stat-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    background: var(--surface-opaque);
    border-radius: var(--radius-lg);
    padding: 14px 8px;
    box-shadow: var(--card-shadow);
    gap: 2px;
  }

  .stat-value {
    font-size: 1.3rem;
    font-weight: 700;
    color: var(--primary-purple);
  }

  .stat-label {
    font-size: 0.68rem;
    font-weight: 600;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  /* Timeline */
  .chronicle-timeline {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  /* First event callout */
  .first-event-callout {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    background: linear-gradient(135deg, rgba(96, 37, 255, 0.05), rgba(96, 37, 255, 0.02));
    border: 1px solid rgba(96, 37, 255, 0.1);
    border-radius: var(--radius-lg);
    padding: 14px 16px;
    margin-bottom: 24px;
  }

  .first-event-dot {
    font-size: 1.4rem;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .first-event-body {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .first-event-label {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--primary-purple);
    opacity: 0.7;
  }

  .first-event-title {
    font-size: 0.92rem;
    font-weight: 600;
    color: var(--text);
  }

  .first-event-date {
    font-size: 0.75rem;
    color: var(--muted);
  }

  /* Continuous Timeline axis & years styling */
  .chronicle-timeline-container {
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .timeline-axis {
    position: absolute;
    left: 15px;
    top: 10px;
    bottom: 30px;
    width: 2px;
    background: linear-gradient(to bottom, var(--light-gray), rgba(0,0,0,0.01));
    z-index: 1;
  }

  .year-header {
    display: flex;
    align-items: center;
    margin: 24px 0 16px;
    position: relative;
    z-index: 2;
  }

  .year-left {
    width: 32px;
    display: flex;
    justify-content: center;
    flex-shrink: 0;
  }

  .year-circle {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background-color: var(--primary-purple);
    box-shadow: 0 0 0 5px var(--background);
  }

  .year-num {
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text);
    margin-left: 12px;
    letter-spacing: -0.02em;
  }

  .year-entries {
    display: flex;
    flex-direction: column;
    gap: 0;
    position: relative;
    z-index: 2;
  }

  /* Empty state */
  .chronicle-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 48px 24px;
    text-align: center;
    gap: 8px;
  }

  .chronicle-empty-icon { font-size: 2.5rem; }

  .chronicle-empty p {
    font-size: 0.9rem;
    color: var(--text);
    font-weight: 500;
    margin: 0;
  }

  .chronicle-empty-hint {
    font-size: 0.82rem !important;
    color: var(--muted) !important;
    font-weight: 400 !important;
  }
</style>

