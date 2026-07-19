<script lang="ts">
  interface Props {
    size?: number;
    class?: string;
  }

  let { size = 48, class: className = '' }: Props = $props();
</script>

<svg
  class={className}
  width={size}
  height={size}
  viewBox="0 0 100 100"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <defs>
    <linearGradient id="chronicleLogoGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6025ff" />
      <stop offset="100%" stop-color="#ff5555" />
    </linearGradient>
    <filter id="logoShadow" x="-10%" y="-10%" width="120%" height="120%">
      <feDropShadow dx="0" dy="4" stdDeviation="6" flood-color="#6025ff" flood-opacity="0.15" />
    </filter>
  </defs>

  <!-- Branded Outer circular timeline path -->
  <circle
    cx="50"
    cy="50"
    r="38"
    stroke="url(#chronicleLogoGrad)"
    stroke-width="5.5"
    stroke-linecap="round"
    stroke-dasharray="165 70"
    filter="url(#logoShadow)"
  />

  <!-- Curved timeline flow path inside -->
  <path
    d="M26 62 C 36 38, 64 38, 74 62"
    stroke="url(#chronicleLogoGrad)"
    stroke-width="4.5"
    stroke-linecap="round"
    opacity="0.95"
  />

  <!-- Chronicle event milestone point -->
  <circle
    cx="50"
    cy="42"
    r="7.5"
    fill="url(#chronicleLogoGrad)"
  />
  <circle
    cx="50"
    cy="42"
    r="12.5"
    stroke="url(#chronicleLogoGrad)"
    stroke-width="1.8"
    opacity="0.3"
  />
</svg>
