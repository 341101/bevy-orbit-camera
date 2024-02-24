pub mod controls;
pub mod util;

use bevy::transform::TransformSystem::TransformPropagate;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use std::{
    f32::consts::{PI, TAU},
    fmt::Debug,
    ops::RangeInclusive,
};

/// Defines a Bevy plugin for a Pan-Orbit camera system, which allows for panning, orbiting, and zooming around a focus point in a 3D scene.
/// # Example
/// ```
/// use std::f32::consts::PI;
/// use bevy::prelude::*;
/// use bevy_orbit_camera::{controls::OrbitControlsPlugin, *};
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins((
///             OrbitCameraPlugin::default(),
///             // optional
///             OrbitControlsPlugin::<With<MainCamera>>::default(),
///         ))
///         .add_systems(Startup, setup)
///         .run();
/// }
///
/// #[derive(Component)]
/// pub struct MainCamera;
///
/// fn setup(
///     mut commands: Commands,
/// ) {
///     // ...
///     commands.spawn((
///         OrbitCamera {
///             radius: 6.0,
///             delta_pitch: PI / 8.0,
///             ..Default::default()
///         },
///         Camera3dBundle::default(),
///         MainCamera,
///     ));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OrbitCameraPlugin<T = PostUpdate> {
    label: T,
}

impl Default for OrbitCameraPlugin<PostUpdate> {
    fn default() -> Self {
        Self { label: PostUpdate }
    }
}

/// A SystemSet for updating camera properties based on input and other factors.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OrbitCameraSystemSet;

impl<T: ScheduleLabel + Clone> Plugin for OrbitCameraPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            self.label.clone(),
            update_transform
                .in_set(OrbitCameraSystemSet)
                .before(TransformPropagate),
        );
    }
}

/// Component for Pan-Orbit camera functionality, allowing the camera to orbit around a focus point, zoom in and out, and pan across the scene.
#[derive(Debug, Clone, Component, PartialEq)]
pub struct OrbitCamera {
    /// The focus point around which the camera orbits.
    pub focus: Vec3,
    /// The distance from the camera to the focus point.
    pub radius: f32,

    pub delta_yaw: f32,
    pub delta_pitch: f32,
    pub delta_roll: f32,
    pub pan: Vec2,

    /// Optional limit for the camera's radius.
    pub radius_limit: RangeInclusive<Option<f32>>,

    pub lock_up_axis: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 1.0)
    }
}

impl OrbitCamera {
    pub fn new(focus: Vec3, radius: f32) -> Self {
        Self {
            focus,
            radius,
            delta_yaw: 0.0,
            delta_pitch: 0.0,
            delta_roll: 0.0,
            pan: Vec2::ZERO,
            radius_limit: RangeInclusive::new(None, None),
            lock_up_axis: false,
        }
    }

    pub fn with_orbit(mut self, delta_yaw: f32, delta_pitch: f32, delta_roll: f32) -> Self {
        self.orbit(delta_yaw, delta_pitch, delta_roll);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_focus(mut self, focus: Vec3) -> Self {
        self.focus = focus;
        self
    }

    pub fn reset_rotation_and_pan_deltas(&mut self) {
        self.delta_yaw = 0.0;
        self.delta_pitch = 0.0;
        self.delta_roll = 0.0;
        self.pan = Vec2::ZERO;
    }

    fn update_transform(&mut self, transform: &mut Transform, projection: &mut Projection) {
        let radius = if let Projection::Orthographic(ref mut p) = projection {
            p.scale = self.radius;
            (p.far + p.near) / 2.0
        } else {
            self.radius
        };
        self.focus += transform.rotation * self.pan.extend(0.0);
        if self.lock_up_axis {
            let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            pitch = (pitch - self.delta_pitch).clamp(-PI / 2.0, PI / 2.0);
            yaw += self.delta_yaw;
            let smoothness = 0.6;
            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, yaw, pitch, smoothness * (roll % TAU));
        } else {
            transform.rotate_axis(transform.local_x().into(), -self.delta_pitch);
            transform.rotate_axis(transform.local_y().into(), self.delta_yaw);
            transform.rotate_axis(transform.local_z().into(), self.delta_roll);
        }
        self.reset_rotation_and_pan_deltas();
        transform.translation = self.focus + transform.rotation * Vec3::new(0.0, 0.0, radius);
    }

    pub fn zoom(&mut self, factor: f32) {
        self.radius *= factor;
        if let Some(lower) = self.radius_limit.start() {
            self.radius = self.radius.max(*lower);
        }
        if let Some(upper) = self.radius_limit.end() {
            self.radius = self.radius.min(*upper);
        }
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.pan += delta;
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32, delta_roll: f32) {
        self.delta_yaw += delta_yaw;
        self.delta_pitch += delta_pitch;
        self.delta_roll += delta_roll;
    }

    pub fn yaw(&mut self, delta: f32) {
        self.delta_yaw += delta;
    }

    pub fn pitch(&mut self, delta: f32) {
        self.delta_pitch += delta;
    }

    pub fn roll(&mut self, delta: f32) {
        self.delta_roll += delta;
    }
}

fn update_transform(mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Projection)>) {
    for (mut pan_orbit_camera, mut transform, mut projection) in query.iter_mut() {
        pan_orbit_camera.update_transform(&mut transform, &mut projection);
    }
}
