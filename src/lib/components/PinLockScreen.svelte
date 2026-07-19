<script lang="ts">
  import { fade, scale } from 'svelte/transition';

  interface Props {
    onVerify: (pin: string) => Promise<boolean>;
    onSuccess: () => void;
  }

  let { onVerify, onSuccess }: Props = $props();

  let enteredPin = $state('');
  let shake = $state(false);
  let verifying = $state(false);
  let errorMsg = $state('');
  let successAnim = $state(false);

  async function handleKey(num: string) {
    if (verifying || enteredPin.length >= 4) return;
    enteredPin += num;
    errorMsg = '';

    if (enteredPin.length === 4) {
      verifying = true;
      await new Promise(r => setTimeout(r, 120));
      const ok = await onVerify(enteredPin);
      if (ok) {
        successAnim = true;
        await new Promise(r => setTimeout(r, 350));
        onSuccess();
      } else {
        enteredPin = '';
        verifying = false;
        shake = true;
        errorMsg = 'Неверный PIN-код';
        setTimeout(() => { shake = false; }, 500);
      }
    }
  }

  function handleBackspace() {
    if (enteredPin.length > 0 && !verifying) {
      enteredPin = enteredPin.slice(0, -1);
      errorMsg = '';
    }
  }
</script>

<div class="lock-screen-overlay" class:success-bg={successAnim} transition:fade={{ duration: 400 }}>
  <!-- Ambient glow orbs -->
  <div class="orb orb-1"></div>
  <div class="orb orb-2"></div>

  <div class="lock-container" class:shake-anim={shake}>
    <!-- Logo / Branding -->
    <div class="lock-brand" in:fade={{ duration: 600, delay: 100 }}>
      <div class="brand-icon">
        <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg" width="40" height="40">
          <circle cx="20" cy="20" r="18" stroke="url(#g1)" stroke-width="2.5" fill="none"/>
          <path d="M13 20 Q20 12 27 20 Q20 28 13 20Z" fill="url(#g2)" opacity="0.9"/>
          <defs>
            <linearGradient id="g1" x1="0" y1="0" x2="40" y2="40" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stop-color="#a78bfa"/>
              <stop offset="100%" stop-color="#6d28d9"/>
            </linearGradient>
            <linearGradient id="g2" x1="0" y1="0" x2="40" y2="40" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stop-color="#c4b5fd"/>
              <stop offset="100%" stop-color="#7c3aed"/>
            </linearGradient>
          </defs>
        </svg>
      </div>
      <span class="brand-name">ХРОНИКИ</span>
    </div>

    <!-- Status text -->
    <div class="lock-status" in:fade={{ duration: 400, delay: 200 }}>
      <p class="status-text" class:error-text={!!errorMsg}>
        {errorMsg || (verifying ? 'Проверка…' : 'Введите PIN-код')}
      </p>
    </div>

    <!-- PIN Dots -->
    <div class="dots-row" in:scale={{ duration: 400, delay: 250, start: 0.8 }}>
      {#each [1, 2, 3, 4] as idx}
        <div
          class="dot"
          class:filled={enteredPin.length >= idx}
          class:success-dot={successAnim && enteredPin.length >= idx}
        ></div>
      {/each}
    </div>

    <!-- Keypad -->
    <div class="keypad-grid" in:fade={{ duration: 400, delay: 300 }}>
      {#each ['1', '2', '3', '4', '5', '6', '7', '8', '9'] as num}
        <button
          type="button"
          class="key-btn"
          id="pin-key-{num}"
          onclick={() => handleKey(num)}
          disabled={verifying}
        >
          <span class="key-num">{num}</span>
          <span class="key-letters">{['', 'ABC', 'DEF', 'GHI', 'JKL', 'MNO', 'PQRS', 'TUV', 'WXYZ'][parseInt(num)]}</span>
        </button>
      {/each}
      <button type="button" class="key-btn action-key" onclick={() => enteredPin = ''} disabled={verifying}>
        Сброс
      </button>
      <button
        type="button"
        class="key-btn"
        id="pin-key-0"
        onclick={() => handleKey('0')}
        disabled={verifying}
      >
        <span class="key-num">0</span>
      </button>
      <button type="button" class="key-btn action-key" onclick={handleBackspace} disabled={verifying}>
        ⌫
      </button>
    </div>
  </div>
</div>

<style>
  .lock-screen-overlay {
    position: fixed;
    inset: 0;
    background: radial-gradient(ellipse at 40% 30%, #1a1030 0%, #0d0b14 60%, #090810 100%);
    z-index: 9999;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: background 0.5s ease;
    overflow: hidden;
  }

  .lock-screen-overlay.success-bg {
    background: radial-gradient(ellipse at 40% 30%, #0d2a1a 0%, #060f0a 100%);
  }

  .orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    pointer-events: none;
    animation: orb-pulse 8s ease-in-out infinite alternate;
  }

  .orb-1 {
    width: 280px;
    height: 280px;
    background: radial-gradient(circle, rgba(109, 40, 217, 0.25) 0%, transparent 70%);
    top: -60px;
    right: -60px;
    animation-delay: 0s;
  }

  .orb-2 {
    width: 220px;
    height: 220px;
    background: radial-gradient(circle, rgba(139, 92, 246, 0.15) 0%, transparent 70%);
    bottom: 40px;
    left: -60px;
    animation-delay: 3s;
  }

  @keyframes orb-pulse {
    0%   { opacity: 0.6; transform: scale(1) translate(0, 0); }
    100% { opacity: 1; transform: scale(1.2) translate(10px, -10px); }
  }

  .lock-container {
    position: relative;
    z-index: 1;
    width: 100%;
    max-width: 320px;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 32px 24px 40px;
    color: white;
  }

  .lock-brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    margin-bottom: 40px;
  }

  .brand-icon {
    width: 60px;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(167, 139, 250, 0.08);
    border-radius: 18px;
    border: 1px solid rgba(167, 139, 250, 0.2);
    backdrop-filter: blur(8px);
    box-shadow: 0 0 30px rgba(109, 40, 217, 0.2);
  }

  .brand-name {
    font-size: 0.85rem;
    font-weight: 700;
    letter-spacing: 0.25em;
    color: rgba(196, 181, 253, 0.8);
    text-transform: uppercase;
  }

  .lock-status {
    height: 22px;
    margin-bottom: 24px;
  }

  .status-text {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.5);
    text-align: center;
    transition: color 0.2s;
    font-weight: 400;
    margin: 0;
  }

  .status-text.error-text {
    color: #f87171;
    font-weight: 500;
  }

  .dots-row {
    display: flex;
    gap: 22px;
    margin-bottom: 52px;
  }

  .dot {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    border: 1.5px solid rgba(255, 255, 255, 0.2);
    background: transparent;
    transition: all 0.22s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .dot.filled {
    background: #8b5cf6;
    border-color: #8b5cf6;
    box-shadow: 0 0 14px rgba(139, 92, 246, 0.7);
    transform: scale(1.2);
  }

  .dot.success-dot {
    background: #34d399;
    border-color: #34d399;
    box-shadow: 0 0 14px rgba(52, 211, 153, 0.7);
  }

  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    width: 100%;
    max-width: 280px;
  }

  .key-btn {
    border: 1px solid rgba(255, 255, 255, 0.07);
    background: rgba(255, 255, 255, 0.04);
    color: white;
    width: 76px;
    height: 76px;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    transition: background 0.15s, transform 0.1s, border-color 0.15s;
    justify-self: center;
    gap: 2px;
    -webkit-tap-highlight-color: transparent;
    backdrop-filter: blur(4px);
  }

  .key-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.14);
  }

  .key-btn:active:not(:disabled) {
    background: rgba(139, 92, 246, 0.2);
    border-color: rgba(139, 92, 246, 0.4);
    transform: scale(0.91);
  }

  .key-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .key-num {
    font-size: 1.7rem;
    font-weight: 300;
    line-height: 1;
    letter-spacing: -0.5px;
    color: #fff;
  }

  .key-letters {
    font-size: 0.52rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    color: rgba(255, 255, 255, 0.35);
    line-height: 1;
  }

  .key-btn.action-key {
    background: transparent;
    border-color: transparent;
    font-size: 0.9rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.5);
    letter-spacing: 0.03em;
  }

  .key-btn.action-key:hover:not(:disabled) {
    background: transparent;
    color: rgba(255, 255, 255, 0.85);
    border-color: transparent;
  }

  .key-btn.action-key:active:not(:disabled) {
    background: transparent;
    border-color: transparent;
    transform: scale(0.92);
    color: white;
  }

  .shake-anim {
    animation: shake 0.42s cubic-bezier(0.36, 0.07, 0.19, 0.97);
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    15%       { transform: translateX(-10px); }
    30%       { transform: translateX(9px); }
    45%       { transform: translateX(-7px); }
    60%       { transform: translateX(6px); }
    75%       { transform: translateX(-4px); }
    90%       { transform: translateX(3px); }
  }
</style>



