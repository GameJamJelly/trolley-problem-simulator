//! Application components.

use bevy::prelude::*;

/// The track texture component.
#[derive(Component)]
pub struct TrackTexture;

/// The lever/player texture component.
#[derive(Component)]
pub struct LeverPlayerTexture;

/// The hostages on track A texture component.
#[derive(Component)]
pub struct HostagesTrackATexture;

/// The hostages on track B texture component.
#[derive(Component)]
pub struct HostagesTrackBTexture;

/// The trolley texture component.
#[derive(Component)]
pub struct TrolleyTexture;

/// The scenario timer text component.
#[derive(Component)]
pub struct TimerText;

/// The Youtube tripod texture component.
#[derive(Component)]
pub struct TripodTexture;

/// The cliff texture component.
#[derive(Component)]
pub struct CliffTexture;

/// The track texture component for the right half of the double it scenario.
#[derive(Component)]
pub struct DoubleItRightHalfTrackTexture;

/// The double hostages texture.
#[derive(Component)]
pub struct DoubleItHostagesTexture;

/// The next person texture.
#[derive(Component)]
pub struct NextPersonTexture;
