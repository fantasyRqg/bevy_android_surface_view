#include <jni.h>
#include <string>
#include <android/native_window.h>
#include <android/native_window_jni.h>


#include "log.h"

#define LOG_TAG "NativeBridge"

extern "C" void surfaceRedrawNeeded();
extern "C" void surfaceCreated(ANativeWindow *pWindow);
extern "C" void surfaceChanged(uint32_t width, uint32_t height);
extern "C" void surfaceDestroyed();
extern "C" void runGameLoop();
extern "C" void stopGame();
extern "C" void touchEvent(uint32_t pointerId, uint32_t action, float x, float y);
extern "C" void onResume();
extern "C" void onPause();


extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceRedrawNeeded(JNIEnv *env, jobject thiz) {
    surfaceRedrawNeeded();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceCreated(JNIEnv *env, jobject thiz, jobject surface) {
    auto win = ANativeWindow_fromSurface(env, surface);
    surfaceCreated(win);
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceChanged(JNIEnv *env, jobject thiz, jint width, jint height) {
    surfaceChanged(width, height);
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceDestroyed(JNIEnv *env, jobject thiz) {
    surfaceDestroyed();
}

extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_onResume(JNIEnv *env, jobject thiz) {
    onResume();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_onPause(JNIEnv *env, jobject thiz) {
    onPause();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_runGameLoop(JNIEnv *env, jobject thiz) {
    runGameLoop();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_stopGame(JNIEnv *env, jobject thiz) {
    stopGame();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_touchEvent(JNIEnv *env, jobject thiz, jint pointer_id, jint acton, jfloat x, jfloat y) {
    touchEvent(pointer_id, acton, x, y);
}