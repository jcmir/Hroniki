<script lang="ts">
  interface Props {
    categoryName: string;
    class?: string;
    size?: number;
  }

  let { categoryName, class: className = '', size = 20 }: Props = $props();

  // Normalize category name
  const name = $derived(categoryName.trim());
</script>

{#if name === 'Сад' || name === 'Садоводство'}
  <!-- Sprout / Leaf SVG -->
  <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
    <path d="M12 22C12 22 20 18 20 12C20 7.5 17.5 4 12 2C6.5 4 4 7.5 4 12C4 18 12 22 12 22Z" />
    <path d="M12 2V22" />
    <path d="M12 12C12 12 16 10 18 12" />
    <path d="M12 15C12 15 8 13 6 15" />
  </svg>
{:else}
  {#if name === 'Авто' || name === 'Автомобиль'}
    <!-- Car SVG -->
    <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2" />
      <circle cx="7" cy="17" r="2" />
      <circle cx="17" cy="17" r="2" />
      <path d="M9 17h6" />
    </svg>
  {:else}
    {#if name === 'Дом' || name === 'Жилье'}
      <!-- House SVG -->
      <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
        <polyline points="9 22 9 12 15 12 15 22" />
      </svg>
    {:else}
      {#if name === 'Здоровье'}
        <!-- Heart SVG -->
        <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
        </svg>
      {:else}
        {#if name === 'Питомцы' || name === 'Питомец'}
          <!-- Paw Print SVG -->
          <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="5" r="2" />
            <circle cx="5.5" cy="9" r="2" />
            <circle cx="18.5" cy="9" r="2" />
            <circle cx="8" cy="16" r="3" />
            <circle cx="16" cy="16" r="3" />
          </svg>
        {:else}
          {#if name === 'Документы' || name === 'Документ'}
            <!-- File SVG -->
            <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
              <path d="M14 2v4a2 2 0 0 0 2 2h4" />
              <path d="M10 9H8" />
              <path d="M16 13H8" />
              <path d="M16 17H8" />
            </svg>
          {:else}
            {#if name === 'Спорт' || name === 'Фитнес'}
              <!-- Activity / Heartbeat SVG -->
              <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
              </svg>
            {:else}
              {#if name === 'Работа' || name === 'Дела'}
                <!-- Briefcase SVG -->
                <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M15 2H9a2 2 0 0 0-2 2v2H3a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h18a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-4V4a2 2 0 0 0-2-2z" />
                  <path d="M7 6h10" />
                  <path d="M3 11h18" />
                </svg>
              {:else}
                {#if name === 'Техника' || name === 'Гаджеты'}
                  <!-- Laptop/Monitor SVG -->
                  <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
                    <line x1="8" y1="21" x2="16" y2="21" />
                    <line x1="12" y1="17" x2="12" y2="21" />
                  </svg>
                {:else}
                  {#if name === 'Путешествия' || name === 'Поездки'}
                    <!-- Airplane SVG -->
                    <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3.5c-.5-.5-2.5 0-4 1.5L13.5 8.5 5.3 6.7 3.5 8.5l8.5 3.5-3.5 3.5-3.5-.5L3.5 16.5l3.5 1.8 1.8 3.5 1.5-1.5-.5-3.5 3.5-3.5 3.5 8.5Z" />
                    </svg>
                  {:else}
                    {#if name === 'Напоминания' || name === 'Напоминание'}
                      <!-- Bell SVG -->
                      <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
                        <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
                      </svg>
                    {:else}
                      <!-- Default Sparkles/Star SVG -->
                      <svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" />
                      </svg>
                    {/if}
                  {/if}
                {/if}
              {/if}
            {/if}
          {/if}
        {/if}
      {/if}
    {/if}
  {/if}
{/if}
