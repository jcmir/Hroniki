<script lang="ts">
  import { fly } from 'svelte/transition';
  import { getCategoryIcon } from '../utils/categoryIcons';
  import type { ChronicleObject } from '../types/ChronicleObject';

  interface Props {
    object: ChronicleObject;
    categoryName?: string;
    entryCount?: number;
    index?: number;
    onSelect: (obj: ChronicleObject) => void;
  }

  let { object, categoryName = 'Категория', entryCount = 0, index = 0, onSelect }: Props = $props();

  const icon = $derived(getCategoryIcon(categoryName));

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
    if (totalDays < 1) return 'сегодня';
    const years = Math.floor(totalDays / 365);
    const months = Math.floor((totalDays % 365) / 30);
    const parts: string[] = [];
    if (years > 0) parts.push(`${years} ${pluralRu(years, 'год', 'года', 'лет')}`);
    if (months > 0) parts.push(`${months} мес`);
    if (years === 0 && months === 0) parts.push(`${totalDays % 30} дн`);
    return parts.join(' ');
  }

  const age = $derived(calcAge(object.created_at));
</script>

<button
  type="button"
  class="object-card"
  onclick={() => onSelect(object)}
  in:fly={{ y: 16, duration: 280, delay: index * 55 }}
>
  <div class="object-card-icon">{icon}</div>

  <div class="object-card-body">
    <span class="object-card-name">{object.name}</span>
    <span class="object-card-sub">
      {#if age}{age}{/if}{#if age && entryCount > 0} · {/if}{#if entryCount > 0}{entryCount}&nbsp;{pluralRu(entryCount, 'событие', 'события', 'событий')}{/if}{#if !age && !entryCount}{object.description || 'Нет записей'}{/if}
    </span>
  </div>

  <svg class="object-card-arrow" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
    <path d="M6 3l5 5-5 5"/>
  </svg>
</button>

<style>
  .object-card {
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--surface-opaque);
    border-radius: var(--radius-lg);
    padding: 16px;
    border: none;
    box-shadow: var(--card-shadow);
    text-align: left;
    cursor: pointer;
    transition: transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.2s;
    width: 100%;
  }
  .object-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 14px 36px rgba(96, 37, 255, 0.12);
  }
  .object-card:active { transform: scale(0.98); }
  .object-card-icon {
    font-size: 1.6rem;
    background: var(--light-gray);
    width: 48px;
    height: 48px;
    border-radius: 14px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
  }
  .object-card-body {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    gap: 3px;
  }
  .object-card-name {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .object-card-sub {
    font-size: 0.78rem;
    color: var(--muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .object-card-arrow {
    width: 16px;
    height: 16px;
    color: var(--muted);
    flex-shrink: 0;
    opacity: 0.5;
  }
</style>
