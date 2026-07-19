# HRONIKI Design System Specification ("Было!")

Документ является единственным источником UI-стандартов приложения ХРОНИКИ («Apple Photos + Day One + Notion + Семейный архив»).

---

## 1. Цветовая Палитра (Color Palette)

- **Основной фон (Background)**: `#FCF7FA` (Мягкий светлый пастельный фон)
- **Основной фиолетовый акцент (Primary)**: `#7C3AED`
- **Розовый акцент (Accent Pink)**: `#EC4899`
- **Изумрудный акцент (Success/Active)**: `#10B981`
- **Синий акцент (Info)**: `#3B82F6`
- **Янтарный акцент (Amber Warm)**: `#F59E0B`
- **Основной текст (Text Primary)**: `#171717`
- **Вторичный текст (Text Secondary)**: `#737373`

---

## 2. Стеклянные Карточки (GlassCard Tokens)

- **Border Radius**: `24px` (`var(--radius-card)`)
- **Background**: `rgba(255, 255, 255, 0.72)`
- **Backdrop Filter**: `blur(20px)`
- **Border**: `1px solid rgba(255, 255, 255, 0.6)`
- **Shadow**: `0 8px 32px rgba(124, 58, 237, 0.08)` (`var(--shadow-soft)`)

---

## 3. Типографика (Typography)

- **Заголовки экранов**: `Outfit` / `28px` / `SemiBold (600)`
- **Заголовки секций**: `Outfit` / `20px` / `SemiBold (600)`
- **Основной текст**: `Inter` / `16px` / `Regular (400)`
- **Метаданные и теги**: `Inter` / `13px` / `Medium (500)`

---

## 4. Канонические Маршруты (Routes)

1. `/` — Главный экран Ленты Хроник (Timeline)
2. `/objects` — Сетка Жизненных Объектов (Objects Grid)
3. `/object/[id]` — История Владения и Развития Объекта (Object History)
4. `/settings` — Настройки Профиля, Безопасности и Бэкапов (Settings)

---

## 5. Правило Архитектуры Вызовов

`UI Component` ➔ `Store` ➔ `IPC` ➔ `Rust Core`. Прямые вызовы `invoke(...)` из визуальных компонентов запрещены.
