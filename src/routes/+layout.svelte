<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import '../app.css';
  import { sessionStore } from '$lib/stores/session';
  import { platformStore } from '$lib/stores/platform';
  import { categoriesStore } from '$lib/stores/categories';
  import { entriesStore } from '$lib/stores/entries';
  import BottomNav from '$lib/components/BottomNav.svelte';
  import PINLockModal from '$lib/components/PINLockModal.svelte';
  import SplashScreen from '$lib/components/SplashScreen.svelte';
  import FirstRunWizard from '$lib/components/FirstRunWizard.svelte';
  import CreateEntryModal from '$lib/components/CreateEntryModal.svelte';
  import CreateObjectModal from '$lib/components/CreateObjectModal.svelte';
  import FloatingActionButton from '$lib/design/FloatingActionButton.svelte';

  // Absolute initial state: Booting is TRUE
  let booting = true;
  let showSplash = true;
  let showWizard = false;
  let showCreateEntry = false;
  let showCreateObject = false;
  let initialized = false;

  $: currentPath = $page.url.pathname;
  $: isSettings = currentPath === '/settings';
  $: isObjects = currentPath === '/objects';
  $: isReminders = currentPath === '/reminders';
  $: isObjectDetail = currentPath.startsWith('/object/');
  $: currentObjectId = isObjectDetail ? currentPath.split('/').pop() : undefined;

  // Contextual FAB logic: Only on Feed and Object Detail
  $: showFab = !booting && !showSplash && !isSettings && !isObjects && !isReminders && $sessionStore.state !== 'locked';

  onMount(() => {
    console.log('[Layout] Initializing stores...');

    const handleRequestCreate = () => showCreateObject = true;
    window.addEventListener('request-create-object', handleRequestCreate);

    (async () => {
      try {
        // Parallel init of all stores
        await Promise.all([
          sessionStore.init(),
          platformStore.loadCapabilities(),
          categoriesStore.loadCategories(),
          entriesStore.loadEntries()
        ]);
        initialized = true;
      } catch (e) {
        console.error('[Layout] Init error:', e);
      }
    })();

    return () => window.removeEventListener('request-create-object', handleRequestCreate);
  });

  // Wizard logic: Only if no entries AND no PIN configured
  $: if (initialized && !showSplash && $entriesStore.entries.length === 0 && $sessionStore.state === 'pinNotConfigured') {
    showWizard = true;
  }

  function handleFabClick() {
    if (currentPath === '/' || isObjectDetail) {
      showCreateEntry = true;
    }
  }

  function handleSplashDone() {
    console.log('[Layout] Splash done. Transitioning to app.');
    showSplash = false;
    booting = false;
  }

  function openCreateObject() {
    showCreateObject = true;
  }

  function handleObjectCreated() {
    showCreateObject = false;
    // Potentially reload objects if store exists, but list in +page.svelte usually re-fetches or uses invoke
    // Since we don't have an objectsStore yet, the page will need to handle its own refresh
    window.dispatchEvent(new CustomEvent('object-created'));
  }
</script>

<!-- Build Marker (Debug only) -->
{#if !showSplash}
<div class="debug-marker">DEBUG: 21.07.2026 - v1.1.r</div>
{/if}

<div class="app-shell" class:is-locked={$sessionStore.state === 'locked'}>
  {#if showSplash}
    <SplashScreen on:done={handleSplashDone} />
  {/if}

  {#if !booting}
    <!-- Main UI - only rendered after boot -->
    <header class="top-bar">
      <div class="top-bar-content">
        <div class="brand">
          <span class="logo">📖</span>
          <div class="titles">
            <h1>ХРОНИКИ</h1>
            <span class="subtitle">Архив Воспоминаний</span>
          </div>
        </div>

        <div class="status-actions">
          {#if $sessionStore.state === 'locked'}
            <span class="lock-icon">🔒</span>
          {:else if $sessionStore.state === 'unlocked' || $sessionStore.state === 'pinNotConfigured'}
            <button class="lock-btn" type="button" on:click={() => sessionStore.lock()} aria-label="Заблокировать">
              🔓
            </button>
          {/if}
        </div>
      </div>
    </header>

    <main class="main-container" aria-hidden={$sessionStore.state === 'locked'}>
      <slot {openCreateObject} />
    </main>

    {#if showFab}
      <FloatingActionButton on:click={handleFabClick} />
    {/if}

    <BottomNav />

    <!-- Modals -->
    {#if showCreateEntry}
      <CreateEntryModal
        objectId={currentObjectId}
        on:close={() => (showCreateEntry = false)}
      />
    {/if}

    {#if showCreateObject}
      <CreateObjectModal
        on:close={() => (showCreateObject = false)}
        on:success={handleObjectCreated}
      />
    {/if}

    {#if $sessionStore.state === 'locked'}
      <div class="lock-screen-overlay">
        <PINLockModal />
      </div>
    {/if}

    {#if showWizard && $sessionStore.state !== 'locked'}
      <FirstRunWizard on:complete={() => (showWizard = false)} />
    {/if}
  {/if}
</div>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100dvh;
    overflow: hidden;
    position: relative;
    background-color: var(--bg-app);
  }

  .debug-marker {
    position: fixed;
    top: 2px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 10000;
    font-size: 8px;
    color: rgba(124, 58, 237, 0.4);
    pointer-events: none;
    font-family: monospace;
  }

  /* Interaction blocking when locked */
  .is-locked .main-container,
  .is-locked :global(.bottom-nav-safe-area),
  .is-locked :global(.fab-button),
  .is-locked .top-bar-content {
    pointer-events: none !important;
    user-select: none !important;
    filter: blur(12px) grayscale(0.8);
    opacity: 0.6;
  }

  .lock-screen-overlay {
    position: fixed;
    inset: 0;
    z-index: 5000;
    background: var(--bg-app);
  }

  .top-bar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 400;
    background-color: var(--bg-glass);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border-bottom: 1px solid var(--border-subtle);
    padding-top: var(--safe-area-top);
  }

  .top-bar-content {
    height: 64px;
    padding: 0 1.25rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .brand { display: flex; align-items: center; gap: 0.75rem; }
  .logo { font-size: 1.5rem; }
  .titles h1 {
    font-family: var(--font-heading);
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-main);
    line-height: 1.1;
  }
  .subtitle { font-size: 0.7rem; color: var(--text-muted); font-weight: 500; }

  .lock-btn {
    background: none;
    border: none;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: var(--radius-sm);
  }

  .main-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding-top: var(--top-total-offset);
    padding-bottom: var(--bottom-total-offset);
    scroll-behavior: smooth;
    -webkit-overflow-scrolling: touch;
    padding-left: var(--safe-area-left);
    padding-right: var(--safe-area-right);
  }
</style>
