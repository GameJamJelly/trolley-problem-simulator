//! Utility functions.

use crate::constants::*;
use bevy::prelude::*;
use std::time::Duration;

/// Returns the value between `a` and `b`, such that the value is the `amount`
/// portion of the way between `a` and `b`.
pub const fn point_between(a: f32, b: f32, amount: f32) -> f32 {
    (a * (1.0 - amount)) + (b * amount)
}

/// Normalizes a translation on the screen relative to the canvas.
pub const fn normalize_translation_to_canvas_with_z(point: Vec2, z: f32) -> Vec3 {
    Vec3::new(
        point.x - (SCREEN_WIDTH / 2.0),
        (SCREEN_HEIGHT - point.y) - (SCREEN_HEIGHT / 2.0),
        z,
    )
}

/// Normalizes a translation on the screen relative to the canvas, assuming
/// `0.0` for the z coordinate.
pub const fn normalize_translation_to_canvas(point: Vec2) -> Vec3 {
    normalize_translation_to_canvas_with_z(point, 0.0)
}

/// Calculates the linear transform to perform on an object moving on the
/// screen.
pub fn movement_transform(
    start_transform: Transform,
    end_transform: Transform,
    duration: Duration,
    time_remaining: Duration,
) -> Transform {
    let start_point_x = start_transform.translation.x;
    let start_point_y = start_transform.translation.y;
    let start_scale = start_transform.scale.x;
    let end_point_x = end_transform.translation.x;
    let end_point_y = end_transform.translation.y;
    let end_scale = end_transform.scale.x;

    let amount = 1.0 - (time_remaining.as_secs_f32() / duration.as_secs_f32());

    let transformed_x = point_between(start_point_x, end_point_x, amount);
    let transformed_y = point_between(start_point_y, end_point_y, amount);
    let transformed_scale = point_between(start_scale, end_scale, amount);

    Transform::IDENTITY
        .with_translation(normalize_translation_to_canvas(Vec2::new(
            transformed_x,
            transformed_y,
        )))
        .with_scale(Vec3::new(transformed_scale, transformed_scale, 1.0))
}

/// Calculates the transform to perform on an object on the horizon approaching
/// a point on the screen.
pub fn horizon_distance_transform(
    horizon_point: Vec2,
    end_transform: Transform,
    time_remaining: Duration,
) -> Transform {
    let start_point_x = horizon_point.x;
    let start_point_y = horizon_point.y;
    let start_scale = 0.0f32;
    let end_point_x = end_transform.translation.x;
    let end_point_y = end_transform.translation.y;
    let end_scale = end_transform.scale.x;

    let transformed_x = point_between(
        start_point_x,
        end_point_x,
        HORIZON_TRANSLATE_BASE.powf(-time_remaining.as_secs_f32()),
    );
    let transformed_y = point_between(
        start_point_y,
        end_point_y,
        HORIZON_TRANSLATE_BASE.powf(-time_remaining.as_secs_f32()),
    );
    let transformed_scale = point_between(
        start_scale,
        end_scale,
        HORIZON_SCALE_BASE.powf(-time_remaining.as_secs_f32()),
    );

    Transform::IDENTITY
        .with_translation(normalize_translation_to_canvas(Vec2::new(
            transformed_x,
            transformed_y,
        )))
        .with_scale(Vec3::new(transformed_scale, transformed_scale, 1.0))
}

/// Formats the timer text.
pub fn format_timer_text(duration: Duration) -> String {
    let seconds = duration.as_secs_f32().ceil() as u64;
    format!("{}:{:0>2}", seconds / 60, seconds % 60)
}

/// Checks if the timer has just reached a target time.
pub const fn time_remaining_reached(
    previous_time_remaining: f32,
    current_time_remaining: f32,
    target_time_remaining: f32,
) -> bool {
    previous_time_remaining >= target_time_remaining
        && target_time_remaining >= current_time_remaining
}
