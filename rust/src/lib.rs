use std::sync::{Arc, Condvar, mpsc, Mutex, MutexGuard, OnceLock};
use std::sync::mpsc::Sender;
use ::winit::platform::android::activity::AndroidApp;

use bevy::asset::ErasedAssetLoader;
use bevy::input::touch::TouchPhase;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowMode;
use bevy::winit::WinitPlugin;
use ndk::asset::AssetManager;
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
    asset_manager: Arc<Mutex<Option<ndk::asset::AssetManager>>>,
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
            asset_manager: Arc::new(Mutex::new(None)),
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
            .disable::<WinitPlugin>() // removed by bevy feature selection
        )
        .insert_resource(LastTouchMove::default())
        .add_plugins(MyWinitPlugin {})
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update,
            move_system,
            btn_system,
        ),
        )
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

#[derive(Component)]
struct TextCount;

#[derive(Resource, Deref, DerefMut, Default)]
struct LastTouchMove(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_loader: Res<AssetServer>,
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

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            height: Val::Px(100.),
            right: Val::Px(10.),
            bottom: Val::Px(10.),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections(vec![
                TextSection {
                    value: "Button Click Count: ".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.,
                        color: Color::WHITE,
                        ..default()
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.,
                        color: Color::RED,
                        ..default()
                    },
                },
            ]),
                          TextCount
            ));

            parent.spawn(ButtonBundle {
                style: Style {
                    height: Val::Px(50.),
                    width: Val::Px(200.),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                ..default()
            })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Click Me",
                        TextStyle {
                            font: asset_loader.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Elm>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(time.delta_seconds() * 2.1);
    }
}


fn move_system(
    mut query: Query<&mut Transform, With<Elm>>,
    mut touch_input: EventReader<TouchInput>,
    mut last_touch_move: ResMut<LastTouchMove>,
) {
    for touch in touch_input.read() {
        match touch.phase {
            TouchPhase::Started => {
                last_touch_move.0 = touch.position;
            }
            TouchPhase::Moved => {
                for mut transform in query.iter_mut() {
                    transform.translation.x += touch.position.x - last_touch_move.x;
                    // due to the coordinate system of android, y axis is reversed.
                    transform.translation.y -= touch.position.y - last_touch_move.y;

                    last_touch_move.0 = touch.position;
                }
            }
            TouchPhase::Ended => {}
            TouchPhase::Canceled => {}
        }
    }
}

fn btn_system(
    mut query: Query<&mut Text, With<TextCount>>,
    mut btn_query: Query<&mut Interaction, (Changed<Interaction>, With<Button>)>,
    mut btn_click_count: Local<u32>,
) {
    for mut interaction in btn_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *btn_click_count += 1;
            for mut text in query.iter_mut() {
                text.sections[1].value = btn_click_count.to_string();
            }
        }
    }
}

#[no_mangle]
pub fn get_asset_manager() -> Option<ndk::asset::AssetManager> {
    let asset_manager = CMD_QUEUE.get().unwrap()
        .asset_manager.lock().unwrap();
    match *asset_manager {
        None => {
            None
        }
        Some(ref asset_manager) => {
            unsafe {
                Some(AssetManager::from_ptr(asset_manager.ptr()))
            }
        }
    }
}

// only for compile. bevy depends on android-activity, and this lib needs this function to compile.
#[no_mangle]
fn android_main(_: AndroidApp) {}

