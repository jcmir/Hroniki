<script lang="ts">
  interface Props { size?: number; class?: string; }
  let { size = 20, class: className = '' }: Props = $props();
</script>
<svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="5" r="2" />
  <circle cx="5.5" cy="9" r="2" />
  <circle cx="18.5" cy="9" r="2" />
  <circle cx="8" cy="16" r="3" />
  <circle cx="16" cy="16" r="3" />
</svg>
