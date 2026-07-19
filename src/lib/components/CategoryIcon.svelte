<script lang="ts">
  import SproutIcon from './icons/SproutIcon.svelte';
  import CarIcon from './icons/CarIcon.svelte';
  import HouseIcon from './icons/HouseIcon.svelte';
  import HeartIcon from './icons/HeartIcon.svelte';
  import PawIcon from './icons/PawIcon.svelte';
  import FileIcon from './icons/FileIcon.svelte';
  import ActivityIcon from './icons/ActivityIcon.svelte';
  import BriefcaseIcon from './icons/BriefcaseIcon.svelte';
  import MonitorIcon from './icons/MonitorIcon.svelte';
  import PlaneIcon from './icons/PlaneIcon.svelte';
  import BellIcon from './icons/BellIcon.svelte';
  import SparklesIcon from './icons/SparklesIcon.svelte';

  interface Props {
    categoryName: string;
    class?: string;
    size?: number;
  }

  let { categoryName, class: className = '', size = 20 }: Props = $props();

  const name = $derived(categoryName.trim());

  const iconComponent = $derived(() => {
    switch (name) {
      case 'Сад':
      case 'Садоводство':
        return SproutIcon;
      case 'Авто':
      case 'Автомобиль':
        return CarIcon;
      case 'Дом':
      case 'Жилье':
        return HouseIcon;
      case 'Здоровье':
        return HeartIcon;
      case 'Питомцы':
      case 'Питомец':
        return PawIcon;
      case 'Документы':
      case 'Документ':
        return FileIcon;
      case 'Спорт':
      case 'Фитнес':
        return ActivityIcon;
      case 'Работа':
      case 'Дела':
        return BriefcaseIcon;
      case 'Техника':
      case 'Гаджеты':
        return MonitorIcon;
      case 'Путешествия':
      case 'Поездки':
        return PlaneIcon;
      case 'Напоминания':
      case 'Напоминание':
        return BellIcon;
      default:
        return SparklesIcon;
    }
  });
</script>

{#if iconComponent()}
  {@const SelectedIcon = iconComponent()}
  <SelectedIcon class={className} {size} />
{/if}

