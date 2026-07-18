<script lang="ts">
  import Card from './Card.svelte';

  interface Props {
    categoryName: string;
    categoryIcon: string;
    categoryTheme: 'green' | 'blue' | 'pink' | 'orange' | 'purple';
    time: string;
    content: string;
    images?: string[];
    tags?: string[];
    commentsCount?: number;
    likesCount?: number;
  }

  let {
    categoryName,
    categoryIcon,
    categoryTheme,
    time,
    content,
    images = [],
    tags = [],
    commentsCount = 0,
    likesCount = 0
  }: Props = $props();

  const themeClasses = {
    green: 'theme-green',
    blue: 'theme-blue',
    pink: 'theme-pink',
    orange: 'theme-orange',
    purple: 'theme-purple'
  };
</script>

<div class="timeline-item">
  <!-- Left Side: Date / Timeline Line -->
  <div class="timeline-left">
    <div class="timeline-dot {themeClasses[categoryTheme]}"></div>
  </div>

  <!-- Right Side: Content Card -->
  <div class="timeline-content">
    <Card>
      <!-- Header with Category -->
      <div class="card-header">
        <span class="category-badge {themeClasses[categoryTheme]}">
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
        <span class="time-label">{time}</span>
        <div class="actions">
          <button class="action-btn">
            <svg class="action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/>
            </svg>
            <span class="action-count">{commentsCount}</span>
          </button>
          <button class="action-btn like">
            <svg class="action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
            </svg>
            <span class="action-count">{likesCount}</span>
          </button>
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
    background-color: var(--muted);
    z-index: 2;
    margin-top: 30px;
    box-shadow: 0 0 0 4px var(--background);
    transition: transform 0.2s ease;
  }

  .timeline-item:hover .timeline-dot {
    transform: scale(1.3);
  }

  /* Themes */
  .theme-green {
    --badge-bg: var(--accent-green-bg);
    --badge-color: var(--accent-green);
  }
  .theme-blue {
    --badge-bg: var(--accent-blue-bg);
    --badge-color: var(--accent-blue);
  }
  .theme-pink {
    --badge-bg: var(--accent-pink-bg);
    --badge-color: var(--accent-pink);
  }
  .theme-orange {
    --badge-bg: var(--accent-orange-bg);
    --badge-color: var(--accent-orange);
  }
  .theme-purple {
    --badge-bg: rgba(96, 37, 255, 0.08);
    --badge-color: var(--primary-purple);
  }

  .timeline-dot.theme-green { background-color: var(--accent-green); }
  .timeline-dot.theme-blue { background-color: var(--accent-blue); }
  .timeline-dot.theme-pink { background-color: var(--accent-pink); }
  .timeline-dot.theme-orange { background-color: var(--accent-orange); }
  .timeline-dot.theme-purple { background-color: var(--primary-purple); }

  .timeline-content {
    flex-grow: 1;
    padding-left: 12px;
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

  /* Photo Grid */
  .photo-grid {
    display: grid;
    gap: 8px;
    margin-bottom: 16px;
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .cols-1 { grid-template-columns: 1fr; }
  .cols-2 { grid-template-columns: 1fr 1fr; }
  .cols-3 { grid-template-columns: 1.2fr 1fr 1fr; }

  .photo-wrapper {
    position: relative;
    aspect-ratio: 4/3;
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
    background-color: var(--light-gray);
    color: var(--muted);
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 500;
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

  .actions {
    display: flex;
    gap: 16px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--muted);
    transition: color 0.2s ease, transform 0.1s ease;
  }

  .action-btn:hover {
    color: var(--primary-purple);
    transform: scale(1.05);
  }

  .action-btn.like:hover {
    color: var(--primary-pink);
  }

  .action-icon {
    width: 18px;
    height: 18px;
  }

  .action-count {
    font-size: 0.8rem;
    font-weight: 600;
  }
</style>
