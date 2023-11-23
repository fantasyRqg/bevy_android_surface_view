#include <jni.h>
#include <string>
#include <android/native_window.h>

extern "C" void rust_function();


extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceRedrawNeeded(JNIEnv *env, jobject thiz) {

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceCreated(JNIEnv *env, jobject thiz, jobject surface) {

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceChanged(JNIEnv *env, jobject thiz, jint width, jint height) {

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_surfaceDestroyed(JNIEnv *env, jobject thiz) {

}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_gameStart(JNIEnv *env, jobject thiz) {
    rust_function();
}
extern "C"
JNIEXPORT void JNICALL
Java_com_rqg_bevy_surface_NativeBridge_00024Companion_gameStop(JNIEnv *env, jobject thiz) {

}