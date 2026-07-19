# Документация модуля: Reminders (Напоминания)

Модуль `reminder` обеспечивает планирование и обработку напоминаний, связанных с записями хроники или автономных уведомлений. Он разработан с учетом слабой связанности (Loose Coupling) и легко масштабируется на любые внешние платформы доставки (Tauri desktop notifications, Mobile push, SMS/Email).

---

## 1. Архитектура и Жизненный цикл

Вся логика планирования отделена от системных вызовов ОС через абстракцию `NotificationProvider`. Фоновый поток `ReminderScheduler` периодически сканирует хранилище на предмет наступивших напоминаний в статусе `Pending` или `Scheduled`, переводит их статус в `Triggered` (с использованием пессимистичной/оптимистичной блокировки во избежание дублирования) и делегирует отправку провайдеру.

### Жизненный цикл напоминания:
1. **`Pending`**: напоминание создано пользователем и ожидает наступления целевого времени.
2. **`Scheduled`**: напоминание зафиксировано планировщиком (опциональное промежуточное состояние для распределенных инстансов).
3. **`Triggered`**: время пришло, уведомление отправлено в ОС/канал связи.
4. **`Completed`**: пользователь вручную подтвердил выполнение напоминания.
5. **`Cancelled`**: напоминание было отменено до наступления времени срабатывания.

---

## 2. Domain Models

- `Reminder`: Основная сущность напоминания.
- `ReminderStatus`: Перечисление состояний жизненного цикла напоминания (`Pending`, `Scheduled`, `Triggered`, `Completed`, `Cancelled`).
- `RecurrenceRule`: Правила повторения напоминаний (`Once`, `Daily`, `Weekly`, `Monthly`).

---

## 3. Interfaces

### `ReminderRepository`
Абстрагирует операции чтения и записи сущностей напоминаний в хранилище.

```rust
#[async_trait]
pub trait ReminderRepository: Send + Sync {
    async fn save(&self, reminder: &Reminder) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Reminder>, String>;
    async fn get_active_reminders(&self) -> Result<Vec<Reminder>, String>;
    async fn update_status(&self, id: &str, old: ReminderStatus, new: ReminderStatus) -> Result<bool, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}
```

### `NotificationProvider`
Абстрагирует канал отправки уведомления.

```rust
#[async_trait]
pub trait NotificationProvider: Send + Sync {
    async fn send(&self, title: &str, body: Option<&str>) -> Result<(), String>;
}
```

---

## 4. Фоновые процессы

### `ReminderScheduler`
Фоновый цикл (`tokio::spawn`), который выполняет проверку ("тики") времени каждую минуту. При наступлении целевого времени `trigger_at`:
1. Блокирует запись в БД (изменяя статус `Pending -> Triggered`).
2. При успешной смене статуса вызывает `NotificationProvider::send`.

### `ReminderSubscriber`
Асинхронный подписчик на шину `EventBus`. Реагирует на события домена (например, удаление записи хроники должно автоматически переводить связанные напоминания в статус `Cancelled`).
