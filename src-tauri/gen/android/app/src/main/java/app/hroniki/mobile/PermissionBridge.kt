package app.hroniki.mobile

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.annotation.Keep
import androidx.core.content.ContextCompat

@Keep
object PermissionBridge {

    fun checkNotificationPermission(context: Context): Int {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (ContextCompat.checkSelfPermission(
                    context,
                    Manifest.permission.POST_NOTIFICATIONS
                ) == PackageManager.PERMISSION_GRANTED
            ) {
                0 // Granted
            } else {
                1 // Denied
            }
        } else {
            0 // Granted by default pre-Android 13
        }
    }

    fun checkStoragePermission(context: Context): Int {
        return if (ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.READ_EXTERNAL_STORAGE
            ) == PackageManager.PERMISSION_GRANTED
        ) {
            0 // Granted
        } else {
            1 // Denied
        }
    }
}
