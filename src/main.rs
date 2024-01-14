use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
mod bevy_geo_nodes;
use bevy::window::{PresentMode, WindowTheme};
use bevy_geo_nodes::{GeoNode, Node};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 480.0;
pub const GAME_TITLE: &str = "Rusty Tank Game";
pub const START_X_POX: f32 = 1080.0;
pub const START_Y_POX: f32 = 0.0;
pub const RESOLUTION: (f32, f32) = (WIDTH, HEIGHT);
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am window!".into(),
                    position: WindowPosition::At(IVec2::new(1720, 0)),
                    resolution: RESOLUTION.into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WorldInspectorPlugin::new(),
            WireframePlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_controls, bevy::window::close_on_esc))
        .run();
}

#[derive(Debug, Clone, Resource, Reflect)]
pub struct Bounds2D {
    pub min_x: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_z: f32,
}

pub const GAME_X_MIN: f32 = 0.0;
pub const GAME_Z_MIN: f32 = 0.0;
pub const GAME_X_MAX: f32 = 32.0;
pub const GAME_Z_MAX: f32 = 32.0;

pub const CAMERA_ROTATION_SPEED: f32 = 2.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 25.0;

pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: GAME_X_MIN,
    min_z: GAME_Z_MIN,
    max_x: GAME_X_MAX,
    max_z: GAME_Z_MAX,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut geo_node = GeoNode::from_mesh(Mesh::from(shape::Cube::new(0.40)));
    let mut geo_node2 = GeoNode::from_mesh(Mesh::from(shape::Cube::new(0.80)));

    // Set scale
    const CUBE_SCALE: Vec3 = Vec3::new(10.0, 10.0, 10.0);
    geo_node.scale = CUBE_SCALE;
    // geo_node2.scale = CUBE_SCALE - 4.0;

    geo_node.combine(geo_node2);

    // Add color to the mesh
    geo_node.material = StandardMaterial {
        base_color: Color::rgba(0.8, 0.1, 0.6, 0.02),
        emissive: Color::rgba(0.8, 0.1, 0.6, 0.02),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    };

    let mut pbr = geo_node.get_pbr_bundle(&mut meshes, &mut materials);

    pbr.transform.scale = CUBE_SCALE;

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(GAME_BOUNDS.max_x, 16., GAME_BOUNDS.max_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5900.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(GAME_BOUNDS.max_x, GAME_BOUNDS.max_z, GAME_BOUNDS.max_z),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands
        .spawn(pbr)
        .insert(Name::new("Model"))
        .insert(Wireframe);
}

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    // mut game: ResMut<Game>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut entities: Query<&mut Transform, Without<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = CAMERA_MOVEMENT_SPEED;
    let rotate_speed = CAMERA_ROTATION_SPEED;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
    // if keyboard.pressed(KeyCode::Z) {
    //     camera.translation.y += speed * time.delta_seconds();
    // }
    // if keyboard.pressed(KeyCode::X) {
    //     camera.translation.y -= speed * time.delta_seconds();
    // }
    // Zoom in/out
    if keyboard.pressed(KeyCode::Z) {
        // Iterate through all meshes and zoom them in/out
        for mut transform in entities.iter_mut() {
            transform.scale *= 1.0 + time.delta_seconds();
        }
    }
    if keyboard.pressed(KeyCode::X) {
        for mut transform in entities.iter_mut() {
            transform.scale *= 1.0 - time.delta_seconds();
        }
    }
}
