# Bevy Orbit Camera

`bevy-orbit-camera` is a Bevy plugin designed to provide a orbit camera. Allows quickly add and configure a orbit camera.


## Usage

### Basic Setup

To use the OrbitCameraPlugin, simply add it to your Bevy application:

```rust
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
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ...
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
```

### Configuration Controls

Configure the camera's control behavior according to your needs:

```rust
// Example configuration with custom settings
fn setup(mut commands: Commands) {
    commands.insert_resource(OrbitControlConfig {
        zoom_speed: 0.2,
        rotation_speed: PI,
        pan_speed: 1.0,
        roll_speed: PI,
        enable: true,
        enable_zoom: true,
        enable_rotation: true,
        enable_pan: true,
        enable_roll: true,
        zoom_smoothness: 0.8,
        rotate_button: Some(MouseButton::Left),
        zoom_button: None, // Use scroll wheel
        pan_button: Some(MouseButton::Middle),
        roll_button: Some((KeyCode::KeyQ, KeyCode::KeyE)),
    });

    // Camera and other setup code
}

```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report issues, or suggest new features.

