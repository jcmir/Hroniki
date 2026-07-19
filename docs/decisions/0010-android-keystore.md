# ADR 0010: Архитектура Android Keystore и Изоляция SDK

## Контекст и Проблема
Интеграция с Android SDK (Keystore API, Cipher, Activity lifecycle) требует вызова Java-методов из Rust-процесса. Прямое связывание бизнес-логики с JNI-вызовами нарушает переносимость кода и усложняет сборку на десктопных ОС.

## Решение
Мы фиксируем правило: **Android SDK полностью изолируется за платформенными адаптерами (Platform Adapters)**.
1. **Порты (Traits)**: Доменная логика обращается только к трейтам в `platform/`.
2. **Адаптеры**: Реализации вызовов JNI/Kotlin изолируются в подкаталоге `platform/adapters/android/`.
3. **Версионирование Секретов (`WrappedSecret`)**: Секреты оборачиваются и хранятся локально в файлах в виде структуры, поддерживающей смену алгоритмов шифрования:
   ```rust
   pub struct WrappedSecret {
       pub version: u32,
       pub algorithm: String,
       pub ciphertext: Vec<u8>,
   }
   ```
4. **Аппаратный Ключ (Master Key)**: Хранится внутри Android Keystore (TEE/StrongBox) и используется исключительно для шифрования и дешифрования локальных ключей (Database Encryption Key, Session Token, Recovery Key) через AES-GCM-NoPadding.
5. **Абстракция Жизненного Цикла**: События ОС маппятся на нейтральные платформенные события:
   ```rust
   pub enum PlatformLifecycleEvent {
       Background,
       Foreground,
       Terminating,
       MemoryPressure,
   }
   ```

## Последствия
- Бизнес-логика Хроник полностью защищена от утечки JNI/Android вызовов в доменную область.
- Возможность безопасной миграции алгоритмов шифрования и версий API.
