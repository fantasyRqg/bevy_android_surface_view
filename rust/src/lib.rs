use std::cell::OnceCell;
use std::sync::{Arc, mpsc, Mutex, OnceLock};
use std::sync::mpsc::Sender;
use ndk::native_window::NativeWindow;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowMode;
use crate::winit::MyWinitPlugin;

mod winit;
mod c_api;


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
}

pub static CMD_QUEUE: OnceLock<CmdQueue> = OnceLock::new();

impl Cmd {
    fn send(self) {
        let cmd_queue = CMD_QUEUE.get().unwrap();
        let sender = cmd_queue.sender.lock().unwrap();
        sender.send(self).unwrap();
    }
}

pub fn run_game_loop() {
    CMD_QUEUE.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Cmd>();

        CmdQueue {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    });


    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }
        ).set(ImagePlugin::default_nearest())
    ).add_plugins(MyWinitPlugin {})
        .add_systems(Startup, setup)
        .add_systems(Update, update)
    ;

    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
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
        transform.rotate_z(time.delta_seconds() * 2.0);
    }
}


