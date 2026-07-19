<script lang="ts">
  export let photos: Array<{ id: string; file_path: string }> = [];
</script>

{#if photos && photos.length > 0}
  <div class="photo-grid" class:single={photos.length === 1} class:multi={photos.length > 1}>
    {#each photos.slice(0, 4) as photo (photo.id)}
      <div class="photo-thumb">
        <img src={photo.file_path} alt="Вложение хроники" loading="lazy" />
      </div>
    {/each}
  </div>
{/if}

<style>
  .photo-grid {
    display: grid;
    gap: 0.5rem;
    margin-top: 0.75rem;
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .photo-grid.single {
    grid-template-columns: 1fr;
    max-height: 240px;
  }

  .photo-grid.multi {
    grid-template-columns: repeat(2, 1fr);
    max-height: 200px;
  }

  .photo-thumb {
    width: 100%;
    height: 100%;
    overflow: hidden;
    border-radius: var(--radius-sm);
    background-color: rgba(0, 0, 0, 0.05);
  }

  .photo-thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;
  }

  .photo-thumb:hover img {
    transform: scale(1.04);
  }
</style>
