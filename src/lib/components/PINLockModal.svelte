<script lang="ts">
  import { sessionStore } from '$lib/stores/session';

  let pin = '';
  let isChecking = false;

  function handleDigit(digit: string) {
    if (pin.length < 6) {
      pin += digit;
      sessionStore.clearError();
    }
  }

  function handleBackspace() {
    if (pin.length > 0) {
      pin = pin.slice(0, -1);
      sessionStore.clearError();
    }
  }

  async function handleUnlock() {
    if (pin.length < 4) return;
    isChecking = true;
    const ok = await sessionStore.unlockWithPin(pin);
    isChecking = false;
    if (!ok) {
      pin = '';
    }
  }
</script>

<div class="lock-overlay">
  <div class="lock-card">
    <div class="lock-icon">🔒</div>
    <h2>ХРОНИКИ Заблокированы</h2>
    <p class="lock-sub">Введите ваш PIN-код для доступа к личным воспоминаниям</p>

    <!-- PIN dots preview -->
    <div class="pin-dots">
      {#each Array(4) as _, i}
        <span class="dot" class:filled={i < pin.length}></span>
      {/each}
    </div>

    {#if $sessionStore.error}
      <div class="error-msg">{$sessionStore.error}</div>
    {/if}

    <!-- Numeric Keypad -->
    <div class="keypad-grid">
      {#each ['1','2','3','4','5','6','7','8','9'] as num}
        <button class="key-btn" type="button" on:click={() => handleDigit(num)}>{num}</button>
      {/each}
      <button class="key-btn action-key" type="button" on:click={() => (pin = '')}>C</button>
      <button class="key-btn" type="button" on:click={() => handleDigit('0')}>0</button>
      <button class="key-btn action-key" type="button" on:click={handleBackspace}>⌫</button>
    </div>

    <button
      class="unlock-submit-btn"
      type="button"
      disabled={pin.length < 4 || isChecking}
      on:click={handleUnlock}
    >
      {isChecking ? 'Проверка...' : 'Разблокировать'}
    </button>
  </div>
</div>

<style>
  .lock-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: #08090c;
    backdrop-filter: blur(16px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 2000;
  }

  .lock-card {
    background-color: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    width: 90%;
    max-width: 380px;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    box-shadow: var(--shadow-card);
  }

  .lock-icon {
    font-size: 2.5rem;
    margin-bottom: 0.75rem;
  }

  .lock-card h2 {
    font-family: var(--font-heading);
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-main);
    margin-bottom: 0.4rem;
  }

  .lock-sub {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-bottom: 1.5rem;
    line-height: 1.4;
  }

  .pin-dots {
    display: flex;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid var(--border-subtle);
    transition: background-color 0.2s ease, border-color 0.2s ease;
  }

  .dot.filled {
    background-color: var(--accent-amber);
    border-color: var(--accent-amber);
    box-shadow: var(--shadow-glow);
  }

  .error-msg {
    color: var(--accent-crimson);
    font-size: 0.85rem;
    margin-bottom: 1rem;
  }

  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
    width: 100%;
    margin-bottom: 1.5rem;
  }

  .key-btn {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    color: var(--text-main);
    font-size: 1.25rem;
    font-weight: 600;
    height: 54px;
    border-radius: var(--radius-md);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }

  .key-btn:active {
    background-color: var(--bg-surface-elevated);
    transform: scale(0.95);
  }

  .action-key {
    color: var(--text-muted);
    font-size: 1rem;
  }

  .unlock-submit-btn {
    width: 100%;
    background-color: var(--accent-amber);
    border: none;
    color: #000;
    padding: 0.85rem;
    border-radius: var(--radius-pill);
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
  }

  .unlock-submit-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
