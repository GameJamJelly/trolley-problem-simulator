//! Trolley problem scenario implementation.

use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use crate::util::*;
use bevy::ecs::schedule::SystemConfigs;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::sync::Mutex;
use std::time::Duration;
use typed_builder::TypedBuilder;

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

// /// A trolley problem scenario.
// #[derive(Debug, Clone, PartialEq)]
// pub struct Scenario<
//     TracksNormal,
//     TracksSwitched,
//     LeverNormal,
//     LeverSwitched,
//     HostagesANormal,
//     HostagesBNormal,
// > where
//     TracksNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     TracksSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     LeverNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     LeverSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     HostagesANormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     HostagesBNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
// {
//     /// The scenario text.
//     pub text: String,
//     /// The duration of time, in seconds, which the player will have to decide.
//     pub duration: f32,
//     /// Pixel position (centered) of the hostages on track A.
//     pub hostages_track_a_pos: Vec2,
//     /// Pixel position (centered) of the hostages on track B.
//     pub hostages_track_b_pos: Vec2,
//     /// Marker to use the generics.
//     pub marker: PhantomData<
//         fn() -> (
//             Box<TracksNormal>,
//             Box<TracksSwitched>,
//             Box<LeverNormal>,
//             Box<LeverSwitched>,
//             Box<HostagesANormal>,
//             Box<HostagesBNormal>,
//         ),
//     >,
// }

// impl<
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
//     Scenario<
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// where
//     TracksNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     TracksSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     LeverNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     LeverSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     HostagesANormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     HostagesBNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
// {
//     /// Returns a function that, when called, will set up the scenario.
//     pub fn setup_fn(
//         &self,
//     ) -> impl FnMut(
//         Commands,
//         Res<TracksNormal>,
//         Res<LeverNormal>,
//         Res<HostagesANormal>,
//         Res<HostagesBNormal>,
//         Res<TrolleyFrontRes>,
//     ) + use<
//         '_,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     > {
//         let text = self.text.clone();
//         let duration_secs = self.duration.clone();
//         let hostages_track_a_pos = self.hostages_track_a_pos.clone();
//         let hostages_track_b_pos = self.hostages_track_b_pos.clone();

//         move |mut commands: Commands,
//               tracks_normal_texture: Res<TracksNormal>,
//               lever_player_normal_texture: Res<LeverNormal>,
//               hostages_track_a_normal_texture: Res<HostagesANormal>,
//               hostages_track_b_normal_texture: Res<HostagesBNormal>,
//               trolley_front_texture: Res<TrolleyFrontRes>| {
//             let duration = Duration::from_secs_f32(duration_secs);

//             // Spawn the track texture
//             let track_entity = commands
//                 .spawn((
//                     SpriteBundle {
//                         texture: tracks_normal_texture.clone(),
//                         ..default()
//                     },
//                     TrackTexture,
//                 ))
//                 .id();

//             // Spawn the lever/player texture
//             let lever_player_entity = commands
//                 .spawn((
//                     SpriteBundle {
//                         texture: lever_player_normal_texture.clone(),
//                         ..default()
//                     },
//                     LeverPlayerTexture,
//                 ))
//                 .id();

//             // Spawn the texture for the hostages on track A
//             let hostages_track_a_entity = commands
//                 .spawn((
//                     SpriteBundle {
//                         texture: hostages_track_a_normal_texture.clone(),
//                         transform: Transform::from_translation(normalize_translation_to_canvas(
//                             hostages_track_a_pos,
//                         )),
//                         ..default()
//                     },
//                     HostagesTrackATexture,
//                 ))
//                 .id();

//             // Spawn the texture for the hostages on track B
//             let hostages_track_b_entity = commands
//                 .spawn((
//                     SpriteBundle {
//                         texture: hostages_track_b_normal_texture.clone(),
//                         transform: Transform::from_translation(normalize_translation_to_canvas(
//                             hostages_track_b_pos,
//                         )),
//                         ..default()
//                     },
//                     HostagesTrackBTexture,
//                 ))
//                 .id();

//             // Spawn the trolley texture
//             let trolley_entity = commands
//                 .spawn((
//                     SpriteBundle {
//                         texture: trolley_front_texture.clone(),
//                         transform: horizon_distance_transform(
//                             APPROACHING_TROLLEY_HORIZON_POINT,
//                             APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
//                             duration,
//                         ),
//                         ..default()
//                     },
//                     TrolleyTexture,
//                 ))
//                 .id();

//             // Spawn the scenario text
//             let scenario_text_entity = commands
//                 .spawn(NodeBundle {
//                     style: Style {
//                         padding: UiRect::all(Val::Px(24.0)),
//                         position_type: PositionType::Absolute,
//                         bottom: Val::Px(0.0),
//                         left: Val::Px(0.0),
//                         width: Val::Vw(100.0),
//                         ..default()
//                     },
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(
//                         TextBundle::from_section(
//                             text.clone(),
//                             TextStyle {
//                                 font_size: 24.0,
//                                 color: Color::BLACK,
//                                 ..default()
//                             },
//                         )
//                         .with_text_justify(JustifyText::Center),
//                     );
//                 })
//                 .id();

//             // Spawn the timer text
//             let timer_text_entity = commands
//                 .spawn(NodeBundle {
//                     style: Style {
//                         padding: UiRect::all(Val::Px(8.0)),
//                         position_type: PositionType::Absolute,
//                         top: Val::Px(0.0),
//                         right: Val::Px(0.0),
//                         ..default()
//                     },
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn((
//                         TextBundle::from_section(
//                             format_timer_text(duration),
//                             TextStyle {
//                                 font_size: 24.0,
//                                 color: Color::BLACK,
//                                 ..default()
//                             },
//                         )
//                         .with_text_justify(JustifyText::Right),
//                         TimerText,
//                     ));
//                 })
//                 .id();

//             // Insert a resource containing all entities spawned so we can
//             // remove them later
//             commands.insert_resource(ScenarioEntitiesRes(vec![
//                 track_entity,
//                 lever_player_entity,
//                 hostages_track_a_entity,
//                 hostages_track_b_entity,
//                 trolley_entity,
//                 scenario_text_entity,
//                 timer_text_entity,
//             ]));

//             // Insert the timer resource
//             commands.insert_resource(ScenarioTimer(Timer::from_seconds(
//                 duration_secs,
//                 TimerMode::Once,
//             )));
//         }
//     }

//     /// Returns a function that, when called, will update the scenario every
//     /// game tick.
//     pub fn update_fn(
//         &self,
//     ) -> impl FnMut(
//         Res<Time>,
//         ResMut<ScenarioTimer>,
//         Query<&mut Text, With<TimerText>>,
//         Query<&mut Transform, With<TrolleyTexture>>,
//         Query<&mut Handle<Image>, With<TrolleyTexture>>,
//         Res<TrolleyTurnRes>,
//         Res<TrolleySideRes>,
//     ) {
//         move |time: Res<Time>,
//               mut timer: ResMut<ScenarioTimer>,
//               mut timer_text: Query<&mut Text, With<TimerText>>,
//               mut trolley_transform: Query<&mut Transform, With<TrolleyTexture>>,
//               mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
//               trolley_turn_texture: Res<TrolleyTurnRes>,
//               trolley_side_texture: Res<TrolleySideRes>| {
//             let previous_time_remaining = timer.remaining_secs();

//             // Advance the state of the timer, checking if time just ran out
//             if timer.tick(time.delta()).just_finished() {
//                 // TODO: determine which way the trolley goes here
//                 println!("time's up!");
//             }

//             let current_time_remaining = timer.remaining_secs();

//             // Update the timer text
//             timer_text.single_mut().sections[0].value =
//                 format_timer_text(timer.remaining().max(Duration::from_secs(0)));

//             // Trigger the trolley to turn slightly
//             if time_remaining_reached(previous_time_remaining, current_time_remaining, 3.0) {
//                 *trolley_texture.single_mut() = trolley_turn_texture.clone();
//             }

//             // Trigger the trolley to turn sideways
//             if time_remaining_reached(previous_time_remaining, current_time_remaining, 2.0) {
//                 *trolley_texture.single_mut() = trolley_side_texture.clone();
//             }

//             // Update the trolley transform
//             if timer.remaining_secs() > 3.0 {
//                 let new_transform = horizon_distance_transform(
//                     APPROACHING_TROLLEY_HORIZON_POINT,
//                     APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
//                     timer.remaining() - Duration::from_secs(3),
//                 );
//                 *trolley_transform.single_mut() = new_transform;
//             } else if timer.remaining_secs() > 2.0 {
//                 let new_transform = movement_transform(
//                     APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
//                     APPROACHING_TROLLEY_TURNING_END_TRANSFORM,
//                     Duration::from_secs_f32(1.0),
//                     timer.remaining() - Duration::from_secs_f32(2.0),
//                 );
//                 *trolley_transform.single_mut() = new_transform;
//             } else if timer.remaining_secs() > 0.0 {
//                 let new_transform = movement_transform(
//                     APPROACHING_TROLLEY_TURNING_END_TRANSFORM,
//                     APPROACHING_TROLLEY_SIDE_END_TRANSFORM,
//                     Duration::from_secs_f32(2.0),
//                     timer.remaining(),
//                 );
//                 *trolley_transform.single_mut() = new_transform;
//             }
//         }
//     }

//     /// Returns a function that, when called, will handle scenario click events.
//     pub fn handle_click_fn(
//         &self,
//     ) -> impl FnMut(
//         Query<&Window, With<PrimaryWindow>>,
//         Res<State<LeverState>>,
//         ResMut<NextState<LeverState>>,
//         ParamSet<(
//             Query<&mut Handle<Image>, With<TrackTexture>>,
//             Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
//         )>,
//         Res<TracksNormal>,
//         Res<TracksSwitched>,
//         Res<LeverNormal>,
//         Res<LeverSwitched>,
//     ) {
//         move |windows: Query<&Window, With<PrimaryWindow>>,
//               lever_state: Res<State<LeverState>>,
//               mut next_lever_state: ResMut<NextState<LeverState>>,
//               mut texture_set: ParamSet<(
//             Query<&mut Handle<Image>, With<TrackTexture>>,
//             Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
//         )>,
//               tracks_normal_texture: Res<TracksNormal>,
//               tracks_switched_texture: Res<TracksSwitched>,
//               lever_player_normal_texture: Res<LeverNormal>,
//               lever_player_switched_texture: Res<LeverSwitched>| {
//             let lever_rect = Rect::new(346.0, 135.0, 410.0, 202.0);

//             if let Some(mouse_pos) = windows.single().cursor_position() {
//                 if lever_rect.contains(mouse_pos) {
//                     match lever_state.get() {
//                         LeverState::Normal => {
//                             next_lever_state.set(LeverState::Pulled);
//                             *texture_set.p0().single_mut() = tracks_switched_texture.clone();
//                             *texture_set.p1().single_mut() = lever_player_switched_texture.clone();
//                         }
//                         LeverState::Pulled => {
//                             next_lever_state.set(LeverState::Normal);
//                             *texture_set.p0().single_mut() = tracks_normal_texture.clone();
//                             *texture_set.p1().single_mut() = lever_player_normal_texture.clone();
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     /// Returns a function that, when called, will clean up the scenario.
//     pub fn cleanup_fn(&self) -> impl FnMut(Commands, Res<ScenarioEntitiesRes>) {
//         move |mut commands: Commands, entities: Res<ScenarioEntitiesRes>| {
//             // Despawn the entities
//             for entity in &**entities {
//                 let entity_commands = commands.entity(*entity);
//                 entity_commands.despawn_recursive();
//             }

//             // Remove the entities resource
//             commands.remove_resource::<ScenarioEntitiesRes>();

//             // Remove the scenario timer
//             commands.remove_resource::<ScenarioTimer>();
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////

/// Sets up a scenario.
pub fn scenario_setup(
    mut commands: Commands,
    // tracks_normal_texture: Res<TracksNormal>,
    // lever_player_normal_texture: Res<LeverNormal>,
    // hostages_track_a_normal_texture: Res<HostagesANormal>,
    // hostages_track_b_normal_texture: Res<HostagesBNormal>,
    scenarios_config: Res<ScenariosConfigRes>,
    playing_state: Res<State<PlayingState>>,
    image_assets: Res<ImageAssetMap>,
    trolley_front_texture: Res<TrolleyFrontRes>,
) {
    let scenario_index = playing_state.0.unwrap();
    let scenario = scenarios_config.get(scenario_index);
    let tracks_normal_texture = image_assets.get_by_name(&scenario.tracks_normal_texture);
    let lever_player_normal_texture = image_assets.get_by_name(&scenario.lever_normal_texture);
    let hostages_track_a_normal_texture =
        image_assets.get_by_name(&scenario.hostages_track_a_normal_texture);
    let hostages_track_b_normal_texture =
        image_assets.get_by_name(&scenario.hostages_track_b_normal_texture);
    let duration = Duration::from_secs_f32(scenario.duration);

    // Spawn the track texture
    let track_entity = commands
        .spawn((
            SpriteBundle {
                texture: tracks_normal_texture.clone(),
                ..default()
            },
            TrackTexture,
        ))
        .id();

    // Spawn the lever/player texture
    let lever_player_entity = commands
        .spawn((
            SpriteBundle {
                texture: lever_player_normal_texture.clone(),
                ..default()
            },
            LeverPlayerTexture,
        ))
        .id();

    // Spawn the texture for the hostages on track A
    let hostages_track_a_entity = commands
        .spawn((
            SpriteBundle {
                texture: hostages_track_a_normal_texture.clone(),
                transform: Transform::from_translation(normalize_translation_to_canvas(
                    scenario.hostages_track_a_pos,
                )),
                ..default()
            },
            HostagesTrackATexture,
        ))
        .id();

    // Spawn the texture for the hostages on track B
    let hostages_track_b_entity = commands
        .spawn((
            SpriteBundle {
                texture: hostages_track_b_normal_texture.clone(),
                transform: Transform::from_translation(normalize_translation_to_canvas(
                    scenario.hostages_track_b_pos,
                )),
                ..default()
            },
            HostagesTrackBTexture,
        ))
        .id();

    // Spawn the trolley texture
    let trolley_entity = commands
        .spawn((
            SpriteBundle {
                texture: trolley_front_texture.clone(),
                transform: horizon_distance_transform(
                    APPROACHING_TROLLEY_HORIZON_POINT,
                    APPROACHING_TROLLEY_HORIZON_END_TRANSFORM,
                    duration,
                ),
                ..default()
            },
            TrolleyTexture,
        ))
        .id();

    // Spawn the scenario text
    let scenario_text_entity = commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(24.0)),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Vw(100.0),
                ..default()
            },
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
        .id();

    // Spawn the timer text
    let timer_text_entity = commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(8.0)),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
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
        .id();

    // Insert a resource containing all entities spawned so we can
    // remove them later
    commands.insert_resource(ScenarioEntitiesRes(vec![
        track_entity,
        lever_player_entity,
        hostages_track_a_entity,
        hostages_track_b_entity,
        trolley_entity,
        scenario_text_entity,
        timer_text_entity,
    ]));

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
) {
    let previous_time_remaining = timer.remaining_secs();

    // Advance the state of the timer, checking if time just ran out
    if timer.tick(time.delta()).just_finished() {
        // TODO: determine which way the trolley goes here
        println!("time's up!");
    }

    let current_time_remaining = timer.remaining_secs();

    // Update the timer text
    timer_text.single_mut().sections[0].value =
        format_timer_text(timer.remaining().max(Duration::from_secs(0)));

    // Trigger the trolley to turn slightly
    if time_remaining_reached(previous_time_remaining, current_time_remaining, 3.0) {
        *trolley_texture.single_mut() = trolley_turn_texture.clone();
    }

    // Trigger the trolley to turn sideways
    if time_remaining_reached(previous_time_remaining, current_time_remaining, 2.0) {
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
    windows: Query<&Window, With<PrimaryWindow>>,
    lever_state: Res<State<LeverState>>,
    mut next_lever_state: ResMut<NextState<LeverState>>,
    mut texture_set: ParamSet<(
        Query<&mut Handle<Image>, With<TrackTexture>>,
        Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
    )>,
    // tracks_normal_texture: Res<TracksNormal>,
    // tracks_switched_texture: Res<TracksSwitched>,
    // lever_player_normal_texture: Res<LeverNormal>,
    // lever_player_switched_texture: Res<LeverSwitched>,
    scenarios_config: Res<ScenariosConfigRes>,
    playing_state: Res<State<PlayingState>>,
    image_assets: Res<ImageAssetMap>,
) {
    let scenario_index = playing_state.0.unwrap();
    let scenario = scenarios_config.get(scenario_index);
    let tracks_normal_texture = image_assets.get_by_name(&scenario.tracks_normal_texture);
    let tracks_switched_texture = image_assets.get_by_name(&scenario.tracks_switched_texture);
    let lever_player_normal_texture = image_assets.get_by_name(&scenario.lever_normal_texture);
    let lever_player_switched_texture = image_assets.get_by_name(&scenario.lever_switched_texture);

    let lever_rect = Rect::new(346.0, 135.0, 410.0, 202.0);

    if let Some(mouse_pos) = windows.single().cursor_position() {
        if lever_rect.contains(mouse_pos) {
            match lever_state.get() {
                LeverState::Normal => {
                    next_lever_state.set(LeverState::Pulled);
                    *texture_set.p0().single_mut() = tracks_switched_texture.clone();
                    *texture_set.p1().single_mut() = lever_player_switched_texture.clone();
                }
                LeverState::Pulled => {
                    next_lever_state.set(LeverState::Normal);
                    *texture_set.p0().single_mut() = tracks_normal_texture.clone();
                    *texture_set.p1().single_mut() = lever_player_normal_texture.clone();
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

// macro_rules! typed_builder_value_option {
//     ( $unknown_ident:ident, $known_ident:ident, $config_trait_ident:ident, $value_ty:ty ) => {
//         pub struct $unknown_ident;

//         pub struct $known_ident($value_ty);

//         pub trait $config_trait_ident {}

//         impl $config_trait_ident for $unknown_ident {}

//         impl $config_trait_ident for $known_ident {}
//     };
// }

// macro_rules! typed_builder_texture_option {
//     ( $unknown_ident:ident, $known_ident:ident, $config_trait_ident:ident ) => {
//         pub struct $unknown_ident;

//         pub struct $known_ident<T>(PhantomData<fn() -> T>)
//         where
//             T: ?Sized + Resource + Deref<Target = Handle<Image>>;

//         pub trait $config_trait_ident {}

//         impl $config_trait_ident for $unknown_ident {}

//         impl<T> $config_trait_ident for $known_ident<T> where
//             T: ?Sized + Resource + Deref<Target = Handle<Image>>
//         {
//         }
//     };
// }

// typed_builder_value_option!(TextUnknown, TextKnown, TextConfig, String);
// typed_builder_value_option!(DurationUnknown, DurationKnown, DurationConfig, f32);
// typed_builder_value_option!(
//     HostagesTrackAPosUnknown,
//     HostagesTrackAPosKnown,
//     HostagesTrackAPosConfig,
//     Vec2
// );
// typed_builder_value_option!(
//     HostagesTrackBPosUnknown,
//     HostagesTrackBPosKnown,
//     HostagesTrackBPosConfig,
//     Vec2
// );
// typed_builder_value_option!(
//     TracksNormalUnknown,
//     TracksNormalKnown,
//     TracksNormalConfig,
//     String
// );
// typed_builder_value_option!(
//     TracksSwitchedUnknown,
//     TracksSwitchedKnown,
//     TracksSwitchedConfig,
//     String
// );
// typed_builder_value_option!(
//     LeverNormalUnknown,
//     LeverNormalKnown,
//     LeverNormalConfig,
//     String
// );
// typed_builder_value_option!(
//     LeverSwitchedUnknown,
//     LeverSwitchedKnown,
//     LeverSwitchedConfig,
//     String
// );
// typed_builder_value_option!(
//     HostagesANormalUnknown,
//     HostagesANormalKnown,
//     HostagesANormalConfig,
//     String
// );
// typed_builder_value_option!(
//     HostagesBNormalUnknown,
//     HostagesBNormalKnown,
//     HostagesBNormalConfig,
//     String
// );

// pub struct ScenarioBuilder<
//     Text: TextConfig,
//     Duration: DurationConfig,
//     HostagesTrackAPos: HostagesTrackAPosConfig,
//     HostagesTrackBPos: HostagesTrackBPosConfig,
//     TracksNormal: TracksNormalConfig,
//     TracksSwitched: TracksSwitchedConfig,
//     LeverNormal: LeverNormalConfig,
//     LeverSwitched: LeverSwitchedConfig,
//     HostagesANormal: HostagesANormalConfig,
//     HostagesBNormal: HostagesBNormalConfig,
// > {
//     text: Text,
//     duration: Duration,
//     hostages_track_a_pos: HostagesTrackAPos,
//     hostages_track_b_pos: HostagesTrackBPos,
//     tracks_normal_texture: TracksNormal,
//     tracks_switched_texture: TracksSwitched,
//     lever_normal_texture: LeverNormal,
//     lever_switched_texture: LeverSwitched,
//     hostages_track_a_normal_texture: HostagesANormal,
//     hostages_track_b_normal_texture: HostagesBNormal,
// }

// impl
//     ScenarioBuilder<
//         TextUnknown,
//         DurationUnknown,
//         HostagesTrackAPosUnknown,
//         HostagesTrackBPosUnknown,
//         TracksNormalUnknown,
//         TracksSwitchedUnknown,
//         LeverNormalUnknown,
//         LeverSwitchedUnknown,
//         HostagesANormalUnknown,
//         HostagesBNormalUnknown,
//     >
// {
//     pub fn new() -> Self {
//         Self {
//             text: TextUnknown,
//             duration: DurationUnknown,
//             hostages_track_a_pos: HostagesTrackAPosUnknown,
//             hostages_track_b_pos: HostagesTrackBPosUnknown,
//             tracks_normal_texture: TracksNormalUnknown,
//             tracks_switched_texture: TracksSwitchedUnknown,
//             lever_normal_texture: LeverNormalUnknown,
//             lever_switched_texture: LeverSwitchedUnknown,
//             hostages_track_a_normal_texture: HostagesANormalUnknown,
//             hostages_track_b_normal_texture: HostagesBNormalUnknown,
//         }
//     }
// }

// impl<
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         TextUnknown,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_text(
//         self,
//         text: impl Into<String>,
//     ) -> ScenarioBuilder<
//         TextKnown,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     > {
//         ScenarioBuilder {
//             text: TextKnown(text.into()),
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         DurationUnknown,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_duration(
//         self,
//         duration: f32,
//     ) -> ScenarioBuilder<
//         Text,
//         DurationKnown,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     > {
//         ScenarioBuilder {
//             text: self.text,
//             duration: DurationKnown(duration),
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPosUnknown,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_hostages_a_pos(
//         self,
//         pos: Vec2,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPosKnown,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     > {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: HostagesTrackAPosKnown(pos),
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPosUnknown,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_hostages_b_pos(
//         self,
//         pos: Vec2,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPosKnown,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     > {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: HostagesTrackBPosKnown(pos),
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormalUnknown,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_tracks_normal_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormalKnown<T>,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitchedUnknown,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_tracks_switched_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitchedKnown<T>,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormalUnknown,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_lever_normal_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormalKnown<T>,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormal,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         HostagesANormal: HostagesANormalConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitchedUnknown,
//         HostagesANormal,
//         HostagesBNormal,
//     >
// {
//     pub fn with_lever_switched_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitchedKnown<T>,
//         HostagesANormal,
//         HostagesBNormal,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesBNormal: HostagesBNormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormalUnknown,
//         HostagesBNormal,
//     >
// {
//     pub fn with_hostages_a_normal_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormalKnown<T>,
//         HostagesBNormal,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         Text: TextConfig,
//         Duration: DurationConfig,
//         HostagesTrackAPos: HostagesTrackAPosConfig,
//         HostagesTrackBPos: HostagesTrackBPosConfig,
//         TracksNormal: TracksNormalConfig,
//         TracksSwitched: TracksSwitchedConfig,
//         LeverNormal: LeverNormalConfig,
//         LeverSwitched: LeverSwitchedConfig,
//         HostagesANormal: HostagesANormalConfig,
//     >
//     ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormalUnknown,
//     >
// {
//     pub fn with_hostages_b_normal_texture<T>(
//         self,
//     ) -> ScenarioBuilder<
//         Text,
//         Duration,
//         HostagesTrackAPos,
//         HostagesTrackBPos,
//         TracksNormal,
//         TracksSwitched,
//         LeverNormal,
//         LeverSwitched,
//         HostagesANormal,
//         HostagesBNormalKnown<T>,
//     >
//     where
//         T: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     {
//         ScenarioBuilder {
//             text: self.text,
//             duration: self.duration,
//             hostages_track_a_pos: self.hostages_track_a_pos,
//             hostages_track_b_pos: self.hostages_track_b_pos,
//             tracks_normal_texture: self.tracks_normal_texture,
//             tracks_switched_texture: self.tracks_switched_texture,
//             lever_normal_texture: self.lever_normal_texture,
//             lever_switched_texture: self.lever_switched_texture,
//             hostages_track_a_normal_texture: self.hostages_track_a_normal_texture,
//             hostages_track_b_normal_texture: self.hostages_track_b_normal_texture,
//         }
//     }
// }

// impl<
//         TracksNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//         TracksSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//         LeverNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//         LeverSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
//         HostagesANormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//         HostagesBNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
//     >
//     ScenarioBuilder<
//         TextKnown,
//         DurationKnown,
//         HostagesTrackAPosKnown,
//         HostagesTrackBPosKnown,
//         TracksNormalKnown<TracksNormal>,
//         TracksSwitchedKnown<TracksSwitched>,
//         LeverNormalKnown<LeverNormal>,
//         LeverSwitchedKnown<LeverSwitched>,
//         HostagesANormalKnown<HostagesANormal>,
//         HostagesBNormalKnown<HostagesBNormal>,
//     >
// {
//     pub fn build(self) -> ScenarioSystems {
//         ScenarioSystems {
//             text: self.text.0,
//             duration: self.duration.0,
//             hostages_track_a_pos: self.hostages_track_a_pos.0,
//             hostages_track_b_pos: self.hostages_track_b_pos.0,
//             setup: scenario_setup::<TracksNormal, LeverNormal, HostagesANormal, HostagesBNormal>
//                 .into_configs(),
//             update: scenario_update.into_configs(),
//             handle_click: scenario_handle_click::<
//                 TracksNormal,
//                 TracksSwitched,
//                 LeverNormal,
//                 LeverSwitched,
//             >
//                 .into_configs(),
//             cleanup: scenario_cleanup.into_configs(),
//         }
//     }
// }

#[derive(TypedBuilder)]
pub struct Scenario {
    #[builder(setter(into))]
    text: String,
    duration: f32,
    hostages_track_a_pos: Vec2,
    hostages_track_b_pos: Vec2,
    #[builder(setter(into))]
    tracks_normal_texture: String,
    #[builder(setter(into))]
    tracks_switched_texture: String,
    #[builder(setter(into))]
    lever_normal_texture: String,
    #[builder(setter(into))]
    lever_switched_texture: String,
    #[builder(setter(into))]
    hostages_track_a_normal_texture: String,
    #[builder(setter(into))]
    hostages_track_b_normal_texture: String,
}

pub struct ScenarioCollectionPlugin {
    scenarios: Mutex<Option<Vec<Scenario>>>,
}

impl ScenarioCollectionPlugin {
    pub fn builder() -> ScenarioCollectionPluginBuilder {
        ScenarioCollectionPluginBuilder::new()
    }
}

impl Plugin for ScenarioCollectionPlugin {
    fn build(&self, app: &mut App) {
        let maybe_scenarios = self.scenarios.lock().unwrap().take();

        if let Some(scenarios) = maybe_scenarios {
            for index in 0..scenarios.len() {
                app.add_systems(OnEnter(PlayingState(Some(index))), scenario_setup)
                    .add_systems(
                        Update,
                        (
                            scenario_update.run_if(in_state(PlayingState(Some(index)))),
                            scenario_handle_click.run_if(
                                in_state(PlayingState(Some(index)))
                                    .and_then(input_just_pressed(MouseButton::Left)),
                            ),
                        ),
                    )
                    .add_systems(OnExit(PlayingState(Some(index))), scenario_cleanup);
            }

            app.insert_resource(ScenariosConfigRes(
                scenarios
                    .into_iter()
                    .map(|scenario| ScenarioConfig {
                        text: scenario.text,
                        duration: scenario.duration,
                        hostages_track_a_pos: scenario.hostages_track_a_pos,
                        hostages_track_b_pos: scenario.hostages_track_b_pos,
                        tracks_normal_texture: scenario.tracks_normal_texture,
                        tracks_switched_texture: scenario.tracks_switched_texture,
                        lever_normal_texture: scenario.lever_normal_texture,
                        lever_switched_texture: scenario.lever_switched_texture,
                        hostages_track_a_normal_texture: scenario.hostages_track_a_normal_texture,
                        hostages_track_b_normal_texture: scenario.hostages_track_b_normal_texture,
                    })
                    .collect(),
            ));
        }
    }
}

pub struct ScenarioCollectionPluginBuilder {
    scenarios: Vec<Scenario>,
}

impl ScenarioCollectionPluginBuilder {
    pub fn new() -> Self {
        Self {
            scenarios: Vec::new(),
        }
    }

    pub fn scenario(mut self, scenario: Scenario) -> Self {
        self.scenarios.push(scenario);
        self
    }

    pub fn build(self) -> ScenarioCollectionPlugin {
        ScenarioCollectionPlugin {
            scenarios: Mutex::new(Some(self.scenarios)),
        }
    }
}
