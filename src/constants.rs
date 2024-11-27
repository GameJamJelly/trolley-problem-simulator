//! Application constants.

use bevy::prelude::*;

/// The width of the screen in pixels.
pub const SCREEN_WIDTH: f32 = 800.0;

/// The height of the screen in pixels.
pub const SCREEN_HEIGHT: f32 = 600.0;

/// The normal color of a button.
pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

/// The color of a button when it is being hovered over.
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

/// The color of a button when it is being pressed.
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

/// The exponential base by which to translate an object as it approaches from
/// the horizon.
pub const HORIZON_TRANSLATE_BASE: f32 = 1.25;

/// The exponential base by which to scale an object as it approaches from the
/// horizon.
pub const HORIZON_SCALE_BASE: f32 = 1.1;

/// The point on the horizon where the trolley approaches from.
pub const APPROACHING_TROLLEY_HORIZON_POINT: Vec2 = Vec2::new(43.0, 61.0);

/// The final state of the approaching trolley transform.
pub const APPROACHING_TROLLEY_END_TRANSFORM: Transform = Transform::IDENTITY
    .with_translation(Vec3::new(50.0, 135.0, 0.0))
    .with_scale(Vec3::new(0.8, 0.8, 1.0));
