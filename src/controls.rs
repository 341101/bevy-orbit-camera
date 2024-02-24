use std::{f32::consts::PI, marker::PhantomData};

use bevy::{
    ecs::query::QueryFilter,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{util::calculate_pan_scaling_factor, OrbitCamera, OrbitCameraSystemSet};

/// A optional default control plugin for pan-orbit camera.
///
/// `Filter` is a generic parameter requiring the `QueryFilter` trait, allowing the plugin to be conditionally applied to entities with specific components.
pub struct OrbitControlsPlugin<Filter: QueryFilter = ()> {
    _marker: PhantomData<Filter>,
}

impl<Filter: QueryFilter> Default for OrbitControlsPlugin<Filter> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<Filter: QueryFilter + Sync + Send + 'static> Plugin for OrbitControlsPlugin<Filter> {
    fn build(&self, app: &mut App) {
        app.init_resource::<OrbitControlsConfig>()
            .add_systems(Update, smooth_component_init::<Filter>)
            .add_systems(
                Update,
                (
                    zoom_control::<Filter>,
                    rotation_control::<Filter>,
                    movement_control::<Filter>,
                    roll_control::<Filter>,
                )
                    .before(OrbitCameraSystemSet),
            );
    }
}

/// Configuration for panning, rotation, and zooming controls.
/// Includes speed settings, enable flags, and mouse button options for activating controls.
#[derive(Debug, Clone, Resource)]
pub struct OrbitControlsConfig {
    pub zoom_speed: f32,
    pub rotation_speed: f32,
    pub pan_speed: f32,
    pub roll_speed: f32,
    pub enable: bool,
    pub enable_zoom: bool,
    pub enable_rotation: bool,
    pub enable_pan: bool,
    pub enable_roll: bool,
    pub zoom_smoothness: f32,
    /// The mouse button to trigger rotation, defaults to left mouse button. Set to `None` for always-on.
    pub rotate_button: Option<MouseButton>,
    /// The mouse button to trigger zooming, defaults to mouse wheel. Set to `None` for always-on.
    pub zoom_button: Option<KeyCode>,
    /// The mouse button to trigger panning, defaults to right mouse button. Set to `None` for always-on.
    pub pan_button: Option<MouseButton>,
    pub roll_button: Option<(KeyCode, KeyCode)>,
}

impl Default for OrbitControlsConfig {
    fn default() -> Self {
        Self {
            zoom_speed: 0.2,
            rotation_speed: PI,
            pan_speed: 1.0,
            roll_speed: PI,

            enable: true,
            enable_zoom: true,
            enable_rotation: true,
            enable_pan: true,
            enable_roll: true,

            zoom_smoothness: 0.75,

            rotate_button: Some(MouseButton::Left),
            zoom_button: None,
            pan_button: Some(MouseButton::Right),
            roll_button: Some((KeyCode::KeyQ, KeyCode::KeyE)),
        }
    }
}

#[derive(Component)]
pub struct TargetZoom(f32);

pub fn smooth_component_init<Filter: QueryFilter>(
    mut commands: Commands,
    mut camera_q: Query<Entity, (Added<OrbitCamera>, Filter)>,
) {
    for entity in camera_q.iter_mut() {
        commands.entity(entity).try_insert(TargetZoom(1.0));
    }
}

/// System for controlling camera zoom based on mouse wheel input.
pub fn zoom_control<Filter: QueryFilter>(
    config: Res<OrbitControlsConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<(&mut OrbitCamera, Option<&mut TargetZoom>), Filter>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if !config.enable || !config.enable_zoom {
        scroll_events.clear();
        return;
    }
    if let Some(button) = config.zoom_button {
        if !keyboard.pressed(button) {
            scroll_events.clear();
        }
    }
    let mut zoom_factor = 1.0;
    for event in scroll_events.read() {
        let scroll_value = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => 0.005 * event.y,
        };
        zoom_factor *= 1.0 - scroll_value * config.zoom_speed;
    }
    for (mut property, target_zoom) in camera_q.iter_mut() {
        let factor = if let Some(mut target_zoom) = target_zoom {
            target_zoom.0 *= zoom_factor;
            let smoothness = config.zoom_smoothness;
            let zoom_factor = f32::lerp(1.0, target_zoom.0, 1.0 - smoothness);
            target_zoom.0 /= zoom_factor;
            zoom_factor
        } else {
            zoom_factor
        };
        property.zoom(factor);
    }
}

pub fn rotation_control<Filter: QueryFilter>(
    config: Res<OrbitControlsConfig>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut camera_q: Query<(&mut OrbitCamera, &Camera), Filter>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    if !config.enable || !config.enable_rotation {
        mouse_motion_events.clear();
        return;
    }
    if let Some(button) = config.rotate_button {
        if !mouse_input.pressed(button) {
            mouse_motion_events.clear();
            return;
        }
    }
    let delta_angle = mouse_motion_events
        .read()
        .map(|event| Vec2::new(-event.delta.x, event.delta.y))
        .sum::<Vec2>();
    for (mut property, camera) in camera_q.iter_mut() {
        if let Some(viewport_size) = camera.physical_viewport_size() {
            let min_size = viewport_size.as_vec2().min_element();
            let delta = config.rotation_speed * delta_angle / min_size;
            property.orbit(delta.x, delta.y, 0.0);
        }
    }
}

pub fn movement_control<Filter: QueryFilter>(
    config: Res<OrbitControlsConfig>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut camera_q: Query<(&mut OrbitCamera, &Camera, &Projection), Filter>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    if !config.enable || !config.enable_pan {
        mouse_motion_events.clear();
        return;
    }
    if let Some(button) = config.pan_button {
        if !mouse_input.pressed(button) {
            mouse_motion_events.clear();
            return;
        }
    }
    let mouse_motion = mouse_motion_events
        .read()
        .map(|event| event.delta)
        .sum::<Vec2>();

    for (mut property, camera, projection) in camera_q.iter_mut() {
        let pan_delta = Vec2::new(-mouse_motion.x, mouse_motion.y);
        if let Some(factor) = calculate_pan_scaling_factor(camera, projection, &property) {
            property.pan(config.pan_speed * factor * pan_delta);
        }
    }
}

pub fn roll_control<Filter: QueryFilter>(
    time: Res<Time>,
    config: Res<OrbitControlsConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<&mut OrbitCamera, Filter>,
) {
    if !config.enable || !config.enable_roll {
        return;
    }
    if let Some(button) = config.roll_button {
        let mut angle = 0.0;
        if keyboard.pressed(button.0) {
            angle += config.roll_speed * time.delta_seconds();
        }
        if keyboard.pressed(button.1) {
            angle -= config.roll_speed * time.delta_seconds();
        }
        for mut property in camera_q.iter_mut() {
            property.roll(angle);
        }
    }
}
