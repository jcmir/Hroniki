export function pluralRu(n: number, one: string, few: string, many: string): string {
  const mod10 = n % 10, mod100 = n % 100;
  if (mod10 === 1 && mod100 !== 11) return one;
  if (mod10 >= 2 && mod10 <= 4 && (mod100 < 10 || mod100 >= 20)) return few;
  return many;
}

export function formatAge(createdAt: string | undefined, short = false): string {
  if (!createdAt) return '';
  const ms = Date.now() - new Date(createdAt).getTime();
  const totalDays = Math.floor(ms / 86_400_000);
  if (totalDays < 1) return short ? 'сегодня' : 'только что';

  const years = Math.floor(totalDays / 365);
  const months = Math.floor((totalDays % 365) / 30);
  const days = totalDays % 30;

  const parts: string[] = [];
  if (years > 0) {
    parts.push(`${years} ${pluralRu(years, 'год', 'года', 'лет')}`);
  }
  if (months > 0) {
    parts.push(short ? `${months} мес` : `${months} ${pluralRu(months, 'месяц', 'месяца', 'месяцев')}`);
  }
  if (years === 0 && months === 0) {
    if (short) {
      parts.push(`${totalDays % 30} дн`);
    } else if (days > 0) {
      parts.push(`${days} ${pluralRu(days, 'день', 'дня', 'дней')}`);
    }
  }
  return parts.join(' ');
}

export function formatDateRu(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' });
}
