package app.hroniki.mobile

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Base64
import androidx.annotation.Keep
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

@Keep
object KeyStoreBridge {
    private const val ANDROID_KEYSTORE = "AndroidKeyStore"
    private const val KEY_ALIAS = "HronikiMasterKey"
    private const val TRANSFORMATION = "AES/GCM/NoPadding"
    private const val GCM_TAG_LENGTH = 128

    private var isStrongBoxBacked: Boolean = false

    fun isStrongBoxAvailable(): Boolean = isStrongBoxBacked

    @Synchronized
    fun initialize(): Boolean {
        return try {
            val keyStore = KeyStore.getInstance(ANDROID_KEYSTORE)
            keyStore.load(null)
            if (!keyStore.containsAlias(KEY_ALIAS)) {
                generateMasterKey()
            }
            true
        } catch (e: Exception) {
            false
        }
    }

    private fun generateMasterKey() {
        // Try StrongBox first, fallback to standard TEE if unavailable
        try {
            createKey(useStrongBox = true)
            isStrongBoxBacked = true
        } catch (e: Exception) {
            createKey(useStrongBox = false)
            isStrongBoxBacked = false
        }
    }

    private fun createKey(useStrongBox: Boolean) {
        val keyGenerator = KeyGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_AES,
            ANDROID_KEYSTORE
        )

        val builder = KeyGenParameterSpec.Builder(
            KEY_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(256)

        if (useStrongBox) {
            builder.setIsStrongBoxBacked(true)
        }

        keyGenerator.init(builder.build())
        keyGenerator.generateKey()
    }

    private fun getSecretKey(): SecretKey {
        val keyStore = KeyStore.getInstance(ANDROID_KEYSTORE)
        keyStore.load(null)
        return keyStore.getKey(KEY_ALIAS, null) as SecretKey
    }

    fun encryptBytes(plaintext: ByteArray): WrappedSecretDto {
        val secretKey = getSecretKey()
        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(Cipher.ENCRYPT_MODE, secretKey)

        val iv = cipher.iv
        val encrypted = cipher.doFinal(plaintext)

        // Split ciphertext and 16-byte authentication tag
        val ciphertextLen = encrypted.size - 16
        val ciphertextBytes = encrypted.copyOfRange(0, ciphertextLen)
        val tagBytes = encrypted.copyOfRange(ciphertextLen, encrypted.size)

        return WrappedSecretDto(
            schemaVersion = 1,
            version = 1,
            algorithm = "AES-GCM-NoPadding",
            nonce = Base64.encodeToString(iv, Base64.NO_WRAP),
            ciphertext = Base64.encodeToString(ciphertextBytes, Base64.NO_WRAP),
            tag = Base64.encodeToString(tagBytes, Base64.NO_WRAP)
        )
    }

    fun decryptBytes(dto: WrappedSecretDto): ByteArray {
        val secretKey = getSecretKey()
        val cipher = Cipher.getInstance(TRANSFORMATION)

        val iv = Base64.decode(dto.nonce, Base64.NO_WRAP)
        val ciphertextBytes = Base64.decode(dto.ciphertext, Base64.NO_WRAP)
        val tagBytes = Base64.decode(dto.tag, Base64.NO_WRAP)

        // Combine ciphertext and auth tag for GCM cipher
        val payload = ciphertextBytes + tagBytes

        val spec = GCMParameterSpec(GCM_TAG_LENGTH, iv)
        cipher.init(Cipher.DECRYPT_MODE, secretKey, spec)

        return cipher.doFinal(payload)
    }
}
