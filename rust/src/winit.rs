use std::time::{Duration, Instant};

use bevy::app::{AppExit, PluginsState};
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::input::touchpad::{TouchpadMagnify, TouchpadRotate};
use bevy::prelude::*;
use bevy::tasks::tick_global_task_pools_on_main_thread;
use bevy::window::{ApplicationLifetime, CursorEntered, CursorLeft, CursorMoved, FileDragAndDrop, Ime, RawHandleWrapper, ReceivedCharacter, Window, WindowBackendScaleFactorChanged, WindowCloseRequested, WindowCreated, WindowDestroyed, WindowFocused, WindowMoved, WindowResized, WindowScaleFactorChanged, WindowThemeChanged};
use ndk::native_window::NativeWindow;
use raw_window_handle::{AndroidDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle};

use crate::{Cmd, CMD_QUEUE};

pub struct MyWinitPlugin {}


#[derive(Debug, Deref, DerefMut)]
struct MyWindow(NativeWindow);

#[derive(Debug)]
struct WinitWindow {
    entity: Option<Entity>,
    app_should_run: bool,
    started: bool,
    window: Option<MyWindow>,
    last_update: Instant,
}

impl From<NativeWindow> for MyWindow {
    fn from(value: NativeWindow) -> Self {
        Self(value)
    }
}


impl Plugin for MyWinitPlugin {
    fn build(&self, app: &mut App) {
        app
            .set_runner(my_runner)
        ;
    }
}


#[derive(SystemParam)]
struct WindowAndInputEventWriters<'w> {
    // `winit` `WindowEvent`s
    window_resized: EventWriter<'w, WindowResized>,
    window_close_requested: EventWriter<'w, WindowCloseRequested>,
    window_scale_factor_changed: EventWriter<'w, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventWriter<'w, WindowBackendScaleFactorChanged>,
    window_focused: EventWriter<'w, WindowFocused>,
    window_moved: EventWriter<'w, WindowMoved>,
    window_theme_changed: EventWriter<'w, WindowThemeChanged>,
    window_destroyed: EventWriter<'w, WindowDestroyed>,
    lifetime: EventWriter<'w, ApplicationLifetime>,
    keyboard_input: EventWriter<'w, KeyboardInput>,
    character_input: EventWriter<'w, ReceivedCharacter>,
    mouse_button_input: EventWriter<'w, MouseButtonInput>,
    touchpad_magnify_input: EventWriter<'w, TouchpadMagnify>,
    touchpad_rotate_input: EventWriter<'w, TouchpadRotate>,
    mouse_wheel_input: EventWriter<'w, MouseWheel>,
    touch_input: EventWriter<'w, TouchInput>,
    ime_input: EventWriter<'w, Ime>,
    file_drag_and_drop: EventWriter<'w, FileDragAndDrop>,
    cursor_moved: EventWriter<'w, CursorMoved>,
    cursor_entered: EventWriter<'w, CursorEntered>,
    cursor_left: EventWriter<'w, CursorLeft>,
    // `winit` `DeviceEvent`s
    mouse_motion: EventWriter<'w, MouseMotion>,
}

fn my_runner(mut app: App) {
    if app.plugins_state() == PluginsState::Ready {
        // If we're already ready, we finish up now and advance one frame.
        // This prevents black frames during the launch transition on iOS.
        app.finish();
        app.cleanup();
        app.update();
    }


    let mut create_window_system_state: SystemState<(
        Commands,
        Query<(Entity, &mut Window), Added<Window>>,
        EventWriter<WindowCreated>,
    )> = SystemState::from_world(&mut app.world);


    let mut event_writer_system_state: SystemState<(
        WindowAndInputEventWriters,
        Query<&mut Window>,
    )> = SystemState::new(&mut app.world);


    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    let cmd_receiver = CMD_QUEUE.get().unwrap()
        .receiver.lock().unwrap();

    let mut winit_window = WinitWindow {
        entity: None,
        app_should_run: false,
        started: false,
        window: None,
        last_update: Instant::now(),
    };

    let mut quit = false;

    let event_handler = move |event: Cmd| {
        match event {
            Cmd::SurfaceCreated(native_window) => {
                let (commands,
                    mut win_query,
                    win_evt_writer,
                ) = create_window_system_state.get_mut(&mut app.world);

                create_windows(
                    commands,
                    win_query.iter_mut(),
                    win_evt_writer,
                    &mut winit_window,
                    native_window,
                );

                create_window_system_state.apply(&mut app.world);
            }
            Cmd::OnResume | Cmd::SurfaceChanged(_, _) | Cmd::SurfaceDestroyed
            | Cmd::StopGame | Cmd::TouchEvent(_) | Cmd::OnPause => {
                let (mut event_writers,
                    mut windows,
                ) = event_writer_system_state.get_mut(&mut app.world);


                let window_entity = winit_window.entity;
                if window_entity == None {
                    panic!("window_entity == None");
                }
                let window_entity = window_entity.unwrap();
                let mut window = windows.get_mut(window_entity).unwrap();

                match event {
                    Cmd::SurfaceChanged(width, height) => {
                        window
                            .resolution
                            .set_physical_resolution(width, height);

                        event_writers.window_resized.send(WindowResized {
                            window: window_entity,
                            width: window.width(),
                            height: window.height(),
                        });

                        winit_window.app_should_run = true;
                    }
                    Cmd::SurfaceDestroyed => {
                        event_writers.window_destroyed.send(WindowDestroyed {
                            window: window_entity,
                        });
                        winit_window.app_should_run = false;
                    }
                    Cmd::OnResume => {
                        match winit_window.started {
                            false => {
                                event_writers.lifetime.send(ApplicationLifetime::Started);
                            }
                            _ => {
                                event_writers.lifetime.send(ApplicationLifetime::Resumed);
                            }
                        }
                    }

                    Cmd::StopGame => {
                        quit = true;
                    }
                    Cmd::TouchEvent(input) => {
                        event_writers
                            .touch_input
                            .send(input);
                    }
                    Cmd::OnPause => {
                        event_writers.lifetime.send(ApplicationLifetime::Suspended);
                        // handel pause
                        if winit_window.app_should_run {
                            app.update();
                        }
                    }
                    _ => {}
                }
            }
        }
    };


    let wait_duration = Duration::from_secs_f64(1.0 / 30.0);
    while !quit {
        // println!("event: {:?}", event);

        if app.plugins_state() != PluginsState::Cleaned {
            if app.plugins_state() != PluginsState::Ready {
                tick_global_task_pools_on_main_thread();
            } else {
                app.finish();
                app.cleanup();
            }
        }

        let next_wait_duration = wait_duration
            .checked_sub(wait_duration - (Instant::now() - winit_window.last_update))
            .unwrap_or_else(|| Duration::from_secs(0));
        if let Ok(event) = cmd_receiver.recv_timeout(next_wait_duration) {
            event_handler(event);
        } else {
            if app.plugins_state() == PluginsState::Cleaned && winit_window.app_should_run {
                winit_window.last_update = Instant::now();
                app.update();
            }

            if let Some(app_exit_events) = app.world.get_resource::<Events<AppExit>>() {
                if app_exit_event_reader.read(app_exit_events).last().is_some() {
                    event_handler(Cmd::StopGame);
                }
            }
        }
    };
}


unsafe impl HasRawDisplayHandle for MyWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Android(AndroidDisplayHandle::empty())
    }
}

fn create_windows<'a>(
    mut commands: Commands,
    mut created_windows: impl Iterator<Item=(Entity, Mut<'a, Window>)>,
    mut event_writer: EventWriter<WindowCreated>,
    winit_windows: &mut WinitWindow,
    native_window: NativeWindow,
) {
    let (win_entity, mut window) = created_windows.next().unwrap();


    window.resolution
        .set_scale_factor(1.0);

    let native_window: MyWindow = native_window.into();


    commands
        .entity(win_entity)
        .insert(RawHandleWrapper {
            window_handle: native_window.raw_window_handle(),
            display_handle: native_window.raw_display_handle(),
        });

    winit_windows.entity = Some(win_entity);
    winit_windows.window = Some(native_window);

    event_writer.send(WindowCreated { window: win_entity });
}