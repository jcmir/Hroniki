<script lang="ts">
  import { fly } from 'svelte/transition';
  import CategoryIcon from './CategoryIcon.svelte';
  import type { ChronicleObject } from '../types';
  import { formatAge, pluralRu } from '../utils/dateHelpers';

  interface Props {
    object: ChronicleObject;
    categoryName?: string;
    entryCount?: number;
    index?: number;
    onSelect: (obj: ChronicleObject) => void;
  }

  let { object, categoryName = 'Категория', entryCount = 0, index = 0, onSelect }: Props = $props();

  const age = $derived(formatAge(object.created_at, true));
</script>

<button
  type="button"
  class="object-card"
  onclick={() => onSelect(object)}
  in:fly={{ y: 16, duration: 280, delay: index * 55 }}
>
  <div class="object-card-icon">
    <CategoryIcon {categoryName} size={22} />
  </div>

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
    background: var(--light-gray);
    color: var(--primary-purple);
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
