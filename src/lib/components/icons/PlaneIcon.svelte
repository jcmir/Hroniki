<script lang="ts">
  interface Props { size?: number; class?: string; }
  let { size = 20, class: className = '' }: Props = $props();
</script>
<svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
  <path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3.5c-.5-.5-2.5 0-4 1.5L13.5 8.5 5.3 6.7 3.5 8.5l8.5 3.5-3.5 3.5-3.5-.5L3.5 16.5l3.5 1.8 1.8 3.5 1.5-1.5-.5-3.5 3.5-3.5 3.5 8.5Z" />
</svg>
