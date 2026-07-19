export const CATEGORY_ICONS: Record<string, string> = {
  'Сад':        '🌿',
  'Здоровье':   '❤️',
  'Авто':       '🚗',
  'Автомобиль': '🚗',
  'Дом':        '🏠',
  'Питомцы':    '🐾',
  'Питомец':    '🐾',
  'Документы':  '📄',
  'Спорт':      '🏃',
  'Работа':     '💼',
  'Техника':    '💻',
  'Путешествия':'✈️',
};

export function getCategoryIcon(categoryName: string): string {
  return CATEGORY_ICONS[categoryName] ?? '📦';
}
