use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
mod bevy_geo_nodes;
use bevy::window::{PresentMode, WindowTheme};
use bevy_geo_nodes::{MarchingCubes, VoxelGrid};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use smooth_bevy_cameras::controllers::fps::{
    FpsCameraBundle, FpsCameraController, FpsCameraPlugin,
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
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
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            // WorldInspectorPlugin::new(),
            WireframePlugin::default(),
            LookTransformPlugin,
            FpsCameraPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // camera_controls,
                // move_camera_system,
                bevy::window::close_on_esc,
            ),
        )
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

pub const ON_COLOR: Color = Color::rgba(0.8, 0.1, 0.0, 1.0);
pub const OFF_COLOR: Color = Color::rgba(0.33, 0.9, 0.33, 1.0);

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let resolution = 4 as usize;

    let eye = Vec3::new(-5.0, 5.0, -5.0);
    let target = Vec3::new(
        resolution as f32 / 3.0,
        resolution as f32 / 3.0,
        resolution as f32 / 3.0,
    );
    let up = Vec3::Y;

    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            eye,
            target,
            up,
        ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7900.0,
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

    let voxel_grid = VoxelGrid::new(&[[0.0, 0.0, 0.0]], resolution);
    voxel_grid.debug(&mut |x, y, z, resolution, noise| {
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Cube { size: 0.10 }.try_into().unwrap()),
            material: materials.add(StandardMaterial {
                base_color: OFF_COLOR,
                emissive: OFF_COLOR,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        });
    });

    // commands
    //     .spawn(pbr)
    //     .insert(Name::new("Model"))
    //     .insert(Wireframe);
}
