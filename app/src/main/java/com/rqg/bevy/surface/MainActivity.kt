package com.rqg.bevy.surface

import android.annotation.SuppressLint
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.os.Handler
import android.util.Log
import android.view.MotionEvent
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.View
import android.widget.Button

class MainActivity : AppCompatActivity() {
    companion object {
        private const val TAG = "MainActivity"
    }

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
                stopGame()
                mBtnToggle.text = "Start"
                gameStarted = false
            } else {
                startGame()
                mBtnToggle.text = "Stop"
                gameStarted = true
            }
        }

        mSurfaceView.holder.addCallback(surfaceCallback)

        mSurfaceView.setOnTouchListener(surfaceTouchListener)

        NativeBridge.initCommandQueue()
    }

    private var gameThread: Thread? = null
    fun startGame() {
        Log.d(TAG, "startGame() called")
        if (gameThread != null) {
            stopGame()
        }

        gameThread = Thread {
            NativeBridge.runGameLoop()
        }
        gameThread?.start()

        // Tell the native renderer that a surface has been created.
        if (mSurfaceView.holder.surface.isValid) {
            Log.d(TAG, "startGame: surface is valid")
            NativeBridge.surfaceCreated(mSurfaceView.holder.surface, this)
            val frame = mSurfaceView.holder.surfaceFrame
            NativeBridge.surfaceChanged(frame.width(), frame.height())
        }
    }

    fun stopGame() {
        Log.d(TAG, "stopGame() called")
        NativeBridge.stopGame()
        gameThread?.join()
        gameThread = null
        Log.d(TAG, "stopGame() finished")
    }

    override fun onResume() {
        Log.d(TAG, "onResume() called")
        super.onResume()
        NativeBridge.onResume()
    }

    override fun onPause() {
        Log.d(TAG, "onPause() called")
        super.onPause()
        NativeBridge.onPause()
    }


    @SuppressLint("ClickableViewAccessibility")
    private val surfaceTouchListener = View.OnTouchListener { _, event ->

        val pointerId = event.getPointerId(event.actionIndex)
        val action = event.actionMasked
        MotionEvent.ACTION_DOWN
        NativeBridge.touchEvent(pointerId, action, event.x, event.y)
        return@OnTouchListener true
    }

    private val surfaceCallback = object : SurfaceHolder.Callback2 {
        override fun surfaceRedrawNeeded(holder: SurfaceHolder) {
            // Tell the native renderer that the surface has been redrawn.
            NativeBridge.surfaceRedrawNeeded()
        }

        override fun surfaceCreated(holder: SurfaceHolder) {
            // Tell the native renderer that a surface has been created.
            NativeBridge.surfaceCreated(holder.surface, this@MainActivity)
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