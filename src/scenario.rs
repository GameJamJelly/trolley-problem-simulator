//! Trolley problem scenario implementation.

use crate::animation::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use crate::util::*;
use bevy::audio::{PlaybackMode, Volume};
use bevy::ecs::schedule::SystemConfigs;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::sync::Mutex;
use std::time::Duration;
use typed_builder::TypedBuilder;

/// Sets up a scenario.
pub fn scenario_setup(
    mut commands: Commands,
    scenarios_config: Res<ScenariosConfigRes>,
    scenario_index_state: Res<State<ScenarioIndexState>>,
    image_assets: Res<ImageAssetMap>,
    trolley_front_texture: Res<TrolleyFrontRes>,
    mut next_lever_state: ResMut<NextState<LeverState>>,
) {
    let scenario_index = scenario_index_state.0.unwrap();
    let scenario = scenarios_config.get_scenario(scenario_index);
    let tracks_normal_texture = image_assets.get_by_name(&scenario.tracks_normal_texture);
    let lever_player_normal_texture = image_assets.get_by_name(&scenario.lever_normal_texture);
    let hostages_track_a_normal_texture = scenario
        .hostages_track_a_normal_texture
        .as_ref()
        .map(|texture| image_assets.get_by_name(texture));
    let hostages_track_b_normal_texture = scenario
        .hostages_track_b_normal_texture
        .as_ref()
        .map(|texture| image_assets.get_by_name(texture));
    let trolley_texture_override = scenario
        .trolley_texture_override
        .as_ref()
        .map(|texture| image_assets.get_by_name(texture));
    let trolley_texture = trolley_texture_override.unwrap_or_else(|| trolley_front_texture.clone());
    let duration = Duration::from_secs_f32(scenario.duration);

    // Reset the lever state
    next_lever_state.set(LeverState::Normal);

    // Keep a collection of all entities spawned
    let mut entities = Vec::new();

    // Spawn the track texture
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: tracks_normal_texture.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -20.0),
                    ..default()
                },
                TrackTexture,
            ))
            .id(),
    );

    // Spawn the lever/player texture
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: lever_player_normal_texture.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -10.0),
                    ..default()
                },
                LeverPlayerTexture,
            ))
            .id(),
    );

    // Spawn the texture for the hostages on track A
    if let Some(texture) = hostages_track_a_normal_texture {
        if let Some(pos) = scenario.hostages_track_a_pos {
            entities.push(
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(
                                normalize_translation_to_canvas_with_z(pos, -10.0),
                            ),
                            ..default()
                        },
                        HostagesTrackATexture,
                    ))
                    .id(),
            );
        }
    }

    // Spawn the texture for the hostages on track B
    if let Some(texture) = hostages_track_b_normal_texture {
        if let Some(pos) = scenario.hostages_track_b_pos {
            entities.push(
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(
                                normalize_translation_to_canvas_with_z(pos, -10.0),
                            ),
                            ..default()
                        },
                        HostagesTrackBTexture,
                    ))
                    .id(),
            );
        }
    }

    // Spawn the trolley texture
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: trolley_texture,
                    transform: horizon_distance_transform(
                        APPROACHING_TROLLEY_HORIZON_POINT,
                        APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
                        duration,
                    ),
                    ..default()
                },
                TrolleyTexture,
            ))
            .id(),
    );

    // Spawn the scenario text
    entities.push(
        commands
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(24.0)),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Vw(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        scenario.text.clone(),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                );
            })
            .id(),
    );

    // Spawn the timer text
    entities.push(
        commands
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(8.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        format_timer_text(duration),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Right),
                    TimerText,
                ));
            })
            .id(),
    );

    // Insert a resource containing all entities spawned so we can
    // remove them later
    commands.insert_resource(ScenarioEntitiesRes(entities));

    // Insert the timer resource
    commands.insert_resource(ScenarioTimer(Timer::from_seconds(
        scenario.duration,
        TimerMode::Once,
    )));
}

/// Updates a scenario every game tick.
pub fn scenario_update(
    time: Res<Time>,
    mut timer: ResMut<ScenarioTimer>,
    mut timer_text: Query<&mut Text, With<TimerText>>,
    mut trolley_transform: Query<&mut Transform, With<TrolleyTexture>>,
    mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
    trolley_turn_texture: Res<TrolleyTurnRes>,
    trolley_side_texture: Res<TrolleySideRes>,
    mut next_animation_state: ResMut<NextState<AnimationState>>,
    scenarios_config: Res<ScenariosConfigRes>,
    scenario_index_state: Res<State<ScenarioIndexState>>,
) {
    let scenario_index = scenario_index_state.0.unwrap();
    let scenario = scenarios_config.get_scenario(scenario_index);
    let trolley_texture_overridden = scenario.trolley_texture_override.is_some();

    let previous_time_remaining = timer.remaining_secs();

    // Advance the state of the timer, checking if time just ran out
    if timer.tick(time.delta()).just_finished() {
        next_animation_state.set(AnimationState::Running);
    }

    let current_time_remaining = timer.remaining_secs();

    // Update the timer text
    timer_text.single_mut().sections[0].value =
        format_timer_text(timer.remaining().max(Duration::from_secs(0)));

    // Trigger the trolley to turn slightly
    if time_remaining_reached(previous_time_remaining, current_time_remaining, 3.0)
        && !trolley_texture_overridden
    {
        *trolley_texture.single_mut() = trolley_turn_texture.clone();
    }

    // Trigger the trolley to turn sideways
    if time_remaining_reached(previous_time_remaining, current_time_remaining, 2.0)
        && !trolley_texture_overridden
    {
        *trolley_texture.single_mut() = trolley_side_texture.clone();
    }

    // Update the trolley transform
    if timer.remaining_secs() > 3.0 {
        let new_transform = horizon_distance_transform(
            APPROACHING_TROLLEY_HORIZON_POINT,
            APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
            timer.remaining() - Duration::from_secs(3),
        );
        *trolley_transform.single_mut() = new_transform;
    } else if timer.remaining_secs() > 2.0 {
        let new_transform = movement_transform(
            APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
            APPROACHING_TROLLEY_TURNING_END_TRANSFORM,
            Duration::from_secs_f32(1.0),
            timer.remaining() - Duration::from_secs_f32(2.0),
        );
        *trolley_transform.single_mut() = new_transform;
    } else if timer.remaining_secs() > 0.0 {
        let new_transform = movement_transform(
            APPROACHING_TROLLEY_TURNING_END_TRANSFORM,
            APPROACHING_TROLLEY_SIDE_END_TRANSFORM,
            Duration::from_secs_f32(2.0),
            timer.remaining(),
        );
        *trolley_transform.single_mut() = new_transform;
    }
}

/// Handles click events in a scenario.
pub fn scenario_handle_click(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    lever_state: Res<State<LeverState>>,
    mut next_lever_state: ResMut<NextState<LeverState>>,
    mut texture_set: ParamSet<(
        Query<&mut Handle<Image>, With<TrackTexture>>,
        Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
    )>,
    scenarios_config: Res<ScenariosConfigRes>,
    scenario_index_state: Res<State<ScenarioIndexState>>,
    image_assets: Res<ImageAssetMap>,
    audio_assets: Res<AudioAssetMap>,
) {
    let scenario_index = scenario_index_state.0.unwrap();
    let scenario = scenarios_config.get_scenario(scenario_index);
    let tracks_normal_texture = image_assets.get_by_name(&scenario.tracks_normal_texture);
    let tracks_switched_texture = scenario
        .tracks_switched_texture
        .as_ref()
        .map(|texture| image_assets.get_by_name(texture));
    let lever_player_normal_texture = image_assets.get_by_name(&scenario.lever_normal_texture);
    let lever_player_switched_texture = scenario
        .lever_switched_texture
        .as_ref()
        .map(|texture| image_assets.get_by_name(texture));
    let switch_audio = audio_assets.get_by_name("switch");

    let lever_rect = Rect::new(346.0, 135.0, 410.0, 202.0);

    if let Some(mouse_pos) = windows.single().cursor_position() {
        if lever_rect.contains(mouse_pos) {
            commands.spawn(AudioBundle {
                source: switch_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    ..default()
                },
            });

            match lever_state.get() {
                LeverState::Normal => {
                    if let Some(tracks_texture) = tracks_switched_texture {
                        if let Some(lever_texture) = lever_player_switched_texture {
                            next_lever_state.set(LeverState::Pulled);
                            *texture_set.p0().single_mut() = tracks_texture;
                            *texture_set.p1().single_mut() = lever_texture;
                        }
                    }
                }
                LeverState::Pulled => {
                    next_lever_state.set(LeverState::Normal);
                    *texture_set.p0().single_mut() = tracks_normal_texture;
                    *texture_set.p1().single_mut() = lever_player_normal_texture;
                }
            }
        }
    }
}

/// Cleans up a scenario.
pub fn scenario_cleanup(mut commands: Commands, entities: Res<ScenarioEntitiesRes>) {
    // Despawn the entities
    for entity in &**entities {
        let entity_commands = commands.entity(*entity);
        entity_commands.despawn_recursive();
    }

    // Remove the entities resource
    commands.remove_resource::<ScenarioEntitiesRes>();

    // Remove the scenario timer
    commands.remove_resource::<ScenarioTimer>();
}

/// Triggers when the animation has completed and prepares to go to the next
/// scenario.
fn animation_complete(mut commands: Commands) {
    commands.insert_resource(PostAnimationTimer(Timer::from_seconds(
        POST_ANIMATION_WAIT_TIME,
        TimerMode::Once,
    )));
}

/// Sets the scenario index state once [`GameState::Playing`] is entered.
fn set_scenario_index_state(mut next_scenario_index_state: ResMut<NextState<ScenarioIndexState>>) {
    next_scenario_index_state.set(ScenarioIndexState(Some(0)));
}

/// Unsets the scenario index state once [`GameState::Playing`] is exited.
fn unset_scenario_index_state(
    mut next_scenario_index_state: ResMut<NextState<ScenarioIndexState>>,
) {
    next_scenario_index_state.set(ScenarioIndexState(None));
}

/// Immediately goes to the next scenario.
fn post_animation_wait(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PostAnimationTimer>,
    scenario_index_state: Res<State<ScenarioIndexState>>,
    mut next_scenario_index_state: ResMut<NextState<ScenarioIndexState>>,
    mut next_animation_state: ResMut<NextState<AnimationState>>,
) {
    if timer.tick(time.delta()).just_finished() {
        next_scenario_index_state.set(ScenarioIndexState(Some(
            scenario_index_state.0.unwrap() + 1,
        )));
        next_animation_state.set(AnimationState::Waiting);
        commands.remove_resource::<PostAnimationTimer>();
    }
}

/// Immediately sets the game state to [`GameState::EndScreen`].
fn goto_end_scenario(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::EndScreen);
}

/// A trolley problem scenario. Construct this using the builder pattern.
#[derive(TypedBuilder)]
#[builder(mutators(
    /// Adds an animation to the scenario.
    pub fn animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    /// Overrides the trolley texture.
    pub fn override_trolley_texture(&mut self, texture: impl Into<String>) {
        self.trolley_texture_override = Some(texture.into());
    }

    /// Overrides the track A hostages scream sound.
    pub fn override_hostages_a_scream_sound(&mut self, sound: impl Into<String>) {
        self.hostages_a_scream_sound_override = Some(sound.into());
    }

    /// Overrides the track B hostages scream sound.
    pub fn override_hostages_b_scream_sound(&mut self, sound: impl Into<String>) {
        self.hostages_b_scream_sound_override = Some(sound.into());
    }

    /// Configures a system to run when the scenario begins.
    pub fn on_start<M>(&mut self, system: impl IntoSystemConfigs<M>) {
        self.on_start = Some(system.into_configs());
    }

    /// Configures a system to run while the scenario is running.
    pub fn on_update<M>(&mut self, system: impl IntoSystemConfigs<M>) {
        self.on_update = Some(system.into_configs());
    }

    /// Configures a system to run when the scenario ends.
    pub fn on_end<M>(&mut self, system: impl IntoSystemConfigs<M>) {
        self.on_end = Some(system.into_configs());
    }
))]
pub struct Scenario {
    /// The scenario text.
    #[builder(setter(into))]
    text: String,
    /// The scenario duration.
    duration: f32,
    /// The position of hostages on track A.
    #[builder(default, setter(strip_option))]
    hostages_track_a_pos: Option<Vec2>,
    /// The position of hostages on track B.
    #[builder(default, setter(strip_option))]
    hostages_track_b_pos: Option<Vec2>,
    /// The name of the normal track texture.
    #[builder(setter(into))]
    tracks_normal_texture: String,
    /// The name of the switched track texture.
    #[builder(default, setter(strip_option, into))]
    tracks_switched_texture: Option<String>,
    /// The name of the normal lever/player texture.
    #[builder(setter(into))]
    lever_normal_texture: String,
    /// The name of the switched lever/player texture.
    #[builder(default, setter(strip_option, into))]
    lever_switched_texture: Option<String>,
    /// The name of the track A hostages texture.
    #[builder(default, setter(strip_option, into))]
    hostages_track_a_normal_texture: Option<String>,
    /// The name of the track B hostages texture.
    #[builder(default, setter(strip_option, into))]
    hostages_track_b_normal_texture: Option<String>,
    /// An optional override on the trolley texture.
    #[builder(default, via_mutators)]
    trolley_texture_override: Option<String>,
    /// An optional override on the track A hostages scream sound.
    #[builder(default, via_mutators)]
    hostages_a_scream_sound_override: Option<String>,
    /// An optional override on the track B hostages scream sound.
    #[builder(default, via_mutators)]
    hostages_b_scream_sound_override: Option<String>,
    /// Whether to pause the music while the track A hostage scream plays.
    #[builder(default, setter(strip_option))]
    pause_music_during_hostages_a_scream: Option<f32>,
    /// Whether to pause the music while the track B hostage scream plays.
    #[builder(default, setter(strip_option))]
    pause_music_during_hostages_b_scream: Option<f32>,
    /// The collection of scenario animations.
    #[builder(default, via_mutators)]
    animations: Vec<Animation>,
    /// An optional system to run when the scenario begins.
    #[builder(default, via_mutators)]
    on_start: Option<SystemConfigs>,
    /// An optional system to run while the scenario is running.
    #[builder(default, via_mutators)]
    on_update: Option<SystemConfigs>,
    /// An optional system to run when the scenario ends.
    #[builder(default, via_mutators)]
    on_end: Option<SystemConfigs>,
}

/// A plugin to simplify the configuration of trolley problem scenarios.
/// Construct this using the builder pattern.
pub struct ScenarioCollectionPlugin {
    /// The list of scenarios. Normally, this could just be a `Vec<Scenario>`,
    /// but [`Plugin::build`] takes `&self`.
    scenarios: Mutex<Option<Vec<Scenario>>>,
}

impl ScenarioCollectionPlugin {
    /// Constructs a builder for this plugin.
    pub const fn builder() -> ScenarioCollectionPluginBuilder {
        ScenarioCollectionPluginBuilder::new()
    }
}

impl Plugin for ScenarioCollectionPlugin {
    fn build(&self, app: &mut App) {
        // Add scenario enter/exit systems
        app.add_systems(OnEnter(GameState::Playing), set_scenario_index_state);
        app.add_systems(OnExit(GameState::Playing), unset_scenario_index_state);

        // Add scenario systems
        let maybe_scenarios = self.scenarios.lock().unwrap().take();

        if let Some(scenarios) = maybe_scenarios {
            let num_scenarios = scenarios.len();

            let (scenario_config, animations_and_systems) = scenarios
                .into_iter()
                .map(|scenario| {
                    (
                        ScenarioConfig {
                            text: scenario.text,
                            duration: scenario.duration,
                            hostages_track_a_pos: scenario.hostages_track_a_pos,
                            hostages_track_b_pos: scenario.hostages_track_b_pos,
                            tracks_normal_texture: scenario.tracks_normal_texture,
                            tracks_switched_texture: scenario.tracks_switched_texture,
                            lever_normal_texture: scenario.lever_normal_texture,
                            lever_switched_texture: scenario.lever_switched_texture,
                            hostages_track_a_normal_texture: scenario
                                .hostages_track_a_normal_texture,
                            hostages_track_b_normal_texture: scenario
                                .hostages_track_b_normal_texture,
                            trolley_texture_override: scenario.trolley_texture_override,
                            hostages_a_scream_sound_override: scenario
                                .hostages_a_scream_sound_override,
                            hostages_b_scream_sound_override: scenario
                                .hostages_b_scream_sound_override,
                            pause_music_during_hostages_a_scream: scenario
                                .pause_music_during_hostages_a_scream,
                            pause_music_during_hostages_b_scream: scenario
                                .pause_music_during_hostages_b_scream,
                        },
                        (
                            scenario.animations,
                            (scenario.on_start, scenario.on_update, scenario.on_end),
                        ),
                    )
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

            let (animations, systems) = animations_and_systems
                .into_iter()
                .unzip::<_, _, Vec<_>, Vec<_>>();

            for (index, scenario_systems) in systems.into_iter().enumerate() {
                app.add_systems(OnEnter(ScenarioIndexState(Some(index))), scenario_setup)
                    .add_systems(
                        Update,
                        (
                            scenario_update.run_if(in_state(ScenarioIndexState(Some(index)))),
                            scenario_handle_click.run_if(
                                in_state(ScenarioIndexState(Some(index)))
                                    .and_then(in_state(AnimationState::Waiting))
                                    .and_then(input_just_pressed(MouseButton::Left)),
                            ),
                        ),
                    )
                    .add_systems(
                        OnEnter(AnimationState::Complete),
                        animation_complete.run_if(in_state(ScenarioIndexState(Some(index)))),
                    )
                    .add_systems(
                        Update,
                        post_animation_wait.run_if(
                            in_state(ScenarioIndexState(Some(index)))
                                .and_then(in_state(AnimationState::Complete)),
                        ),
                    )
                    .add_systems(OnExit(ScenarioIndexState(Some(index))), scenario_cleanup);

                if let Some(on_start) = scenario_systems.0 {
                    app.add_systems(OnEnter(ScenarioIndexState(Some(index))), on_start);
                }

                if let Some(on_update) = scenario_systems.1 {
                    app.add_systems(
                        Update,
                        on_update.run_if(in_state(ScenarioIndexState(Some(index)))),
                    );
                }

                if let Some(on_end) = scenario_systems.2 {
                    app.add_systems(OnExit(ScenarioIndexState(Some(index))), on_end);
                }
            }

            app.add_systems(
                OnEnter(ScenarioIndexState(Some(num_scenarios))),
                goto_end_scenario,
            );

            // Add all scenario configurations resource
            app.insert_resource(ScenariosConfigRes(scenario_config));

            // Add animations
            app.add_plugins(AnimationCollectionPlugin::new(animations));
        }
    }
}

/// A builder for the scenario collection plugin.
pub struct ScenarioCollectionPluginBuilder {
    /// The currently configured scenarios.
    scenarios: Vec<Scenario>,
}

impl ScenarioCollectionPluginBuilder {
    /// Constructs a new builder.
    pub const fn new() -> Self {
        Self {
            scenarios: Vec::new(),
        }
    }

    /// Adds a scenario to the plugin. Construct the scenario using the builder
    /// pattern.
    pub fn scenario(mut self, scenario: Scenario) -> Self {
        self.scenarios.push(scenario);
        self
    }

    /// Finalizes the scenario collection plugin.
    pub fn build(self) -> ScenarioCollectionPlugin {
        ScenarioCollectionPlugin {
            scenarios: Mutex::new(Some(self.scenarios)),
        }
    }
}
