use std::sync::{Arc, Condvar, mpsc, Mutex, OnceLock};
use std::sync::mpsc::Sender;

use ::winit::platform::android::activity::AndroidApp;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowMode;
use ndk::native_window::NativeWindow;

use crate::winit::MyWinitPlugin;

mod winit;
mod c_api;

#[derive(Debug)]
enum Cmd {
    SurfaceCreated(NativeWindow),
    SurfaceChanged(u32, u32),
    SurfaceDestroyed,
    StopGame,
    TouchEvent(TouchInput),
    OnResume,
    OnPause,
}

struct CmdQueue {
    sender: Arc<Mutex<Sender<Cmd>>>,
    receiver: Arc<Mutex<mpsc::Receiver<Cmd>>>,
    surface_destroyed_handle_done: Arc<Mutex<bool>>,
    surface_destroyed_handle_done_var: Arc<Condvar>,
    running_loop: Arc<Mutex<bool>>,
}

static CMD_QUEUE: OnceLock<CmdQueue> = OnceLock::new();

impl Cmd {
    fn send(self) {
        let cmd_queue = CMD_QUEUE.get();
        if let Some(cmd_queue) = cmd_queue {
            let sender = cmd_queue.sender.lock().unwrap();
            sender.send(self).unwrap();
        } else {
            warn!("cmd_queue is None, can not send cmd: {:?}", self);
        }
    }
}


pub fn init_command_queue() {
    CMD_QUEUE.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Cmd>();

        CmdQueue {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
            surface_destroyed_handle_done: Arc::new(Mutex::new(true)),
            surface_destroyed_handle_done_var: Arc::new(Condvar::new()),
            running_loop: Arc::new(Mutex::new(false)),
        }
    });
}

pub fn run_game_loop() {
    info!("start game loop");
    {
        let mut running_loop = CMD_QUEUE.get().unwrap().running_loop.lock().unwrap();
        *running_loop = true;
    }

    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            resizable: false,
                            mode: WindowMode::BorderlessFullscreen,
                            ..default()
                        }),
                        ..default()
                    }
                )
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
            // .disable::<WinitPlugin>() // removed by bevy feature selection
        )
        .add_plugins(MyWinitPlugin {})
        .add_systems(Startup, setup)
        .add_systems(Update, update)
    ;

    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();


    // drain the queue
    {
        let cmd_receiver = CMD_QUEUE.get().unwrap()
            .receiver.lock().unwrap();

        for cmd in cmd_receiver.try_iter() {
            info!("discard previous run cmd: {:?}", cmd);
        }
    }

    {
        let mut running_loop = CMD_QUEUE.get().unwrap().running_loop.lock().unwrap();
        *running_loop = false;
    }
}

#[derive(Component)]
struct Elm;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(50., 100.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Elm
    ));
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Elm>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(time.delta_seconds() * 2.1);
    }
}


// only for compile. bevy depends on android-activity, and this lib needs this function to compile.
#[no_mangle]
fn android_main(_: AndroidApp) {}

