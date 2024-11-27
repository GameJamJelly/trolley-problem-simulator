//! Handles to application resources, such as images and sounds.

use bevy::prelude::*;

/// Scenario timer resource.
#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioTimer(pub Timer);

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

/// The resource for the texture of the single hostage.
#[derive(Resource, Deref, DerefMut)]
pub struct Hostage1Res(pub Handle<Image>);

/// The resource for the texture of the group of five hostages.
#[derive(Resource, Deref, DerefMut)]
pub struct Hostage5Res(pub Handle<Image>);

/// The resource for the front-facing trolley texture.
#[derive(Resource, Deref, DerefMut)]
pub struct TrolleyFrontRes(pub Handle<Image>);

/// The resource for the turning trolley texture.
#[derive(Resource, Deref, DerefMut)]
pub struct TrolleyTurnRes(pub Handle<Image>);

/// The resource for the side-facing trolley texture.
#[derive(Resource, Deref, DerefMut)]
pub struct TrolleySideRes(pub Handle<Image>);
