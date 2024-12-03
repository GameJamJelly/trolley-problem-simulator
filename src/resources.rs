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
    /// List of audio asset paths.
    pub audio_asset_paths: Vec<String>,
    /// List of music asset paths.
    pub music_asset_paths: Vec<String>,
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

/// A map of audio asset paths to their corresponding handles.
#[derive(Resource, Deref, DerefMut)]
pub struct AudioAssetMap(pub HashMap<String, Handle<AudioSource>>);

impl AudioAssetMap {
    /// Gets an audio asset by its full path.
    pub fn get_by_path(&self, path: &str) -> Handle<AudioSource> {
        self.0.get(path).unwrap().clone()
    }

    /// Gets an audio asset by its name. This assumes that the audio is in
    /// `assets/sounds/` and is an MP3.
    pub fn get_by_name(&self, name: &str) -> Handle<AudioSource> {
        self.get_by_path(&format!("assets/sounds/{}.mp3", name))
    }
}

/// A map of music asset paths to their corresponding handles.
#[derive(Resource, Deref, DerefMut)]
pub struct MusicAssetMap(pub HashMap<String, Handle<AudioSource>>);

impl MusicAssetMap {
    /// Gets a music asset by its full path.
    pub fn get_by_path(&self, path: &str) -> Handle<AudioSource> {
        self.0.get(path).unwrap().clone()
    }

    /// Gets a music asset by its name. This assumes that the music is in
    /// `assets/music/` and is an MP3.
    pub fn get_by_name(&self, name: &str) -> Handle<AudioSource> {
        self.get_by_path(&format!("assets/music/{}.mp3", name))
    }
}

/// Scenario configuration.
pub struct ScenarioConfig {
    /// The scenario text.
    pub text: String,
    /// The scenario duration.
    pub duration: f32,
    /// The position of hostages on track A.
    pub hostages_track_a_pos: Option<Vec2>,
    /// The position of hostages on track B.
    pub hostages_track_b_pos: Option<Vec2>,
    /// The name of the normal track texture.
    pub tracks_normal_texture: String,
    /// The name of the switched track texture.
    pub tracks_switched_texture: Option<String>,
    /// The name of the normal lever/player texture.
    pub lever_normal_texture: String,
    /// The name of the switched lever/player texture.
    pub lever_switched_texture: Option<String>,
    /// The name of the track A hostages texture.
    pub hostages_track_a_normal_texture: Option<String>,
    /// The name of the track B hostages texture.
    pub hostages_track_b_normal_texture: Option<String>,
    /// The number of hostages on track A.
    pub num_hostages_track_a: usize,
    /// The number of hostages on track B.
    pub num_hostages_track_b: usize,
    /// The overridden trolley texture.
    pub trolley_texture_override: Option<String>,
    /// The overridden track A hostages scream sound.
    pub hostages_a_scream_sound_override: Option<String>,
    /// The overridden track B hostages scream sound.
    pub hostages_b_scream_sound_override: Option<String>,
    /// Whether to pause the music while the track B hostage scream plays.
    pub pause_music_during_hostages_a_scream: Option<f32>,
    /// Whether to pause the music while the track B hostage scream plays.
    pub pause_music_during_hostages_b_scream: Option<f32>,
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
    /// The optional wounded texture.
    pub wounded_texture: Option<String>,
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

/// The resource containing extra entities spawned for a scenario.
#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioExtraEntitiesRes(pub Vec<Entity>);

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

/// The resource for the lever-pulled turning trolley texture.
#[derive(Resource, Deref, DerefMut)]
pub struct TrolleySwitchedRes(pub Handle<Image>);

/// The double it next person's switch timer resource.
#[derive(Resource, Deref, DerefMut)]
pub struct NextPersonSwitchTimerRes(pub Timer);

/// The resource containing the next person's switch state.
#[derive(Resource, Deref, DerefMut)]
pub struct NextPersonSwitchRes(pub bool);

/// The marker resource to indicate that the next switch has been reached.
#[derive(Resource)]
pub struct NextSwitchReachedRes;

/// A resource to time when the other hostages texture swap should happen.
#[derive(Resource, Deref, DerefMut)]
pub struct OtherHostagesTextureSwapTimerRes(pub Timer);

/// A resource tracking whether the player is jumping onto the track.
#[derive(Resource, Default, PartialEq)]
pub enum SelfJumping {
    /// The player has not jumped.
    #[default]
    NotJumping,
    /// The player has jumped.
    Jumping,
    /// The player has been run over.
    RunOver,
}

impl SelfJumping {
    /// Returns whether the player jumped onto the tracks.
    pub const fn jumped(&self) -> bool {
        !matches!(self, Self::NotJumping)
    }
}

/// The summary of the game in progress.
#[derive(Resource)]
pub struct GameSummary {
    /// Total number of people killed.
    pub people_killed: usize,
    /// Total number of people saved.
    pub people_saved: usize,
    /// Whether Hitler was killed.
    pub killed_hitler: bool,
    /// Whether the player got the cool hat.
    pub got_cool_hat: bool,
    /// Whether the player caused a preventable tragedy.
    pub caused_preventable_tragedy: bool,
    /// Whether the player enforced Darwinism.
    pub enforced_darwinism: bool,
    /// Whether the player made the trolley do a sick loop-da-loop.
    pub did_sick_loop: bool,
    /// Total number of lobsters killed.
    pub lobsters_killed: usize,
    /// Total number of lobsters saved.
    pub lobsters_saved: usize,
    /// Whether the player returned the shopping cart.
    pub returned_shopping_cart: bool,
    /// Whether the player doubled it and gave it to the next person.
    pub doubled_it: bool,
    /// Whether the player watched Thomas the tank engine kill people.
    pub watched_thomas_kill_people: bool,
    /// Whether the player contributed to the viral YouTube prank.
    pub did_viral_prank: bool,
    /// Whether the player killed themself.
    pub killed_self: bool,
    /// Whether the player solved philosophy.
    pub solved_philosophy: bool,
}

impl GameSummary {
    /// Create a new game summary with default values.
    pub const fn new() -> Self {
        Self {
            people_killed: 0,
            people_saved: 0,
            killed_hitler: false,
            got_cool_hat: false,
            caused_preventable_tragedy: false,
            enforced_darwinism: false,
            did_sick_loop: false,
            lobsters_killed: 0,
            lobsters_saved: 0,
            returned_shopping_cart: false,
            doubled_it: false,
            watched_thomas_kill_people: false,
            killed_self: false,
            did_viral_prank: false,
            solved_philosophy: true,
        }
    }
}

/// A timer for pausing the game music.
#[derive(Resource, Deref, DerefMut)]
pub struct GameMusicPauseTimerRes(pub Timer);
