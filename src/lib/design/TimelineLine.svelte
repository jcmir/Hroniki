<script lang="ts">
  export let active: boolean = true;
</script>

<div class="timeline-line-container">
  <div class="node-dot" class:active></div>
  <div class="vertical-line"></div>
</div>

<style>
  .timeline-line-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 24px;
    margin-right: 0.75rem;
  }

  .node-dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background-color: var(--text-subtle);
    border: 3px solid var(--bg-app);
    box-shadow: 0 0 0 2px var(--border-subtle);
    z-index: 2;
    transition: all 0.2s ease;
  }

  .node-dot.active {
    background-color: var(--accent-primary);
    box-shadow: 0 0 0 3px var(--accent-primary-glow, rgba(124, 58, 237, 0.2));
  }

  .vertical-line {
    flex: 1;
    width: 2px;
    background: linear-gradient(180deg, var(--accent-primary), rgba(124, 58, 237, 0.1));
    margin-top: 4px;
  }
</style>
