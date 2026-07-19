# Документация модуля: Session Security (Управление Блокировкой и Разблокировкой)

Модуль `session_security` реализует конечный автомат состояний сессии (`SessionState`) и обеспечивает стирание sensitive данных в RAM при блокировке устройства.

---

## 1. Конечный Автомат Состояний (`SessionState`)

| Состояние | Описание |
|---|---|
| `Active` | Сессия активна, токены и кэш доступны в RAM. |
| `Locked` | Экран заблокирован. Все RAM-токены и кэши очищены. |
| `AwaitingUnlock` | Приложение развернуто после блокировки, ожидает успешной аутентификации (PIN / биометрия). |

---

## 2. Поток Событий

```
ApplicationLocked  ──► SessionManager ──► Transition to Locked & clear RAM
ApplicationResumed ──► SessionManager ──► Transition to AwaitingUnlock & publish AuthenticationRequired
Auth Success       ──► SessionManager ──► Transition to Active & publish SessionRestored
```

### Гарантии Безопасности:
- При блокировке экрана (`ApplicationLocked`) RAM-токены и расшифрованный кэш очищаются моментально.
- Ключи KeyStore и зашифрованный файл базы данных SQLite на диске **не удаляются**.
- При разворачивании (`ApplicationResumed`) доступ к данным заблокирован до тех пор, пока пользователь не прошел аутентификацию.
