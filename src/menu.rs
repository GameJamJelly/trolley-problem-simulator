//! The menu screen.

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::prelude::*;

/// Sets up the menu screen.
pub fn setup_menu_screen(
    mut commands: Commands,
    music_assets: Res<MusicAssetMap>,
    music: Query<(), With<GameMusic>>,
) {
    // Spawn the menu screen text
    let text_entity = commands
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(75.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(32.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Trolley Problem Simulator",
                            TextStyle {
                                color: Color::BLACK,
                                font_size: 64.0,
                                ..default()
                            },
                        )
                        .with_text_justify(JustifyText::Center),
                    );

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(16.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::srgb(1.0, 1.0, 1.0),
                                    ..default()
                                },
                            ));
                        });
                });
        })
        .id();

    // Save the button
    commands.insert_resource(MenuEntityRes(text_entity));

    // Insert the game summary resource
    commands.insert_resource(GameSummary::new());

    // Spawn the game music
    if music.is_empty() {
        let game_music = music_assets.get_by_name("trolley-main");
        commands.spawn((
            AudioBundle {
                source: game_music,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::new(GAME_VOLUME),
                    ..default()
                },
            },
            GameMusic,
        ));
    }
}

/// Updates the menu screen when the "Play" button is pressed.
pub fn update_menu_screen(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Playing);
            }
        }
    }
}

/// Cleans up the menu screen.
pub fn cleanup_menu_screen(mut commands: Commands, entity: Res<MenuEntityRes>) {
    // Despawn the entity
    let entity_commands = commands.entity(**entity);
    entity_commands.despawn_recursive();

    // Remove the entity resource
    commands.remove_resource::<MenuEntityRes>();
}
