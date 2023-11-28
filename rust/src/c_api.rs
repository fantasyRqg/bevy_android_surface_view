use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::fd::{FromRawFd, RawFd};
use std::ptr::NonNull;

use bevy::input::touch::TouchPhase;
use bevy::log::{info, Level};
use bevy::math::vec2;
use bevy::prelude::TouchInput;
use jni_sys::{JavaVM, jobject};
use ndk::event::MotionAction;
use ndk::native_window::NativeWindow;
use ndk_sys::ANativeWindow;

use crate::{Cmd, CMD_QUEUE, init_command_queue, run_game_loop};

fn forward_stdio_to_logcat() {
    // XXX: make this stdout/stderr redirection an optional / opt-in feature?...

    let file = unsafe {
        let mut logpipe: [RawFd; 2] = Default::default();
        libc::pipe2(logpipe.as_mut_ptr(), libc::O_CLOEXEC);
        libc::dup2(logpipe[1], libc::STDOUT_FILENO);
        libc::dup2(logpipe[1], libc::STDERR_FILENO);
        libc::close(logpipe[1]);

        File::from_raw_fd(logpipe[0])
    };

    std::thread::Builder::new()
        .name("stdio-to-logcat".to_string())
        .spawn(move || {
            let tag = CStr::from_bytes_with_nul(b"RustStdoutStderr\0").unwrap();
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            loop {
                buffer.clear();
                let len = match reader.read_line(&mut buffer) {
                    Ok(len) => len,
                    Err(e) => {
                        println!("Logcat forwarder failed to read stdin/stderr: {e:?}");
                        break Err(e);
                    }
                };
                if len == 0 {
                    break Ok(());
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    android_log(Level::INFO, tag, &msg);
                }
            }
        })
        .expect("Failed to start stdout/stderr to logcat forwarder thread");
}

fn android_log(level: Level, tag: &CStr, msg: &CStr) {
    let prio = match level {
        Level::ERROR => ndk_sys::android_LogPriority::ANDROID_LOG_ERROR,
        Level::WARN => ndk_sys::android_LogPriority::ANDROID_LOG_WARN,
        Level::INFO => ndk_sys::android_LogPriority::ANDROID_LOG_INFO,
        Level::DEBUG => ndk_sys::android_LogPriority::ANDROID_LOG_DEBUG,
        Level::TRACE => ndk_sys::android_LogPriority::ANDROID_LOG_VERBOSE,
    };
    unsafe {
        ndk_sys::__android_log_write(prio.0 as libc::c_int, tag.as_ptr(), msg.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn initCommandQueue() {
    forward_stdio_to_logcat();
    init_command_queue();
}

#[no_mangle]
pub extern "C" fn drainCommandQueue() {
    let rx = CMD_QUEUE.get().unwrap()
        .receiver.lock().unwrap();
    while let Ok(_) = rx.try_recv() {
        // ignore
    }
}

#[no_mangle]
pub extern "C" fn surfaceRedrawNeeded() {
    //ignore this
}

#[no_mangle]
pub extern "C" fn activityCreated(
    vm: *mut JavaVM,
    activity: jobject,
) {
    unsafe {
        ndk_context::initialize_android_context(vm.cast(), activity.cast());
        println!("surfaceCreated called, initialize the context");
    }
}

#[no_mangle]
pub extern "C" fn activityDestroyed() {
    unsafe {
        ndk_context::release_android_context();
        println!("surfaceDestroyed called, release the context")
    }
}


#[no_mangle]
pub extern "C" fn surfaceCreated(
    window: *mut ANativeWindow,
) {
    unsafe {
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
    let running_loop = CMD_QUEUE.get().unwrap()
        .running_loop.lock().unwrap();

    if !*running_loop {
        return;
    }

    let mut done = CMD_QUEUE.get().unwrap()
        .surface_destroyed_handle_done.lock().unwrap();
    *done = false;

    Cmd::SurfaceDestroyed.send();

    while !*done {
        // wait for surfaceDestroyed to be handled
        info!("wait for surfaceDestroyed to be handled");
        done = CMD_QUEUE.get().unwrap()
            .surface_destroyed_handle_done_var.wait(done).unwrap();
    }

    info!("surfaceDestroyed handled");
}

#[no_mangle]
pub extern "C" fn runGameLoop() {
    run_game_loop();
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


