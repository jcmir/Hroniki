<script lang="ts">
  import { fade, slide } from 'svelte/transition';

  interface Props {
    onComplete: (username: string) => void;
    onSeedDemo: () => Promise<void>;
    onSetPin: (pin: string) => Promise<void>;
  }

  let { onComplete, onSeedDemo, onSetPin }: Props = $props();

  let step = $state(1); // 1: Welcome, 2: Name, 3: Protection
  let username = $state('');
  let protectionType = $state<'none' | 'pin'>('none');
  
  let pinCode = $state('');
  let pinConfirm = $state('');
  let pinError = $state('');
  let isSubmitting = $state(false);

  async function handleStart(demo = false) {
    if (demo) {
      isSubmitting = true;
      try {
        await onSeedDemo();
        // Skip straight to Alexander as default name
        username = 'Александр';
        step = 3;
      } catch (e) {
        console.error(e);
      } finally {
        isSubmitting = false;
      }
    } else {
      step = 2;
    }
  }

  function handleNextStep2() {
    if (username.trim()) {
      step = 3;
    }
  }

  async function handleFinish() {
    if (protectionType === 'pin') {
      if (pinCode.length < 4) {
        pinError = 'PIN должен состоять из 4 цифр';
        return;
      }
      if (pinCode !== pinConfirm) {
        pinError = 'PIN-коды не совпадают';
        return;
      }
      isSubmitting = true;
      try {
        await onSetPin(pinCode);
      } catch (e) {
        pinError = 'Не удалось установить PIN-код';
        isSubmitting = false;
        return;
      }
    }

    isSubmitting = true;
    try {
      onComplete(username);
    } catch (e) {
      console.error(e);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="onboarding-overlay" transition:fade={{ duration: 400 }}>
  <div class="onboarding-card">
    {#if step === 1}
      <div class="step-content text-center" in:fade={{ duration: 200 }}>
        <span class="logo-spark">🌱</span>
        <h1 class="logo-title">ХРОНИКИ</h1>
        <p class="tagline">Ваша личная история. События. Фото. Объекты.</p>
        <p class="tagline-sub">Всё локально, приватно и в одном месте.</p>
        
        <div class="action-buttons">
          <button type="button" class="btn btn-primary" onclick={() => handleStart(false)}>
            Создать новый архив
          </button>
          <button type="button" class="btn btn-secondary" onclick={() => handleStart(true)} disabled={isSubmitting}>
            {isSubmitting ? 'Создаем демо...' : 'Посмотреть демо-хронику'}
          </button>
        </div>
      </div>
    {:else if step === 2}
      <div class="step-content" in:fade={{ duration: 200 }}>
        <h2>Как вас зовут?</h2>
        <p class="step-desc">Хроники будут обращаться к вам по имени</p>
        
        <div class="form-group">
          <input
            id="username-onboarding"
            type="text"
            class="form-input"
            placeholder="Ваше имя (например, Александр)"
            bind:value={username}
            autofocus
            onkeydown={(e) => e.key === 'Enter' && handleNextStep2()}
          />
        </div>

        <button type="button" class="btn btn-primary" onclick={handleNextStep2} disabled={!username.trim()}>
          Продолжить
        </button>
      </div>
    {:else if step === 3}
      <div class="step-content" in:fade={{ duration: 200 }}>
        <h2>Защитить архив?</h2>
        <p class="step-desc">Шифрование и блокировка доступа к вашей истории</p>
        
        <div class="protection-options">
          <label class="option-card {protectionType === 'none' ? 'selected' : ''}">
            <input type="radio" name="protection" value="none" bind:group={protectionType} />
            <div class="option-info">
              <span class="option-title">🔓 Без защиты</span>
              <span class="option-desc">Быстрый вход без PIN-кода (позже можно включить в настройках)</span>
            </div>
          </label>

          <label class="option-card {protectionType === 'pin' ? 'selected' : ''}">
            <input type="radio" name="protection" value="pin" bind:group={protectionType} />
            <div class="option-info">
              <span class="option-title">🔒 Защитить PIN-кодом</span>
              <span class="option-desc">Запрашивать 4-значный код при каждом запуске</span>
            </div>
          </label>
        </div>

        {#if protectionType === 'pin'}
          <div class="pin-inputs" transition:slide>
            <div class="form-row">
              <div class="form-group">
                <label class="form-label" for="pin-ob-1">Введите PIN</label>
                <input
                  id="pin-ob-1"
                  type="password"
                  maxlength="4"
                  pattern="[0-9]*"
                  inputmode="numeric"
                  class="form-input text-center"
                  placeholder="••••"
                  bind:value={pinCode}
                />
              </div>
              <div class="form-group">
                <label class="form-label" for="pin-ob-2">Повторите PIN</label>
                <input
                  id="pin-ob-2"
                  type="password"
                  maxlength="4"
                  pattern="[0-9]*"
                  inputmode="numeric"
                  class="form-input text-center"
                  placeholder="••••"
                  bind:value={pinConfirm}
                />
              </div>
            </div>
            {#if pinError}
              <p class="error-text" transition:fade>{pinError}</p>
            {/if}
          </div>
        {/if}

        <button type="button" class="btn btn-primary" onclick={handleFinish} disabled={isSubmitting}>
          {isSubmitting ? 'Создаем...' : 'Завершить настройку'}
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .onboarding-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: radial-gradient(circle at center, rgba(30, 27, 38, 0.98), #0c0a10);
    z-index: 99999;
    display: flex;
    justify-content: center;
    align-items: center;
    backdrop-filter: blur(20px);
  }

  .onboarding-card {
    width: 100%;
    max-width: 400px;
    padding: 36px 24px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 28px;
    box-shadow: 0 24px 80px rgba(0, 0, 0, 0.5);
    color: white;
  }

  .step-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .text-center {
    text-align: center;
    align-items: center;
  }

  .logo-spark {
    font-size: 4rem;
    display: block;
    margin-bottom: 8px;
  }

  .logo-title {
    font-size: 2.2rem;
    font-weight: 800;
    letter-spacing: 2px;
    background: linear-gradient(135deg, #a855f7, #6366f1);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 8px;
  }

  .tagline {
    font-size: 1.1rem;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.4;
  }

  .tagline-sub {
    font-size: 0.88rem;
    color: rgba(255, 255, 255, 0.5);
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 100%;
    margin-top: 16px;
  }

  h2 {
    font-size: 1.6rem;
    font-weight: 700;
    letter-spacing: 0.5px;
  }

  .step-desc {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.6);
    margin-top: -12px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
  }

  .form-label {
    font-size: 0.78rem;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.5);
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .form-input {
    width: 100%;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 14px 20px;
    color: white;
    font-size: 1.05rem;
    transition: all 0.25s ease;
  }

  .form-input:focus {
    outline: none;
    border-color: #a855f7;
    background: rgba(255, 255, 255, 0.08);
    box-shadow: 0 0 12px rgba(168, 85, 247, 0.2);
  }

  .text-center {
    text-align: center;
  }

  .btn {
    width: 100%;
    padding: 14px 24px;
    border-radius: 16px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
  }

  .btn-primary {
    background: linear-gradient(135deg, #a855f7, #6366f1);
    color: white;
  }

  .btn-primary:hover {
    transform: translateY(-1px);
    box-shadow: 0 8px 24px rgba(168, 85, 247, 0.35);
  }

  .btn-primary:active {
    transform: translateY(0);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.1);
    color: white;
  }

  .protection-options {
    display: flex;
    flex-direction: column;
    gap: 16px;
    width: 100%;
  }

  .option-card {
    display: flex;
    gap: 16px;
    padding: 16px;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 18px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .option-card:hover {
    background: rgba(255, 255, 255, 0.04);
    border-color: rgba(255, 255, 255, 0.15);
  }

  .option-card.selected {
    background: rgba(168, 85, 247, 0.08);
    border-color: #a855f7;
  }

  .option-card input[type="radio"] {
    display: none;
  }

  .option-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .option-title {
    font-size: 0.98rem;
    font-weight: 600;
  }

  .option-desc {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.5);
    line-height: 1.3;
  }

  .pin-inputs {
    background: rgba(0, 0, 0, 0.2);
    padding: 16px;
    border-radius: 18px;
    border: 1px solid rgba(255, 255, 255, 0.04);
  }

  .form-row {
    display: flex;
    gap: 16px;
  }

  .error-text {
    color: #ff5555;
    font-size: 0.8rem;
    margin-top: 8px;
    text-align: center;
  }
</style>
