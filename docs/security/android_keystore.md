# Спецификация безопасности: Android Keystore и Key Wrapping

Этот документ описывает схему защиты мастер-ключей и версионирования секретов в приложении ХРОНИКИ.

---

## 1. Двухуровневая модель шифрования

```
[ TEE / StrongBox (Android Keystore) ]
                  │
          (Master Key - MK)
                  │
                  ▼
        [ AES-GCM Key Wrap ]  ◄─── (Database Encryption Key - DEK)
                  │
                  ▼
   [ WrappedSecret (Local File) ]
```

1. **Аппаратный ключ (Master Key)**: Постоянно находится внутри защищенной памяти Android Keystore (под защитой TEE или StrongBox чипа).
2. **Локальный шифрованный файл (Wrapped Key)**: Зашифрованный DEK хранится в приватной директории приложения в файловой системе Android (`/data/data/com.hroniki/files/keys/db.key.enc`).

---

## 2. Структура WrappedSecret (Версионирование)

Для предотвращения жесткой привязки к алгоритмам при смене поколений шифрования, все шифрованные локальные ключи сохраняются в виде структуры:

```rust
pub struct WrappedSecret {
    pub version: u32,       // Номер версии схемы (1 = AES-256-GCM)
    pub algorithm: String, // Идентификатор алгоритма (например, "AES-GCM-NoPadding")
    pub ciphertext: Vec<u8>, // Ciphertext + IV (Initialization Vector) + Auth Tag
}
```

Это позволяет приложении в будущем легко поддерживать новые версии алгоритмов (например, AES-GCM-SIV или ChaCha20-Poly1305), считывая метаданные из структуры `WrappedSecret` и прогоняя миграцию при загрузке.
