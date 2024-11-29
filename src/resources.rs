//! Handles to application resources, such as images and sounds.

use crate::animation::AnimationFn;
use crate::states::LeverState;
use bevy::prelude::*;
use std::collections::HashMap;

/// Lists of loaded asset paths.
#[derive(Resource)]
pub struct AssetMapPaths {
    /// List of image asset paths.
    pub image_asset_paths: Vec<String>,
}

/// A map of image asset paths to their corresponding handles.
#[derive(Resource, Deref, DerefMut)]
pub struct ImageAssetMap(pub HashMap<String, Handle<Image>>);

impl ImageAssetMap {
    /// Gets an image asset by its full path.
    pub fn get_by_path(&self, path: &str) -> Handle<Image> {
        self.0.get(path).unwrap().clone()
    }

    /// Gets an image asset by its name. This assumes that the image is in
    /// `assets/images/` and is a PNG.
    pub fn get_by_name(&self, name: &str) -> Handle<Image> {
        self.get_by_path(&format!("assets/images/{}.png", name))
    }
}

/// Scenario configuration.
pub struct ScenarioConfig {
    /// The scenario text.
    pub text: String,
    /// The scenario duration.
    pub duration: f32,
    /// The position of hostages on track A.
    pub hostages_track_a_pos: Vec2,
    /// The position of hostages on track B.
    pub hostages_track_b_pos: Vec2,
    /// The name of the normal track texture.
    pub tracks_normal_texture: String,
    /// The name of the switched track texture.
    pub tracks_switched_texture: String,
    /// The name of the normal lever/player texture.
    pub lever_normal_texture: String,
    /// The name of the switched lever/player texture.
    pub lever_switched_texture: String,
    /// The name of the track A hostages texture.
    pub hostages_track_a_normal_texture: String,
    /// The name of the track B hostages texture.
    pub hostages_track_b_normal_texture: String,
}

/// Scenarios configuration resource.
#[derive(Resource, Deref, DerefMut)]
pub struct ScenariosConfigRes(pub Vec<ScenarioConfig>);

impl ScenariosConfigRes {
    /// Gets the requested scenario.
    pub fn get_scenario(&self, scenario_index: usize) -> &ScenarioConfig {
        self.0.get(scenario_index).unwrap()
    }
}

/// Scenario timer resource.
#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioTimer(pub Timer);

/// Animation node configuration.
/// A single node in an animation. Construct this using the builder pattern.
pub struct AnimationNodeConfig {
    /// The duration in seconds of this section of the animation.
    pub duration: f32,
    /// The end transformation value.
    pub transform: Transform,
    /// The function to model this section of the animation transformation.
    pub animation_fn: AnimationFn,
}

/// Animation configuration.
pub struct AnimationConfig {
    /// An optional lever-state-related condition to decide whether to run the
    /// animation.
    pub lever_state_condition: Option<LeverState>,
    /// The animation start transformation.
    pub start_transform: Transform,
    /// The collection of animation nodes.
    pub nodes: Vec<AnimationNodeConfig>,
}

/// Resource containing animation configuration for all scenarios.
#[derive(Resource, Deref, DerefMut)]
pub struct AnimationConfigRes(pub Vec<Vec<AnimationConfig>>);

/// Resource containing the timer for a section of an animation.
#[derive(Resource, Deref, DerefMut)]
pub struct AnimationSectionTimer(pub Timer);

/// Resource containing a timer to delay the end of the scenario.
#[derive(Resource, Deref, DerefMut)]
pub struct PostAnimationTimer(pub Timer);

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
