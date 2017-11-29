//! Tranforms.

use vecmath::Vector2;

/// A transform, containing an entity's position,
/// rotation, and scale.
pub struct Transform {
    /// An entity's 2D position (world).
    pub position: Vector2<f64>,
    /// An entity's 2D rotation (world) in radians.
    pub rotation: Vector2<f64>,
    /// An entity's 2D scale (world).
    pub scale: Vector2<f64>,
}

impl Transform {
    /// Create a new Transform with the given elements.
    pub fn new(position: Vector2<f64>, rotation: Vector2<f64>, scale: Vector2<f64>) -> Transform {
        Transform {
            position,
            rotation,
            scale,
        }
    }

    /// Create a new Transform with zeroed elements.
    pub fn zero() -> Transform {
        Transform::new([0.0, 0.0], [0.0, 0.0], [0.0, 0.0])
    }
}
