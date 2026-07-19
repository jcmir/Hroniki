<script lang="ts">
  import Card from './Card.svelte';
  import { fly } from 'svelte/transition';

  interface Props {
    id: string;
    categoryName: string;
    categoryIcon: string;
    categoryTheme: 'green' | 'blue' | 'pink' | 'orange' | 'purple';
    time: string;
    content: string;
    images?: string[];
    tags?: string[];
    reminderText?: string;
    index?: number;
    onClick?: (id: string) => void;
  }

  let {
    id,
    categoryName,
    categoryIcon,
    categoryTheme,
    time,
    content,
    images = [],
    tags = [],
    reminderText = '',
    index = 0,
    onClick
  }: Props = $props();

  const themeClasses = {
    green: 'theme-green',
    blue: 'theme-blue',
    pink: 'theme-pink',
    orange: 'theme-orange',
    purple: 'theme-purple'
  };
</script>

<div
  class="timeline-item {themeClasses[categoryTheme]}"
  in:fly={{ y: 14, duration: 300, delay: index * 55 }}
>
  <!-- Left Side: Date / Timeline Line -->
  <div class="timeline-left">
    <div class="timeline-dot"></div>
  </div>

  <!-- Right Side: Content Card -->
  <div
    class="timeline-content"
    onclick={() => onClick && onClick(id)}
    onkeydown={(e) => e.key === 'Enter' && onClick && onClick(id)}
    role="button"
    tabindex="0"
    style="cursor: pointer; outline: none;"
  >
    <Card>
      <!-- Header with Category -->
      <div class="card-header">
        <span class="category-badge">
          <span class="category-icon">{categoryIcon}</span>
          <span class="category-name">{categoryName}</span>
        </span>
      </div>

      <!-- Description Text -->
      <p class="card-text">{content}</p>

      <!-- Photos Grid -->
      {#if images.length > 0}
        <div class="photo-grid cols-{Math.min(images.length, 3)}">
          {#each images as img}
            <div class="photo-wrapper">
              <img src={img} alt="Chronicle media" class="photo" />
            </div>
          {/each}
        </div>
      {/if}

      <!-- Tags List -->
      {#if tags.length > 0}
        <div class="tags-list">
          {#each tags as tag}
            <span class="tag-pill">#{tag}</span>
          {/each}
        </div>
      {/if}

      <!-- Footer Info -->
      <div class="card-footer">
        <span class="time-label">создано {time}</span>
        <div class="meta-row">
          {#if images.length > 0}
            <span class="meta-badge">📷 {images.length} фото</span>
          {/if}
          {#if reminderText}
            <span class="meta-badge reminder">🔔 {reminderText}</span>
          {/if}
        </div>
      </div>
    </Card>
  </div>
</div>

<style>
  .timeline-item {
    display: flex;
    position: relative;
    width: 100%;
  }

  .timeline-left {
    width: 32px;
    position: relative;
    display: flex;
    justify-content: center;
    flex-shrink: 0;
  }

  /* Timeline Dot */
  .timeline-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background-color: var(--badge-color);
    z-index: 2;
    margin-top: 30px;
    box-shadow: 0 0 0 4px var(--background);
    transition: transform 0.2s ease;
  }

  .timeline-item:hover .timeline-dot {
    transform: scale(1.3);
  }

  /* Themes with custom backgrounds and colors */
  .theme-green {
    --badge-bg: var(--accent-green-bg);
    --badge-color: var(--accent-green);
    --card-bg-tint: #f2faf4;
  }
  .theme-blue {
    --badge-bg: var(--accent-blue-bg);
    --badge-color: var(--accent-blue);
    --card-bg-tint: #f2f7fe;
  }
  .theme-pink {
    --badge-bg: var(--accent-pink-bg);
    --badge-color: var(--accent-pink);
    --card-bg-tint: #fdf5f7;
  }
  .theme-orange {
    --badge-bg: var(--accent-orange-bg);
    --badge-color: var(--accent-orange);
    --card-bg-tint: #fef8f2;
  }
  .theme-purple {
    --badge-bg: rgba(96, 37, 255, 0.08);
    --badge-color: var(--primary-purple);
    --card-bg-tint: #f8f6ff;
  }

  .timeline-content {
    flex-grow: 1;
    padding-left: 12px;
  }

  /* Style the global Card component wrapper from parent scope */
  .timeline-content :global(.card) {
    background-color: var(--card-bg-tint);
    border-left: 4px solid var(--badge-color);
    padding-left: 20px;
  }

  .card-header {
    margin-bottom: 12px;
  }

  .category-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 12px;
    background-color: var(--badge-bg);
    color: var(--badge-color);
    font-size: 0.85rem;
    font-weight: 600;
  }

  .category-icon {
    font-size: 0.95rem;
  }

  .card-text {
    font-size: 0.98rem;
    line-height: 1.5;
    color: var(--text);
    margin-bottom: 16px;
  }

  /* Photo Grid popped out spanning full card width */
  .photo-grid {
    display: grid;
    gap: 8px;
    margin-bottom: 16px;
    margin-left: -20px;
    margin-right: -24px;
    width: calc(100% + 44px);
    overflow: hidden;
  }

  .cols-1 { grid-template-columns: 1fr; }
  .cols-2 { grid-template-columns: 1fr 1fr; }
  .cols-3 { grid-template-columns: 1.2fr 1fr 1fr; }

  .photo-wrapper {
    position: relative;
    aspect-ratio: 16/10;
    background-color: var(--light-gray);
  }

  .photo-grid.cols-3 .photo-wrapper:first-child {
    aspect-ratio: auto;
    height: 100%;
  }

  .photo {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.5s ease;
  }

  .photo-wrapper:hover .photo {
    transform: scale(1.05);
  }

  /* Tags */
  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 18px;
  }

  .tag-pill {
    padding: 4px 10px;
    background-color: rgba(255, 255, 255, 0.6);
    color: var(--muted);
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 500;
    border: 1px solid rgba(0,0,0,0.03);
  }

  /* Footer */
  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-top: 1px solid rgba(0,0,0,0.03);
    padding-top: 14px;
  }

  .time-label {
    font-size: 0.8rem;
    color: var(--muted);
  }

  .meta-row {
    display: flex;
    gap: 8px;
  }

  .meta-badge {
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--muted);
    background-color: rgba(255, 255, 255, 0.7);
    padding: 4px 8px;
    border-radius: 6px;
    border: 1px solid rgba(0,0,0,0.02);
  }

  .meta-badge.reminder {
    color: var(--primary-purple);
    background-color: rgba(96, 37, 255, 0.05);
  }
</style>
