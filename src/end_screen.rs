//! The end screen.

use crate::constants::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;

/// Sets up the end screen.
pub fn setup_end_screen(mut commands: Commands, summary: Res<GameSummary>) {
    let mut summary_text_sections = Vec::new();

    summary_text_sections.push(format!("Killed {} people", summary.people_killed));
    summary_text_sections.push(format!("Saved {} people", summary.people_saved));

    if summary.killed_hitler {
        summary_text_sections.push("Killed Hitler".to_owned());
    }

    if summary.got_cool_hat {
        summary_text_sections.push("Got a cool hat".to_owned());
    }

    if summary.caused_preventable_tragedy {
        summary_text_sections.push("Caused an entirely preventable tragedy".to_owned());
    }

    if summary.enforced_darwinism {
        summary_text_sections.push("Enforced Darwinism".to_owned());
    }

    if summary.did_sick_loop {
        summary_text_sections.push("Did a sick loop-da-loop".to_owned());
    }

    if summary.lobsters_killed > 0 {
        summary_text_sections.push(format!("Killed {} lobsters", summary.lobsters_killed));
    }

    if summary.lobsters_saved > 0 {
        summary_text_sections.push(format!("Saved {} lobsters", summary.lobsters_saved));
    }

    if summary.returned_shopping_cart {
        summary_text_sections.push("Returned a shopping cart".to_owned());
    }

    if summary.doubled_it {
        summary_text_sections.push("Doubled it and gave it to the next person".to_owned());
    }

    if summary.watched_thomas_kill_people {
        summary_text_sections
            .push("Watched Thomas the Tank Engine run multiple people over".to_owned());
    }

    if summary.did_viral_prank {
        summary_text_sections.push("Contributed to a viral YouTube prank".to_owned());
    }

    if summary.killed_self {
        summary_text_sections.push("Killed yourself".to_owned());
    }

    if summary.solved_philosophy {
        summary_text_sections.push("Solved philosophy".to_owned());
    }

    summary_text_sections
        .iter_mut()
        .for_each(|line| *line = format!("{} {}", BULLET_POINT, line));

    // Spawn the back to menu button
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
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(75.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Congratulations! You:",
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 32.0,
                            ..default()
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for summary_line in summary_text_sections {
                                parent.spawn(TextBundle::from_section(
                                    summary_line,
                                    TextStyle {
                                        color: Color::BLACK,
                                        font_size: 24.0,
                                        ..default()
                                    },
                                ));
                            }
                        });

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
                                "Back to menu",
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::srgb(1.0, 1.0, 1.0),
                                    ..default()
                                },
                            ));
                        });
                });
        })
        .id();

    // Save the button
    commands.insert_resource(EndScreenEntityRes(button_entity));
}

/// Updates the end screen when the "Play" button is pressed.
pub fn update_end_screen(
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
                next_game_state.set(GameState::InMenu);
            }
        }
    }
}

/// Cleans up the end screen.
pub fn cleanup_end_screen(mut commands: Commands, entity: Res<EndScreenEntityRes>) {
    // Despawn the entity
    let entity_commands = commands.entity(**entity);
    entity_commands.despawn_recursive();

    // Remove the entity resource
    commands.remove_resource::<EndScreenEntityRes>();

    // Remove the game summary resource
    commands.remove_resource::<GameSummary>();
}
