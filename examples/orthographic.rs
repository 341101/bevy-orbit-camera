use std::f32::consts::PI;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_orbit_camera::{controls::OrbitControlsPlugin, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            OrbitCameraPlugin::default(),
            OrbitControlsPlugin::<With<MainCamera>>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, switch_projection)
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
                value: "Press R to switch projection".to_string(),
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
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }),
            ..default()
        },
        MainCamera,
    ));
}

fn switch_projection(
    mut switch_projection: Local<Projection>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut camera_query: Query<&mut Projection, With<MainCamera>>,
) {
    for event in keyboard_events.read() {
        if event.key_code == KeyCode::KeyR && event.state == ButtonState::Pressed {
            let Ok(mut projection) = camera_query.get_single_mut() else {
                return;
            };
            std::mem::swap(&mut *switch_projection, &mut *projection);
        }
    }
}
