# Документация модуля: Secure Storage

Модуль `Secure Storage` обеспечивает безопасное локальное хранение критически важных секретов приложения ХРОНИКИ.

---

## 1. Схема Делегации Вызовов

```
[ AndroidSecureStoragePlatform ]
               │
               ▼
      [ KeyStoreBackend ] (Трейт)
               │
      ┌────────┴────────┐
      ▼                 ▼
[ MemoryBackend ]   [ JniBackend ]
```

Адаптер `AndroidSecureStoragePlatform` делегирует криптографические операции выбранному при инициализации бэкенду `KeyStoreBackend`.

---

## 2. Типы ошибок (KeyStoreError)

Все сбои бэкенда типизированы для точного разграничения проблем:

- `InvalidVersion(u32)`: Попытка загрузить секрет, версия схемы которого не поддерживается текущим кодом.
- `EncryptionFailed(String)`: Сбой алгоритма шифрования или инициализации Cipher.
- `DecryptionFailed`: Ошибка аутентификации блока данных (неверный GCM Auth Tag).
- `BackendUnavailable`: Отсутствие ответа от системной службы Keystore или JNI-окружения.
- `InvalidSecretFormat`: Ошибка десериализации или повреждение структуры `WrappedSecret`.
