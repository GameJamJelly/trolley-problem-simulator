//! Utility functions.

use crate::constants::*;
use bevy::prelude::*;
use std::time::Duration;

/// Returns the value between `a` and `b`, such that the value is the `amount`
/// portion of the way between `a` and `b`.
const fn point_between(a: f32, b: f32, amount: f32) -> f32 {
    (a * (1.0 - amount)) + (b * amount)
}

/// Normalizes a translation on the screen relative to the canvas.
pub const fn normalize_translation_to_canvas(point: Vec2) -> Vec3 {
    Vec3::new(
        point.x - (SCREEN_WIDTH / 2.0),
        (SCREEN_HEIGHT - point.y) - (SCREEN_HEIGHT / 2.0),
        0.0,
    )
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
