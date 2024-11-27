//! Trolley problem scenario implementation.

use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use crate::util::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::marker::PhantomData;
use std::ops::Deref;
use std::time::Duration;

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

/// A trolley problem scenario.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scenario<
    TracksNormal,
    TracksSwitched,
    LeverNormal,
    LeverSwitched,
    HostagesANormal,
    HostagesBNormal,
> where
    TracksNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    TracksSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LeverNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LeverSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
    HostagesANormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    HostagesBNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
{
    /// The scenario text.
    pub text: &'static str,
    /// The duration of time, in seconds, which the player will have to decide.
    pub duration: f32,
    /// Pixel position (centered) of the hostages on track A.
    pub hostages_track_a_pos: Vec2,
    /// Pixel position (centered) of the hostages on track B.
    pub hostages_track_b_pos: Vec2,
    /// Marker to use the generics.
    pub marker: PhantomData<
        fn() -> (
            Box<TracksNormal>,
            Box<TracksSwitched>,
            Box<LeverNormal>,
            Box<LeverSwitched>,
            Box<HostagesANormal>,
            Box<HostagesBNormal>,
        ),
    >,
}

impl<
        TracksNormal,
        TracksSwitched,
        LeverNormal,
        LeverSwitched,
        HostagesANormal,
        HostagesBNormal,
    >
    Scenario<
        TracksNormal,
        TracksSwitched,
        LeverNormal,
        LeverSwitched,
        HostagesANormal,
        HostagesBNormal,
    >
where
    TracksNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    TracksSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LeverNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LeverSwitched: ?Sized + Resource + Deref<Target = Handle<Image>>,
    HostagesANormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
    HostagesBNormal: ?Sized + Resource + Deref<Target = Handle<Image>>,
{
    /// Returns a function that, when called, will set up the scenario.
    pub const fn setup_fn(
        &self,
    ) -> impl FnMut(
        Commands,
        Res<TracksNormal>,
        Res<LeverNormal>,
        Res<HostagesANormal>,
        Res<HostagesBNormal>,
        Res<TrolleyFrontRes>,
    ) + use<
        '_,
        TracksNormal,
        TracksSwitched,
        LeverNormal,
        LeverSwitched,
        HostagesANormal,
        HostagesBNormal,
    > {
        move |mut commands: Commands,
              tracks_normal_texture: Res<TracksNormal>,
              lever_player_normal_texture: Res<LeverNormal>,
              hostages_track_a_normal_texture: Res<HostagesANormal>,
              hostages_track_b_normal_texture: Res<HostagesBNormal>,
              trolley_front_texture: Res<TrolleyFrontRes>| {
            let duration = Duration::from_secs_f32(self.duration);

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
                            self.hostages_track_a_pos,
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
                            self.hostages_track_b_pos,
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
                            APPROACHING_TROLLEY_END_TRANSFORM,
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
                            self.text,
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
                self.duration,
                TimerMode::Once,
            )));
        }
    }

    /// Returns a function that, when called, will update the scenario every
    /// game tick.
    pub const fn update_fn(
        &self,
    ) -> impl FnMut(
        Res<Time>,
        ResMut<ScenarioTimer>,
        Query<&mut Text, With<TimerText>>,
        Query<&mut Transform, With<TrolleyTexture>>,
    ) {
        move |time: Res<Time>,
              mut timer: ResMut<ScenarioTimer>,
              mut timer_text: Query<&mut Text, With<TimerText>>,
              mut trolley_transform: Query<&mut Transform, With<TrolleyTexture>>| {
            // Advance the state of the timer, checking if time just ran out
            if timer.tick(time.delta()).just_finished() {
                // TODO: determine which way the trolley goes here
                println!("time's up!");
            }

            // Update the timer text
            timer_text.single_mut().sections[0].value =
                format_timer_text(timer.remaining().max(Duration::from_secs(0)));

            // Update the trolley transform
            if timer.remaining_secs() > 3.0 {
                let new_transform = horizon_distance_transform(
                    APPROACHING_TROLLEY_HORIZON_POINT,
                    APPROACHING_TROLLEY_END_TRANSFORM,
                    timer.remaining() - Duration::from_secs(3),
                );
                *trolley_transform.single_mut() = new_transform;
            }
        }
    }

    /// Returns a function that, when called, will handle scenario click events.
    pub const fn handle_click_fn(
        &self,
    ) -> impl FnMut(
        Query<&Window, With<PrimaryWindow>>,
        Res<State<LeverState>>,
        ResMut<NextState<LeverState>>,
        ParamSet<(
            Query<&mut Handle<Image>, With<TrackTexture>>,
            Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
        )>,
        Res<TracksNormal>,
        Res<TracksSwitched>,
        Res<LeverNormal>,
        Res<LeverSwitched>,
    ) {
        move |windows: Query<&Window, With<PrimaryWindow>>,
              lever_state: Res<State<LeverState>>,
              mut next_lever_state: ResMut<NextState<LeverState>>,
              mut texture_set: ParamSet<(
            Query<&mut Handle<Image>, With<TrackTexture>>,
            Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
        )>,
              tracks_normal_texture: Res<TracksNormal>,
              tracks_switched_texture: Res<TracksSwitched>,
              lever_player_normal_texture: Res<LeverNormal>,
              lever_player_switched_texture: Res<LeverSwitched>| {
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
    }

    /// Returns a function that, when called, will clean up the scenario.
    pub const fn cleanup_fn(&self) -> impl FnMut(Commands, Res<ScenarioEntitiesRes>) {
        move |mut commands: Commands, entities: Res<ScenarioEntitiesRes>| {
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
    }
}
