use std::os::raw::c_uint;
use std::ptr::NonNull;

use bevy::input::touch::TouchPhase;
use bevy::log::info;
use bevy::math::vec2;
use bevy::prelude::TouchInput;
use bevy::utils::info;
use jni_sys::{JavaVM, JNIEnv, jobject};
use ndk::event::MotionAction;
use ndk::native_window::NativeWindow;
use ndk_sys::ANativeWindow;

use crate::{Cmd, init_command_queue, run_game_loop};


#[no_mangle]
pub extern "C" fn initCommandQueue() {
    init_command_queue();
}

#[no_mangle]
pub extern "C" fn surfaceRedrawNeeded() {
    //ignore this
}

static mut CONTEXT_INITIALIZED: bool = false;

#[no_mangle]
pub extern "C" fn surfaceCreated(
    window: *mut ANativeWindow,
    vm: *mut JavaVM,
    activity: jobject,
) {
    unsafe {
        if CONTEXT_INITIALIZED {
            info!("surfaceCreated called again, release the old context first");
            ndk_context::release_android_context();
        }
        ndk_context::initialize_android_context(vm.cast(), activity.cast());
        info!("surfaceCreated called, initialize the context");
        CONTEXT_INITIALIZED = true;
        let window = NativeWindow::from_ptr(NonNull::new(window).unwrap());
        Cmd::SurfaceCreated(window).send();
    }
}


#[no_mangle]
pub extern "C" fn surfaceChanged(
    width: u32, height: u32,
) {
    Cmd::SurfaceChanged(width, height).send();
}


#[no_mangle]
pub extern "C" fn surfaceDestroyed() {
    Cmd::SurfaceDestroyed.send();
}

#[no_mangle]
pub extern "C" fn runGameLoop() {
    run_game_loop();
    unsafe {
        ndk_context::release_android_context();
        CONTEXT_INITIALIZED = false;
    }
}

#[no_mangle]
pub extern "C" fn stopGame() {
    Cmd::StopGame.send();
}

#[no_mangle]
pub extern "C" fn touchEvent(pointer_id: i32, action: i32, x: f32, y: f32) {
    let action = (action as u32).try_into().unwrap();
    let phase = match action {
        MotionAction::PointerDown | MotionAction::Down => Some(TouchPhase::Started),
        MotionAction::Up | MotionAction::PointerUp => Some(TouchPhase::Ended),
        MotionAction::Move => Some(TouchPhase::Moved),
        MotionAction::Cancel => Some(TouchPhase::Canceled),
        _ => { None }
    };

    if let Some(phase) = phase {
        Cmd::TouchEvent(TouchInput {
            phase,
            position: vec2(x, y),
            force: None,
            id: pointer_id as u64,
        }).send();
    }
}

#[no_mangle]
pub extern "C" fn onResume() {
    Cmd::OnResume.send();
}

#[no_mangle]
pub extern "C" fn onPause() {
    Cmd::OnPause.send();
}


