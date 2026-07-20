package app.hroniki.mobile

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

class ReminderReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        when (intent.action) {
            Intent.ACTION_BOOT_COMPLETED -> {
                // System rebooted: alarms must be restored by platform scheduler subscriber
                android.util.Log.i("ReminderReceiver", "BOOT_COMPLETED received. Restoring registered alarms.")
            }
            AlarmBridge.ACTION_TRIGGER_REMINDER -> {
                val alarmId = intent.getStringExtra("alarm_id") ?: "0"
                val title = intent.getStringExtra("title") ?: "Напоминание ХРОНИКИ"
                val body = intent.getStringExtra("body")

                NotificationBridge.showNotification(
                    context,
                    alarmId.hashCode(),
                    title,
                    body
                )
            }
        }
    }
}
