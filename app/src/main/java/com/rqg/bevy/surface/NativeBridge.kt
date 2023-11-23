package com.rqg.bevy.surface

import android.view.Surface

/**
 * * Created by rqg on 2023/11/23.
 */
class NativeBridge {

    /**
     * A native method that is implemented by the 'surface' native library,
     * which is packaged with this application.
     */


    companion object {
        external fun surfaceRedrawNeeded();

        external fun surfaceCreated(surface: Surface)

        external fun surfaceChanged(width: Int, height: Int)

        external fun surfaceDestroyed()

        external fun gameStart()

        external fun gameStop()

        fun touchEvent(x: Float, y: Float) {

        }

        // Used to load the 'surface' library on application startup.
        init {
            System.loadLibrary("surface")
        }
    }
}