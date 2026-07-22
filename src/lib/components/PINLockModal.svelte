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
    {:else}
      <div class="spacer-error"></div>
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
    inset: 0;
    background-color: rgba(8, 9, 12, 0.95);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 3000; /* Highest priority */
  }

  .lock-card {
    background-color: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: 32px;
    width: 90%;
    max-width: 360px;
    padding: 2.5rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    box-shadow: 0 24px 64px rgba(0, 0, 0, 0.4);
  }

  .lock-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
    filter: drop-shadow(0 0 12px rgba(124, 58, 237, 0.3));
  }

  .lock-card h2 {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
    margin-bottom: 0.5rem;
  }

  .lock-sub {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-bottom: 2rem;
    line-height: 1.5;
  }

  .pin-dots {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .dot {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid var(--border-subtle);
    background-color: transparent;
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .dot.filled {
    background-color: var(--accent-primary);
    border-color: var(--accent-primary);
    transform: scale(1.1);
    box-shadow: 0 0 15px rgba(124, 58, 237, 0.4);
  }

  .error-msg {
    color: #ef4444;
    font-size: 0.85rem;
    font-weight: 600;
    height: 1.5rem;
    margin-bottom: 1rem;
  }

  .spacer-error {
    height: 2.5rem;
  }

  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    width: 100%;
    margin-bottom: 2rem;
  }

  .key-btn {
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
    color: var(--text-main);
    font-size: 1.5rem;
    font-weight: 600;
    height: 64px;
    border-radius: 20px;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: all 0.15s ease;
    -webkit-tap-highlight-color: transparent;
  }

  .key-btn:active {
    background-color: var(--accent-primary);
    color: white;
    transform: scale(0.92);
    border-color: var(--accent-primary);
  }

  .action-key {
    color: var(--text-muted);
    font-size: 1.1rem;
    background-color: transparent;
  }

  .unlock-submit-btn {
    width: 100%;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-pink));
    border: none;
    color: white;
    padding: 1.1rem;
    border-radius: var(--radius-pill);
    font-weight: 700;
    font-size: 1.1rem;
    cursor: pointer;
    box-shadow: 0 8px 24px rgba(124, 58, 237, 0.3);
    transition: all 0.2s ease;
  }

  .unlock-submit-btn:disabled {
    opacity: 0.4;
    box-shadow: none;
    filter: grayscale(1);
    cursor: not-allowed;
  }
</style>
