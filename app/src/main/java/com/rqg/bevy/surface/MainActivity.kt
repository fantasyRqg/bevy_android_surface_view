package com.rqg.bevy.surface

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.MotionEvent
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.View
import android.widget.Button

class MainActivity : AppCompatActivity() {

    private val mSurfaceView: SurfaceView by lazy {
        findViewById(R.id.surface_view)
    }

    private val mBtnToggle: Button by lazy {
        findViewById(R.id.btn_toggle)
    }

    private var gameStarted = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)


        setContentView(R.layout.activity_main)

        mBtnToggle.text = "Start"
        mBtnToggle.setOnClickListener {
            if (gameStarted) {
                NativeBridge.gameStop()
                mBtnToggle.text = "Start"
            } else {
                NativeBridge.gameStart()
                mBtnToggle.text = "Stop"
            }
        }

        mSurfaceView.holder.addCallback(surfaceCallback)

        mSurfaceView.setOnTouchListener(surfaceTouchListener)

    }


    private val surfaceTouchListener = View.OnTouchListener { _, event ->
        NativeBridge.touchEvent(event.x, event.y)

        return@OnTouchListener true
    }

    private val surfaceCallback = object : SurfaceHolder.Callback2 {
        override fun surfaceRedrawNeeded(holder: SurfaceHolder) {
            // Tell the native renderer that the surface has been redrawn.
            NativeBridge.surfaceRedrawNeeded()
        }

        override fun surfaceCreated(holder: SurfaceHolder) {
            // Tell the native renderer that a surface has been created.
            NativeBridge.surfaceCreated(holder.surface)
        }

        override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
            // Tell the native renderer that the surface has changed.
            NativeBridge.surfaceChanged(width, height)
        }

        override fun surfaceDestroyed(holder: SurfaceHolder) {
            // Tell the native renderer that the surface has been destroyed.
            NativeBridge.surfaceDestroyed()
        }
    }


}