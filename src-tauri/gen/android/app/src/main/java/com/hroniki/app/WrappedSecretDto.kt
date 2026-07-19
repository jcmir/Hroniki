package com.hroniki.app

import androidx.annotation.Keep

@Keep
data class WrappedSecretDto(
    val schemaVersion: Int = 1,
    val version: Int = 1,
    val algorithm: String = "AES-GCM-NoPadding",
    val nonce: String,      // Base64 encoded 12-byte IV
    val ciphertext: String, // Base64 encoded payload
    val tag: String         // Base64 encoded 16-byte authentication tag
)
