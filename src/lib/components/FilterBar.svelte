<script lang="ts">
  import { categoriesStore } from '$lib/stores/categories';
  import { entriesStore } from '$lib/stores/entries';

  let searchText = '';

  function handleSearchInput() {
    entriesStore.searchEntries(searchText, $categoriesStore.selectedCategory || undefined);
  }

  function handleSelectCategory(catId: string) {
    const nextCat = catId === $categoriesStore.selectedCategory ? null : catId;
    categoriesStore.selectCategory(nextCat);
    entriesStore.searchEntries(searchText, nextCat || undefined);
  }
</script>

<div class="filter-bar">
  <!-- Search input -->
  <div class="search-box">
    <span class="search-icon">🔍</span>
    <input
      type="text"
      bind:value={searchText}
      on:input={handleSearchInput}
      placeholder="Поиск по воспоминаниям и названиям..."
    />
    {#if searchText}
      <button
        class="clear-search-btn"
        on:click={() => {
          searchText = '';
          handleSearchInput();
        }}
      >✕</button>
    {/if}
  </div>

  <!-- Category Chips -->
  <div class="chips-scroll">
    {#each $categoriesStore.categories as cat (cat.id)}
      <button
        class="chip-btn"
        class:active={$categoriesStore.selectedCategory === cat.id || ($categoriesStore.selectedCategory === null && cat.id === 'all')}
        on:click={() => handleSelectCategory(cat.id)}
      >
        <span class="chip-icon">{cat.icon}</span>
        <span class="chip-label">{cat.name}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .filter-bar {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-pill);
    padding: 0.6rem 1rem;
    transition: border-color 0.2s ease;
  }

  .search-box:focus-within {
    border-color: var(--accent-amber);
  }

  .search-icon {
    font-size: 1rem;
    margin-right: 0.6rem;
    color: var(--text-muted);
  }

  .search-box input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 0.9rem;
  }

  .clear-search-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 0.9rem;
    cursor: pointer;
  }

  .chips-scroll {
    display: flex;
    gap: 0.5rem;
    overflow-x: auto;
    padding-bottom: 0.25rem;
    scrollbar-width: none;
  }

  .chips-scroll::-webkit-scrollbar {
    display: none;
  }

  .chip-btn {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    color: var(--text-muted);
    font-size: 0.85rem;
    padding: 0.4rem 0.85rem;
    border-radius: var(--radius-pill);
    white-space: nowrap;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .chip-btn.active {
    background-color: var(--accent-amber-glow);
    border-color: var(--accent-amber);
    color: var(--accent-amber);
    font-weight: 600;
  }
</style>
