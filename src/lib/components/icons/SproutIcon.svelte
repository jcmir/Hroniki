<script lang="ts">
  interface Props { size?: number; class?: string; }
  let { size = 20, class: className = '' }: Props = $props();
</script>
<svg class={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 22C12 22 20 18 20 12C20 7.5 17.5 4 12 2C6.5 4 4 7.5 4 12C4 18 12 22 12 22Z" />
  <path d="M12 2V22" />
  <path d="M12 12C12 12 16 10 18 12" />
  <path d="M12 15C12 15 8 13 6 15" />
</svg>
