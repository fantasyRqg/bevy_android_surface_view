#include <jni.h>
#include <string>
#include <android/native_window.h>
#include <android/native_window_jni.h>


#include "log.h"

#define LOG_TAG "NativeBridge"

extern "C" void surfaceRedrawNeeded();
extern "C" void surfaceCreated(ANativeWindow *pWindow);
extern "C" void surfaceChanged(int width, int height);
extern "C" void surfaceDestroyed();
extern "C" void gameStart();
extern "C" void gameStop();
extern "C" void touchEvent(float x, float y);
extern "C" void onResume();
extern "C" void onPause();
extern "C" int test();


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
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_gameStart(JNIEnv *env, jobject thiz) {
    gameStart();
    ALOGD("start game %d", test());

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_gameStop(JNIEnv *env, jobject thiz) {
    gameStop();
    ALOGD("stop game %d", test());

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_touchEvent(JNIEnv *env, jobject thiz, jfloat x, jfloat y) {
    touchEvent(x, y);
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