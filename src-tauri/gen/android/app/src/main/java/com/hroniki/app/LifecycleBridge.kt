package com.hroniki.app

import androidx.annotation.Keep

@Keep
object LifecycleBridge {
    external fun onPause()
    external fun onResume()
    external fun onDestroy()
    external fun onTrimMemory()
    external fun onLocked()
}
