use crate::OrbitCamera;
use bevy::prelude::*;

/// Calculates the scaling factor for panning operations.
///
/// This function computes a scaling factor for panning movements based on the current projection type and attributes of the camera.
/// The scaling factor takes into account the camera's viewport size, projection type (perspective or orthographic),
/// and other camera properties such as focal length and aspect ratio.
///
/// # Parameters
/// * `camera` - The camera instance to reference for physical viewport size.
/// * `projection` - The projection type used by the camera, either perspective or orthographic.
/// * `property` - The `PanOrbitCamera` properties, containing attributes like camera focal length.
///
/// # Returns
/// Returns a `Vec2` representing the scaling factor to be applied on the X and Y axes for panning operations.
pub fn calculate_pan_scaling_factor(
    camera: &Camera,
    projection: &Projection,
    property: &OrbitCamera,
) -> Option<Vec2> {
    if let Some(viewport_size) = camera.physical_viewport_size() {
        let viewport_size = viewport_size.as_vec2();
        let factor = match projection {
            Projection::Perspective(p) => {
                property.radius * p.fov * Vec2::new(p.aspect_ratio, 1.0) / viewport_size
            }
            Projection::Orthographic(p) => {
                Vec2::new(p.area.width(), p.area.height()) / viewport_size
            }
        };
        Some(factor)
    } else {
        None
    }
}

/// Calculates the rotation quaternion from a direction and an up vector.
pub fn from_direction(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}
