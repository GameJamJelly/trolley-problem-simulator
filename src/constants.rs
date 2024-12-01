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

/// The amount of time in seconds to wait after the trolley has finished its
/// animation.
pub const POST_ANIMATION_WAIT_TIME: f32 = 3.0;

/// The exponential base by which to translate an object as it approaches from
/// the horizon.
pub const HORIZON_TRANSLATE_BASE: f32 = 1.25;

/// The exponential base by which to scale an object as it approaches from the
/// horizon.
pub const HORIZON_SCALE_BASE: f32 = 1.1;

/// The point on the horizon where the trolley approaches from.
pub const APPROACHING_TROLLEY_HORIZON_POINT: Vec2 = Vec2::new(43.0, 61.0);

/// The final state of the approaching trolley horizon transform.
pub const APPROACHING_TROLLEY_HORIZON_END_TRANSFORM: Transform = Transform::IDENTITY
    .with_translation(Vec3::new(50.0, 135.0, 0.0))
    .with_scale(Vec3::new(0.8, 0.8, 1.0));

/// The final state of the approaching trolley turn transform.
pub const APPROACHING_TROLLEY_TURNING_END_TRANSFORM: Transform = Transform::IDENTITY
    .with_translation(Vec3::new(112.0, 171.0, 0.0))
    .with_scale(Vec3::new(0.85, 0.85, 1.0));

/// The final state of the approaching trolley side transform.
pub const APPROACHING_TROLLEY_SIDE_END_TRANSFORM: Transform = Transform::IDENTITY
    .with_translation(Vec3::new(298.0, 229.0, 0.0))
    .with_scale(Vec3::new(1.0, 1.0, 1.0));

/// The standard position of hostages on track A.
pub const STANDARD_HOSTAGES_POS_TRACK_A: Vec2 = Vec2::new(530.0, 325.0);

/// The standard position of hostages on track B.
pub const STANDARD_HOSTAGES_POS_TRACK_B: Vec2 = Vec2::new(550.0, 230.0);

/// The transform for the double it next person's switch.
pub const NEXT_PERSON_SWITCH_TRANSFORM: Transform = Transform::from_xyz(650.0, 380.0, 0.0);

/// The transform for the player when they jump onto the track.
pub const SELF_JUMP_TRANSFORM: Transform = Transform::from_xyz(285.0, 245.0, -10.0);

/// A bullet point character.
pub const BULLET_POINT: char = '-';

/// The audio volume.
pub const GAME_VOLUME: f32 = 0.5;
