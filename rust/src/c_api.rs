use std::ffi::c_char;
use ndk::native_window::NativeWindow;
use ndk_sys::ANativeWindow;
use android_log_sys;
use android_log_sys::__android_log_print;


#[no_mangle]
pub extern "C" fn test()-> i32 {
    return 10037;
}
#[no_mangle]
pub extern "C" fn surfaceRedrawNeeded() {}

#[no_mangle]
pub extern "C" fn surfaceCreated(
    window: *mut ANativeWindow,
) {}


#[no_mangle]
pub extern "C" fn surfaceChanged(
    width: i32, height: i32,
) {}


#[no_mangle]
pub extern "C" fn surfaceDestroyed() {}

#[no_mangle]
pub extern "C" fn gameStart() {
}

#[no_mangle]
pub extern "C" fn gameStop() {
}

#[no_mangle]
pub extern "C" fn touchEvent(x: f32, y: f32) {}

#[no_mangle]
pub extern "C" fn onResume() {}

#[no_mangle]
pub extern "C" fn onPause() {}


