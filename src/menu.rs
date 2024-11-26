//! The menu screen.

use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;

/// Sets up the menu screen.
pub fn setup_menu_screen(mut commands: Commands) {
    // Spawn the play button
    let button_entity = commands
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
        })
        .id();

    // Save the button
    commands.insert_resource(MenuEntityRes(button_entity));
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
