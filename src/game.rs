//! Top-level game logic.

use crate::animation::*;
use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::end_screen::*;
use crate::menu::*;
use crate::resources::*;
use crate::scenario::*;
use crate::states::*;
use crate::summary::*;
use crate::util::*;
use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::time::Duration;

/// Loads game assets and stores them as resources.
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Store assets as resources
    commands.insert_resource(TrolleyFrontRes(load_embedded_image(
        &asset_server,
        "original-trolley-front",
    )));
    commands.insert_resource(TrolleyTurnRes(load_embedded_image(
        &asset_server,
        "original-trolley-turn",
    )));
    commands.insert_resource(TrolleySideRes(load_embedded_image(
        &asset_server,
        "original-trolley-side",
    )));
    commands.insert_resource(TrolleySwitchedRes(load_embedded_image(
        &asset_server,
        "original-trolley-switched",
    )));

    // UI camera
    commands.spawn(Camera2dBundle::default());
}

/// Turns the trolley as it goes to track B.
fn turn_trolley_switched_start(
    mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
    trolley_switched_texture: Res<TrolleySwitchedRes>,
) {
    *trolley_texture.single_mut() = trolley_switched_texture.clone();
}

/// Turns the trolley back to normal as it continues down track B.
fn turn_trolley_switched_end(
    mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
    trolley_side_texture: Res<TrolleySideRes>,
) {
    *trolley_texture.single_mut() = trolley_side_texture.clone();
}

/// Resumes the game music after the timer expires.
fn resume_music(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<GameMusicPauseTimerRes>>,
    music: Query<&AudioSink, With<GameMusic>>,
) {
    if let Some(mut timer) = timer {
        if timer.tick(time.delta()).just_finished() {
            music.single().play();
            commands.remove_resource::<GameMusicPauseTimerRes>();
        }
    }
}

/// Displays the configured wounded texture on track A when appropriate.
fn show_wounded_track_a(
    mut commands: Commands,
    scenarios_config: Res<ScenariosConfigRes>,
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    mut hostage_texture: Query<&mut Handle<Image>, With<HostagesTrackATexture>>,
    image_assets: Res<ImageAssetMap>,
    audio_assets: Res<AudioAssetMap>,
    music: Query<&AudioSink, With<GameMusic>>,
    trolley_approaching: Query<&AudioSink, With<TrolleyApproachingAudio>>,
) {
    let this_scenario = &scenarios_config[scenario_index.unwrap()];
    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];

    if let Some(wounded_texture_name) = &this_animation.wounded_texture {
        let wounded_texture = image_assets.get_by_name(wounded_texture_name);
        *hostage_texture.single_mut() = wounded_texture;

        for _ in 0..this_scenario.num_hostages_track_a {
            let scream_audio_name = match &this_scenario.hostages_a_scream_sound_override {
                Some(sound_name) => sound_name.clone(),
                None => format!("scream-{}", (rand::random::<usize>() % 24) + 1),
            };

            let scream_audio = audio_assets.get_by_name(&scream_audio_name);
            commands.spawn(AudioBundle {
                source: scream_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    ..default()
                },
            });
        }

        if let Some(duration) = this_scenario.pause_music_during_hostages_a_scream {
            music.single().pause();
            trolley_approaching.single().pause();
            commands.insert_resource(GameMusicPauseTimerRes(Timer::new(
                Duration::from_secs_f32(duration),
                TimerMode::Once,
            )));
        } else {
            let squash_audio = audio_assets.get_by_name("squash");
            commands.spawn(AudioBundle {
                source: squash_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    speed: 2.0,
                    ..default()
                },
            });
        }
    }
}

/// Displays the configured wounded texture on track B when appropriate.
fn show_wounded_track_b(
    mut commands: Commands,
    scenarios_config: Res<ScenariosConfigRes>,
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    mut hostage_texture: Query<&mut Handle<Image>, With<HostagesTrackBTexture>>,
    image_assets: Res<ImageAssetMap>,
    audio_assets: Res<AudioAssetMap>,
    music: Query<&AudioSink, With<GameMusic>>,
    trolley_approaching: Query<&AudioSink, With<TrolleyApproachingAudio>>,
) {
    let this_scenario = &scenarios_config[scenario_index.unwrap()];
    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];

    if let Some(wounded_texture_name) = &this_animation.wounded_texture {
        let wounded_texture = image_assets.get_by_name(wounded_texture_name);
        *hostage_texture.single_mut() = wounded_texture;

        for _ in 0..this_scenario.num_hostages_track_b {
            let scream_audio_name = match &this_scenario.hostages_b_scream_sound_override {
                Some(sound_name) => sound_name.clone(),
                None => format!("scream-{}", (rand::random::<usize>() % 24) + 1),
            };

            let scream_audio = audio_assets.get_by_name(&scream_audio_name);
            commands.spawn(AudioBundle {
                source: scream_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    ..default()
                },
            });
        }

        if let Some(duration) = this_scenario.pause_music_during_hostages_b_scream {
            music.single().pause();
            trolley_approaching.single().pause();
            commands.insert_resource(GameMusicPauseTimerRes(Timer::new(
                Duration::from_secs_f32(duration),
                TimerMode::Once,
            )));
        } else {
            let squash_audio = audio_assets.get_by_name("squash");
            commands.spawn(AudioBundle {
                source: squash_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    speed: 2.0,
                    ..default()
                },
            });
        }
    }
}

/// Generates a standard animation for the trolley on track A.
fn standard_animation_track_a(wounded_texture: Option<&str>) -> Animation {
    let mut animation = Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
        .on_lever_state(LeverState::Normal)
        .node(
            AnimationNode::new(
                2.0,
                Transform::from_xyz(
                    STANDARD_HOSTAGES_POS_TRACK_A.x,
                    STANDARD_HOSTAGES_POS_TRACK_A.y,
                    0.0,
                ),
            )
            .end_action(show_wounded_track_a),
        )
        .node(AnimationNode::new(
            4.0,
            Transform::from_xyz(900.0, 445.0, 0.0),
        ));

    if let Some(texture) = wounded_texture {
        animation = animation.with_wounded_texture(texture);
    }

    animation
}

/// Generates a standard animation for the trolley on track B.
fn standard_animation_track_b(wounded_texture: Option<&str>) -> Animation {
    let mut animation = Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
        .on_lever_state(LeverState::Pulled)
        .with_start_action(turn_trolley_switched_start)
        .node(
            AnimationNode::new(1.0, Transform::from_xyz(400.0, 190.0, 0.0))
                .end_action(turn_trolley_switched_end),
        )
        .node(
            AnimationNode::new(
                1.0,
                Transform::from_xyz(
                    STANDARD_HOSTAGES_POS_TRACK_B.x,
                    STANDARD_HOSTAGES_POS_TRACK_B.y,
                    0.0,
                ),
            )
            .end_action(show_wounded_track_b),
        )
        .node(AnimationNode::new(
            3.0,
            Transform::from_xyz(900.0, 260.0, 0.0),
        ));

    if let Some(texture) = wounded_texture {
        animation = animation.with_wounded_texture(texture);
    }

    animation
}

/// The transform function for the loop animation.
fn loop_animation(
    start_transform: Transform,
    _end_transform: Transform,
    progress: f32,
) -> Transform {
    let loop_radius = 70.0;
    let loop_center = Vec2::new(
        start_transform.translation.x,
        start_transform.translation.y - loop_radius,
    );
    let progress_angle = -std::f32::consts::TAU * (progress - 0.25);

    Transform::IDENTITY
        .with_translation(Vec3::new(
            loop_center.x + loop_radius * progress_angle.cos(),
            loop_center.y + loop_radius * progress_angle.sin(),
            0.0,
        ))
        .with_rotation(Quat::from_rotation_z(std::f32::consts::TAU * progress))
}

/// Cliff start system.
fn scenario_cliff_start(mut commands: Commands, image_assets: Res<ImageAssetMap>) {
    // Spawn the cliff asset
    let cliff_texture = image_assets.get_by_name("cliff");
    let cliff_entity = commands
        .spawn((
            SpriteBundle {
                texture: cliff_texture,
                transform: Transform::from_xyz(0.0, 0.0, -15.0),
                ..default()
            },
            CliffTexture,
        ))
        .id();
    commands.insert_resource(ScenarioExtraEntitiesRes(vec![cliff_entity]));
}

/// Cliff end system.
fn scenario_cliff_end(mut commands: Commands, entities: Res<ScenarioExtraEntitiesRes>) {
    // Despawn the assets
    for entity in &**entities {
        let entity_commands = commands.entity(*entity);
        entity_commands.despawn_recursive();
    }

    // Remove the entities resource
    commands.remove_resource::<ScenarioExtraEntitiesRes>();
}

/// Cool hat end system.
fn scenario_cool_hat_end(mut commands: Commands, lever_state: Res<State<LeverState>>) {
    if !lever_state.pulled() {
        commands.insert_resource(HatAcquiredRes);
    }
}

/// Loan forgiveness start system.
fn scenario_loan_forgiveness_start(
    mut commands: Commands,
    image_assets: Res<ImageAssetMap>,
    scenarios_config: Res<ScenariosConfigRes>,
    scenario_index_state: Res<State<ScenarioIndexState>>,
) {
    // Spawn the other hostages asset
    let other_hostages_texture = image_assets.get_by_name("age-hostage-10");
    let other_hostages_entity = commands
        .spawn((
            SpriteBundle {
                texture: other_hostages_texture,
                transform: normalize_transform_to_canvas(Transform::from_xyz(170.0, 210.0, -10.0)),
                ..default()
            },
            OtherHostagesTexture,
        ))
        .id();
    commands.insert_resource(ScenarioExtraEntitiesRes(vec![other_hostages_entity]));

    // Set a timer for swapping the other hostages asset
    let scenario_index = scenario_index_state.0.unwrap();
    let scenario = scenarios_config.get_scenario(scenario_index);
    let duration = Duration::from_secs_f32(scenario.duration - 1.5);
    commands.insert_resource(OtherHostagesTextureSwapTimerRes(Timer::new(
        duration,
        TimerMode::Once,
    )));
}

/// Loan forgiveness update system.
fn scenario_loan_forgiveness_update(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<OtherHostagesTextureSwapTimerRes>,
    image_assets: Res<ImageAssetMap>,
    audio_assets: Res<AudioAssetMap>,
    mut other_hostages_texture: Query<&mut Handle<Image>, With<OtherHostagesTexture>>,
) {
    if timer.tick(time.delta()).just_finished() {
        let other_hostages_wounded_texture = image_assets.get_by_name("age-hostage-10-wounded");
        *other_hostages_texture.single_mut() = other_hostages_wounded_texture;

        for _ in 0..10 {
            let scream_audio_name = format!("scream-{}", (rand::random::<usize>() % 24) + 1);
            let scream_audio = audio_assets.get_by_name(&scream_audio_name);
            commands.spawn(AudioBundle {
                source: scream_audio,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(GAME_VOLUME),
                    ..default()
                },
            });
        }
    }
}

/// Loan forgiveness end system.
fn scenario_loan_forgiveness_end(mut commands: Commands, entities: Res<ScenarioExtraEntitiesRes>) {
    // Despawn the assets
    for entity in &**entities {
        let entity_commands = commands.entity(*entity);
        entity_commands.despawn_recursive();
    }

    // Remove the entities resource
    commands.remove_resource::<ScenarioExtraEntitiesRes>();

    // Remove the other hostages texture swap timer
    commands.remove_resource::<OtherHostagesTextureSwapTimerRes>();
}

/// Double it start system.
fn scenario_double_it_start(mut commands: Commands, image_assets: Res<ImageAssetMap>) {
    let right_half_texture = image_assets.get_by_name("double-it-right-normal");
    let next_person_texture = image_assets.get_by_name("original-lever-normal");
    let hostage_2_texture = image_assets.get_by_name("double-it-hostage-2");

    let mut entities = Vec::new();

    // Spawn the right half of the screen
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: right_half_texture,
                    transform: Transform::from_xyz(0.0, 0.0, -20.0),
                    ..default()
                },
                DoubleItRightHalfTrackTexture,
            ))
            .id(),
    );

    // Spawn the next person
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: next_person_texture,
                    transform: Transform::from_xyz(280.0, -145.0, -10.0),
                    ..default()
                },
                NextPersonTexture,
            ))
            .id(),
    );

    // Spawn the double hostage texture
    entities.push(
        commands
            .spawn((
                SpriteBundle {
                    texture: hostage_2_texture,
                    transform: normalize_transform_to_canvas(Transform::from_xyz(
                        765.0, 335.0, -10.0,
                    )),
                    ..default()
                },
                DoubleItHostagesTexture,
            ))
            .id(),
    );

    // Insert the extra scenario entities resource
    commands.insert_resource(ScenarioExtraEntitiesRes(entities));

    // Insert the next person's lever state as a resource
    commands.insert_resource(NextPersonSwitchRes(false));

    // Insert the next person's switch timer resource
    commands.insert_resource(NextPersonSwitchTimerRes(Timer::new(
        random_switch_delay(),
        TimerMode::Once,
    )));
}

/// Double it update system.
fn scenario_double_it_update(
    time: Res<Time>,
    mut timer: ResMut<NextPersonSwitchTimerRes>,
    mut switch: ResMut<NextPersonSwitchRes>,
    mut texture_set: ParamSet<(
        Query<&mut Handle<Image>, With<DoubleItRightHalfTrackTexture>>,
        Query<&mut Handle<Image>, With<NextPersonTexture>>,
    )>,
    image_assets: Res<ImageAssetMap>,
    next_switch_reached: Option<Res<NextSwitchReachedRes>>,
) {
    #[allow(clippy::collapsible_if)]
    if next_switch_reached.is_none() {
        if timer.tick(time.delta()).just_finished() {
            **switch = !**switch;
            **timer = Timer::new(random_switch_delay(), TimerMode::Once);

            let (right_half_texture_name, next_person_texture_name) = if **switch {
                ("double-it-right-normal", "original-lever-normal")
            } else {
                ("double-it-right-switched", "original-lever-switched")
            };
            let right_half_texture = image_assets.get_by_name(right_half_texture_name);
            let next_person_texture = image_assets.get_by_name(next_person_texture_name);

            *texture_set.p0().single_mut() = right_half_texture;
            *texture_set.p1().single_mut() = next_person_texture;
        }
    }
}

/// Inserts the resource marking that the next switch has been reached.
fn set_next_switch_reached(
    mut commands: Commands,
    mut switch: ResMut<NextPersonSwitchRes>,
    mut texture_set: ParamSet<(
        Query<&mut Handle<Image>, With<DoubleItRightHalfTrackTexture>>,
        Query<&mut Handle<Image>, With<NextPersonTexture>>,
    )>,
    image_assets: Res<ImageAssetMap>,
) {
    // Insert the marker resource
    commands.insert_resource(NextSwitchReachedRes);

    // Set the next switch to normal
    **switch = false;

    let right_half_texture = image_assets.get_by_name("double-it-right-normal");
    let next_person_texture = image_assets.get_by_name("original-lever-normal");

    *texture_set.p0().single_mut() = right_half_texture;
    *texture_set.p1().single_mut() = next_person_texture;
}

/// Double it end system.
fn scenario_double_it_end(mut commands: Commands, entities: Res<ScenarioExtraEntitiesRes>) {
    // Despawn the assets
    for entity in &**entities {
        let entity_commands = commands.entity(*entity);
        entity_commands.despawn_recursive();
    }

    // Remove the entities resource
    commands.remove_resource::<ScenarioExtraEntitiesRes>();

    // Remove the next person's switch resource
    commands.remove_resource::<NextPersonSwitchRes>();

    // Remove the next person's switch timer resource
    commands.remove_resource::<NextPersonSwitchTimerRes>();

    // Remove the next switch reached marker resource
    commands.remove_resource::<NextSwitchReachedRes>();
}

/// Youtube prank start system.
fn scenario_youtube_prank_start(mut commands: Commands, image_assets: Res<ImageAssetMap>) {
    // Spawn the tripod asset
    let tripod_texture = image_assets.get_by_name("youtube-prank-tripod");
    let tripod_entity = commands
        .spawn((
            SpriteBundle {
                texture: tripod_texture,
                transform: normalize_transform_to_canvas(Transform::from_xyz(470.0, 415.0, 15.0)),
                ..default()
            },
            TripodTexture,
        ))
        .id();
    commands.insert_resource(ScenarioExtraEntitiesRes(vec![tripod_entity]));
}

/// Youtube prank end system.
fn scenario_youtube_prank_end(mut commands: Commands, entities: Res<ScenarioExtraEntitiesRes>) {
    // Despawn the assets
    for entity in &**entities {
        let entity_commands = commands.entity(*entity);
        entity_commands.despawn_recursive();
    }

    // Remove the entities resource
    commands.remove_resource::<ScenarioExtraEntitiesRes>();

    // Prevent the display of the cool hat texture
    commands.remove_resource::<HatAcquiredRes>();
}

/// Self start system.
fn scenario_self_start(mut commands: Commands) {
    // Insert the self jumping resource
    commands.insert_resource(SelfJumping::NotJumping);
}

/// Self update system.
fn scenario_self_update(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut jumping: ResMut<SelfJumping>,
    animation_state: Res<State<AnimationState>>,
    image_assets: Res<ImageAssetMap>,
    audio_assets: Res<AudioAssetMap>,
    mut self_texture: Query<(&mut Handle<Image>, &mut Transform), With<LeverPlayerTexture>>,
) {
    match **animation_state {
        AnimationState::Waiting => {
            if buttons.just_pressed(MouseButton::Left) {
                *jumping = SelfJumping::Jumping;

                let player_texture = image_assets.get_by_name("self");
                *self_texture.single_mut().0 = player_texture;
                *self_texture.single_mut().1 = normalize_transform_to_canvas(SELF_JUMP_TRANSFORM);
            }
        }
        AnimationState::Running => {
            if *jumping == SelfJumping::Jumping {
                *jumping = SelfJumping::RunOver;

                let player_texture = image_assets.get_by_name("self-wounded");
                *self_texture.single_mut().0 = player_texture;
                *self_texture.single_mut().1 = normalize_transform_to_canvas(SELF_JUMP_TRANSFORM);

                let scream_audio = audio_assets.get_by_name("self-scream");
                commands.spawn(AudioBundle {
                    source: scream_audio,
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(GAME_VOLUME),
                        ..default()
                    },
                });
            }
        }
        AnimationState::Complete => {}
    }
}

/// Self end system.
fn scenario_self_end(mut commands: Commands) {
    // Remove the self jumping resource
    commands.remove_resource::<SelfJumping>();
}

/// The plugin which orchestrates the game logic.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add app plugins
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".to_owned()),
                    resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin,
        ));

        // Insert stateful values
        app.insert_state(GameState::Initializing)
            .insert_state(ScenarioIndexState(None))
            .insert_state(LeverState::Normal);

        // Add game setup and menu screen systems
        app.add_systems(Startup, setup_game);
        app.add_systems(OnEnter(GameState::InMenu), setup_menu_screen);
        app.add_systems(
            Update,
            update_menu_screen.run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(OnExit(GameState::InMenu), cleanup_menu_screen);

        // Add end screen systems
        app.add_systems(OnEnter(GameState::EndScreen), setup_end_screen);
        app.add_systems(
            Update,
            update_end_screen.run_if(in_state(GameState::EndScreen)),
        );
        app.add_systems(OnExit(GameState::EndScreen), cleanup_end_screen);

        // Add system to resume music whenever paused
        app.add_systems(Update, resume_music);

        // Original
        let scenario_original = Scenario::builder()
            .text("A trolley is headed towards a group of five people. You can intervene and click on the lever to pull it and switch the tracks so that only one person will be killed. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(5)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_original)
            .build();

        // Age
        let scenario_age = Scenario::builder()
            .text("Everyone on the lower track is 90 years old. There is a child on the upper track. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("age-hostage-10")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(10)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(Some("age-hostage-10-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_age)
            .build();

        // Clone
        let scenario_clone = Scenario::builder()
            .text("If you pull the lever, the trolley will kill a clone of Hitler. The clone has all of Hitler's memories and fully believes that they are him and that they committed all of his actions, even though they didn't. Do you pull the lever?")
            .duration(25.0)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(0)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(None))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_clone)
            .build();

        // Cliff
        let scenario_cliff = Scenario::builder()
            .text("Hitler is the only passenger on the trolley. If you do nothing, five innocent people will be killed, and the trolley will fall off the cliff and explode. If you pull the lever, the innocents will be spared, but Hitler will escape to freedom.")
            .duration(25.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .num_hostages_track_a(5)
            .num_hostages_track_b(0)
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .on_lever_state(LeverState::Normal)
                    .with_wounded_texture("original-hostage-5-wounded")
                    .node(
                        AnimationNode::new(
                            2.0,
                            Transform::from_xyz(STANDARD_HOSTAGES_POS_TRACK_A.x, STANDARD_HOSTAGES_POS_TRACK_A.y, 0.0))
                        .end_action(show_wounded_track_a))
                    .node(
                        AnimationNode::new(
                            1.0,
                            Transform::from_xyz(715.0, 375.0, 0.0)))
                    .node(
                        AnimationNode::new(
                            2.0,
                            Transform::IDENTITY
                                .with_translation(Vec3::new(785.0, 460.0, 0.0))
                                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                                .with_rotation(Quat::from_rotation_z(-0.375 * std::f32::consts::TAU)))))
            .animation(standard_animation_track_b(None))
            .on_start(scenario_cliff_start)
            .on_end((scenario_cliff_end, update_summary_cliff))
            .build();

        // Cool hat
        let scenario_cool_hat = Scenario::builder()
            .text("You can pull the lever to save five people, but you would kill the guy with the really cool hat. Do you pull the lever?")
            .duration(15.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("hat-hostage")
            .num_hostages_track_a(5)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("hat-hostage-wounded")))
            .on_end((scenario_cool_hat_end, update_summary_cool_hat))
            .build();

        // Victim
        let scenario_victim = Scenario::builder()
            .text("The person on the track claims that \"Society needs to pull the lever.\" You have told them to just walk off the tracks. Is this person really the victim if they have knowingly done this to themselves? Will you be responsible if they die?")
            .duration(25.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("victim")
            .num_hostages_track_a(1)
            .num_hostages_track_b(0)
            .animation(standard_animation_track_a(Some("victim-wounded")))
            .animation(standard_animation_track_b(None))
            .on_end(update_summary_victim)
            .build();

        // Darwinism
        let scenario_darwinism = Scenario::builder()
            .text("A person on the lower track is not tied down, and can walk away if he is smart enough to recognize the danger of the trolley. If he is not smart enough, natural selection will do its job and eliminate him from the gene pool, making humanity smarter. The upper track has 5 untied persons, which could potentially apply natural selection to 4 extra people, making humanity even smarter. Do you pull the lever?")
            .duration(30.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("darwinism-hostage-1")
            .hostages_track_b_normal_texture("darwinism-hostage-5")
            .num_hostages_track_a(1)
            .num_hostages_track_b(5)
            .animation(standard_animation_track_a(Some("darwinism-hostage-1-wounded")))
            .animation(standard_animation_track_b(Some("darwinism-hostage-5-wounded")))
            .on_end(update_summary_darwinism)
            .build();

        // Loop
        let scenario_loop = Scenario::builder()
            .text("A trolley is headed towards a group of five people. You can pull the lever to only kill one person, but then the trolley won't do the totally sick loop-da-loop. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("loop-normal")
            .tracks_switched_texture("loop-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(5)
            .num_hostages_track_b(1)
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .on_lever_state(LeverState::Normal)
                    .with_wounded_texture("original-hostage-5-wounded")
                    .node(
                        AnimationNode::new(
                            2.0,
                            Transform::from_xyz(STANDARD_HOSTAGES_POS_TRACK_A.x, STANDARD_HOSTAGES_POS_TRACK_A.y, 0.0))
                        .end_action(show_wounded_track_a))
                    .node(AnimationNode::new(0.25, Transform::from_xyz(570.0, 305.0, 0.0)))
                    .node(AnimationNode::new(2.0, Transform::from_xyz(570.0, 305.0, 0.0)).animation_fn(loop_animation))
                    .node(AnimationNode::new(2.0, Transform::from_xyz(900.0, 445.0, 0.0))))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_loop)
            .build();

        // Professors
        let scenario_professors = Scenario::builder()
            .text("A trolley is headed towards five philosophy professors who like to invent trolley problems. You can pull the lever to switch the track, but then the trolley will kill the nice guy who never had a thought of inventing trolley problems.")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(5)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_professors)
            .build();

        // Loan forgiveness
        let scenario_loan_forgiveness = Scenario::builder()
            .text("The trolley is going to run over ten people. Unless you intervene, it will run over an additional five people. But if you divert the trolley, wouldn't that be unfair to all the people it will have already killed?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .num_hostages_track_a(5)
            .num_hostages_track_b(0)
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(None))
            .on_start(scenario_loan_forgiveness_start)
            .on_update(scenario_loan_forgiveness_update)
            .on_end((scenario_loan_forgiveness_end, update_summary_loan_forgiveness))
            .build();

        // Lobster
        let scenario_lobster = Scenario::builder()
            .text("A trolley is headed towards a group of five lobsters. Are you really going to let five innocent lobsters die?")
            .duration(15.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("lobster-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(1)
            .num_hostages_track_b(1)
            .override_hostages_a_scream_sound("blue-lobster")
            .pause_music_during_hostages_a_scream(5.5)
            .animation(standard_animation_track_a(Some("lobster-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_lobster)
            .build();

        // Shopping cart
        let scenario_shopping_cart = Scenario::builder()
            .text("There is no dire emergency. Do you accept your duty to return the cart even though you gain nothing?")
            .duration(15.0)
            .tracks_normal_texture("shopping-cart-tracks-normal")
            .tracks_switched_texture("shopping-cart-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .num_hostages_track_a(0)
            .num_hostages_track_b(0)
            .override_trolley_texture("shopping-cart")
            .animation(standard_animation_track_a(None))
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .on_lever_state(LeverState::Pulled)
                    .node(AnimationNode::new(1.0, Transform::from_xyz(400.0, 190.0, 0.0)))
                    .node(AnimationNode::new(2.0, Transform::from_xyz(595.0, 240.0, 0.0)))
                    .node(AnimationNode::new(1.0, Transform::from_xyz(680.0, 180.0, 0.0))))
            .on_end(update_summary_shopping_cart)
            .build();

        // Born lever puller
        let scenario_born_lever_puller = Scenario::builder()
            .text("If you do nothing, nobody will get hurt. However, you are a born lever-puller. Do you pull the lever?")
            .duration(15.0)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("born-lever-puller-normal")
            .lever_switched_texture("born-lever-puller-switched")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(0)
            .num_hostages_track_b(1)
            .animation(standard_animation_track_a(None))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .on_end(update_summary_born_lever_puller)
            .build();

        // Double it
        let scenario_double_it = Scenario::builder()
            .text("Would you kill one person or double it and give it to the next person?")
            .duration(15.0)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("double-it-left-normal")
            .tracks_switched_texture("double-it-left-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_b_normal_texture("original-hostage-1")
            .num_hostages_track_a(0)
            .num_hostages_track_b(1)
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .on_lever_state(LeverState::Normal)
                    .node(
                        AnimationNode::new(3.0, NEXT_PERSON_SWITCH_TRANSFORM)
                            .end_action(set_next_switch_reached),
                    )
                    .node(AnimationNode::new(
                        2.0,
                        Transform::from_xyz(900.0, 445.0, 0.0),
                    )),
            )
            .animation(standard_animation_track_b(Some(
                "original-hostage-1-wounded",
            )))
            .on_start(scenario_double_it_start)
            .on_update(scenario_double_it_update)
            .on_end((scenario_double_it_end, update_summary_double_it))
            .build();

        // Thomas the tank engine
        let scenario_thomas_the_tank_engine = Scenario::builder()
            .text("There is only one track. Everyone is in danger and there is nothing you can do to save anyone. Also the trolley is Thomas the Tank Engine.")
            .duration(10.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("self-one-track")
            .lever_normal_texture("self-standing")
            .hostages_track_a_normal_texture("original-hostage-5")
            .num_hostages_track_a(5)
            .num_hostages_track_b(0)
            .override_trolley_texture("thomas-the-tank-engine")
            .override_hostages_a_scream_sound("thomas-theme")
            .pause_music_during_hostages_a_scream(4.0)
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .with_wounded_texture("original-hostage-5-wounded")
                    .node(
                        AnimationNode::new(
                            2.0,
                            Transform::from_xyz(
                                STANDARD_HOSTAGES_POS_TRACK_A.x,
                                STANDARD_HOSTAGES_POS_TRACK_A.y,
                                0.0,
                            ),
                        )
                        .end_action(show_wounded_track_a),
                    )
                    .node(AnimationNode::new(
                        4.0,
                        Transform::from_xyz(900.0, 445.0, 0.0),
                    )))
            .on_end(update_summary_thomas_the_tank_engine)
            .build();

        // Youtube prank
        let scenario_youtube_prank = Scenario::builder()
            .text("Five reaction YouTubers tied themselves to the tracks and a trolley is heading straight for them. You notice the person tied to the other track is a dummy. Do you pull the lever to save them, contributing to their viral prank?")
            .duration(25.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("youtube-prank-youtubers")
            .hostages_track_b_normal_texture("youtube-prank-dummy")
            .num_hostages_track_a(5)
            .num_hostages_track_b(0)
            .animation(standard_animation_track_a(Some("youtube-prank-youtubers-wounded")))
            .animation(standard_animation_track_b(Some("youtube-prank-dummy-wounded")))
            .on_start(scenario_youtube_prank_start)
            .on_end((scenario_youtube_prank_end, update_summary_youtube_prank))
            .build();

        // Self
        let scenario_self = Scenario::builder()
            .text("Nobody is in any danger. Do you jump in front of the moving trolley?")
            .duration(10.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("self-one-track")
            .lever_normal_texture("self-standing")
            .num_hostages_track_a(0)
            .num_hostages_track_b(0)
            .animation(Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM).node(
                AnimationNode::new(6.0, Transform::from_xyz(900.0, 445.0, 0.0)),
            ))
            .on_start(scenario_self_start)
            .on_update(scenario_self_update)
            .on_end((update_summary_self, scenario_self_end).chain())
            .build();

        // Add scenarios
        app.add_plugins(
            ScenarioCollectionPlugin::builder()
                .scenario(scenario_original)
                .scenario(scenario_age)
                .scenario(scenario_clone)
                .scenario(scenario_cliff)
                .scenario(scenario_cool_hat)
                .scenario(scenario_victim)
                .scenario(scenario_darwinism)
                .scenario(scenario_loop)
                .scenario(scenario_professors)
                .scenario(scenario_loan_forgiveness)
                .scenario(scenario_lobster)
                .scenario(scenario_shopping_cart)
                .scenario(scenario_born_lever_puller)
                .scenario(scenario_double_it)
                .scenario(scenario_thomas_the_tank_engine)
                .scenario(scenario_youtube_prank)
                .scenario(scenario_self)
                .build(),
        );
    }
}
