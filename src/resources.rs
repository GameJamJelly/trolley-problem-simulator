//! Handles to application resources, such as images and sounds.

use bevy::prelude::*;

/// The resource containing the UI entity spawned for the menu screen.
#[derive(Resource, Deref, DerefMut)]
pub struct MenuEntityRes(pub Entity);

/// The resource containing entities spawned for a scenario.
#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioEntitiesRes(pub Vec<Entity>);

/// The resource containing the UI entity spawned for the end screen.
#[derive(Resource, Deref, DerefMut)]
pub struct EndScreenEntityRes(pub Entity);

/// The resource for the track texture.
#[derive(Resource, Deref, DerefMut)]
pub struct TracksNormalRes(pub Handle<Image>);

/// The resource for the track texture with the lever pulled.
#[derive(Resource, Deref, DerefMut)]
pub struct TracksSwitchedRes(pub Handle<Image>);

/// The resource for the lever/player texture.
#[derive(Resource, Deref, DerefMut)]
pub struct LeverPlayerNormalRes(pub Handle<Image>);

/// The resource for the lever/player texture with the lever pulled.
#[derive(Resource, Deref, DerefMut)]
pub struct LeverPlayerSwitchedRes(pub Handle<Image>);
