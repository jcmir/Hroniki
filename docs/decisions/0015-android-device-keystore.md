# ADR 0015: Неэкспортируемость мастер-ключей Android Keystore и модель StrongBox

## Контекст и Проблема
При интеграции с системным аппаратно-защищенным хранилищем Android Keystore важно строго зафиксировать границу ответственности. Мастер-ключ шифрования `master_key` никогда не должен покидать защищенный чип устройства.

## Решение
1. **Правило неэкспортируемости (Non-exportability Rule)**:
   - Ключ генерируется внутри чипа TEE или StrongBox Keymaster.
   - Мастер-ключ помечается флагом `KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT`.
   - Ключ **не является экпортируемым** (`isSetUserAuthenticationRequired`, `setIsStrongBoxBacked`).
   - Rust Core никогда не имеет доступа к сырым байтам мастер-ключа.
2. **Аппаратные возможности StrongBox**:
   - `PlatformCapabilities` расширяется полем `strongbox: bool`.
   - Если девайс поддерживает физический модуль StrongBox (Hardware Security Module), ключ генерируется в нем. В противном случае Kotlin-плагин осуществляет прозрачный fallback на системный TEE с изменением флага `strongbox = false`.
3. **Безопасное управление потоками JNI**:
   - В Rust удерживается исключительно `Arc<JavaVM>`.
   - Потоки получают `JNIEnv` через метод `attach_current_thread()`.
   - Все исключения нативной JVM сбрасываются методом `exception_clear()` с возвратом ошибок `KeyStoreError::JavaException`.

## Последствия
- Полная невосприимчивость к дампам оперативной памяти процесса Rust.
- Прозрачное управление hardware-capabilities на различных версиях Android.
