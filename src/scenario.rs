//! Trolley problem scenario implementation.

use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::marker::PhantomData;
use std::ops::Deref;

/// The track texture component.
#[derive(Component)]
pub struct TrackTexture;

/// The lever/player texture component.
#[derive(Component)]
pub struct LeverPlayerTexture;

/// A trolley problem scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scenario<TN, TS, LN, LS>
where
    TN: ?Sized + Resource + Deref<Target = Handle<Image>>,
    TS: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LN: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LS: ?Sized + Resource + Deref<Target = Handle<Image>>,
{
    /// The scenario text.
    pub text: &'static str,
    /// Marker to use the generics.
    pub marker: PhantomData<fn() -> (Box<TN>, Box<TS>, Box<LN>, Box<LS>)>,
}

impl<TN, TS, LN, LS> Scenario<TN, TS, LN, LS>
where
    TN: ?Sized + Resource + Deref<Target = Handle<Image>>,
    TS: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LN: ?Sized + Resource + Deref<Target = Handle<Image>>,
    LS: ?Sized + Resource + Deref<Target = Handle<Image>>,
{
    /// Returns a function that, when called, will set up the scenario.
    pub const fn setup_fn(
        &self,
    ) -> impl FnMut(Commands, Res<TN>, Res<LN>) + use<'_, TN, TS, LN, LS> {
        move |mut commands: Commands,
              tracks_normal_texture: Res<TN>,
              lever_player_normal_texture: Res<LN>| {
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

            // Spawn the scenario text
            let text_entity = commands
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(32.0)),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        left: Val::Px(0.0),
                        width: Val::Vw(100.0),
                        height: Val::Vh(25.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
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

            // Insert a resource containing all entities spawned so we can
            // remove them later
            commands.insert_resource(ScenarioEntitiesRes(vec![
                track_entity,
                lever_player_entity,
                text_entity,
            ]));
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
        Res<TN>,
        Res<TS>,
        Res<LN>,
        Res<LS>,
    ) {
        move |windows: Query<&Window, With<PrimaryWindow>>,
              lever_state: Res<State<LeverState>>,
              mut next_lever_state: ResMut<NextState<LeverState>>,
              mut texture_set: ParamSet<(
            Query<&mut Handle<Image>, With<TrackTexture>>,
            Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
        )>,
              tracks_normal_texture: Res<TN>,
              tracks_switched_texture: Res<TS>,
              lever_player_normal_texture: Res<LN>,
              lever_player_switched_texture: Res<LS>| {
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
        }
    }
}
