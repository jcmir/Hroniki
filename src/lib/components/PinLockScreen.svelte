<script lang="ts">
  import { fade } from 'svelte/transition';

  interface Props {
    onVerify: (pin: string) => Promise<boolean>;
    onSuccess: () => void;
  }

  let { onVerify, onSuccess }: Props = $props();

  let enteredPin = $state('');
  let shake = $state(false);
  let errorMsg = $state('');

  async function handleKey(num: string) {
    if (enteredPin.length < 4) {
      enteredPin += num;
      errorMsg = '';
    }

    if (enteredPin.length === 4) {
      setTimeout(async () => {
        const ok = await onVerify(enteredPin);
        if (ok) {
          onSuccess();
        } else {
          enteredPin = '';
          shake = true;
          errorMsg = 'Неверный PIN-код';
          setTimeout(() => {
            shake = false;
          }, 500);
        }
      }, 100);
    }
  }

  function handleBackspace() {
    if (enteredPin.length > 0) {
      enteredPin = enteredPin.slice(0, -1);
      errorMsg = '';
    }
  }
</script>

<div class="lock-screen-overlay" transition:fade={{ duration: 350 }}>
  <div class="lock-container {shake ? 'shake-anim' : ''}">
    <div class="lock-header">
      <span class="lock-icon">🔒</span>
      <h2>АРХИВ ЗАБЛОКИРОВАН</h2>
      <p class="lock-prompt" style="color: {errorMsg ? '#ff5555' : 'rgba(255,255,255,0.6)'}">
        {errorMsg || 'Введите PIN-код для доступа к Хроникам'}
      </p>
    </div>

    <!-- Dots -->
    <div class="dots-row">
      <div class="dot {enteredPin.length >= 1 ? 'filled' : ''}"></div>
      <div class="dot {enteredPin.length >= 2 ? 'filled' : ''}"></div>
      <div class="dot {enteredPin.length >= 3 ? 'filled' : ''}"></div>
      <div class="dot {enteredPin.length >= 4 ? 'filled' : ''}"></div>
    </div>

    <!-- Keypad -->
    <div class="keypad-grid">
      {#each ['1', '2', '3', '4', '5', '6', '7', '8', '9'] as num}
        <button type="button" class="key-btn" onclick={() => handleKey(num)}>{num}</button>
      {/each}
      <button type="button" class="key-btn action-key" onclick={() => enteredPin = ''}>Сброс</button>
      <button type="button" class="key-btn" onclick={() => handleKey('0')}>0</button>
      <button type="button" class="key-btn action-key" onclick={handleBackspace}>⌫</button>
    </div>
  </div>
</div>

<style>
  .lock-screen-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: radial-gradient(circle at center, rgba(30, 27, 38, 0.98), #0c0a10);
    z-index: 9999;
    display: flex;
    justify-content: center;
    align-items: center;
    backdrop-filter: blur(16px);
  }

  .lock-container {
    width: 100%;
    max-width: 340px;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 24px;
    color: white;
  }

  .lock-header {
    text-align: center;
    margin-bottom: 32px;
  }

  .lock-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: 16px;
  }

  .lock-header h2 {
    font-size: 1.4rem;
    font-weight: 700;
    letter-spacing: 1px;
    margin-bottom: 8px;
    color: #ffffff;
  }

  .lock-prompt {
    font-size: 0.88rem;
    transition: color 0.2s;
  }

  .dots-row {
    display: flex;
    gap: 20px;
    margin-bottom: 48px;
  }

  .dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.3);
    transition: all 0.2s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .dot.filled {
    background: #6025ff;
    border-color: #6025ff;
    box-shadow: 0 0 12px #6025ff;
    transform: scale(1.15);
  }

  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    width: 100%;
  }

  .key-btn {
    border: none;
    background: rgba(255, 255, 255, 0.05);
    color: white;
    font-size: 1.6rem;
    font-weight: 600;
    width: 72px;
    height: 72px;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: background-color 0.2s, transform 0.1s;
    justify-self: center;
  }

  .key-btn:active {
    background: rgba(255, 255, 255, 0.15);
    transform: scale(0.92);
  }

  .key-btn.action-key {
    font-size: 0.95rem;
    font-weight: 500;
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
  }

  .key-btn.action-key:active {
    color: white;
  }

  /* Shake animation */
  .shake-anim {
    animation: shake 0.4s ease;
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    20%, 60% { transform: translateX(-8px); }
    40%, 80% { transform: translateX(8px); }
  }
</style>
