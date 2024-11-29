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
