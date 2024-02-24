use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_orbit_camera::{controls::OrbitControlsPlugin, *};
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            EguiPlugin,
            OrbitCameraPlugin::default(),
            OrbitControlsPlugin::<With<MainCamera>>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, lock_up_axis)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // help
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Press R to lock/unlock up axis".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        },
        ..default()
    });
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        OrbitCamera {
            radius: 6.0,
            delta_pitch: PI / 8.0,
            ..Default::default()
        },
        Camera3dBundle::default(),
        MainCamera,
    ));
}

fn lock_up_axis(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::KeyR) {
        let Ok(mut camera) = camera_query.get_single_mut() else {
            return;
        };
        camera.lock_up_axis = !camera.lock_up_axis;
    }
}
