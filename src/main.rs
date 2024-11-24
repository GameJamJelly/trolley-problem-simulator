use bevy::prelude::*;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands
        .spawn(
            // Create a TextBundle that has a Text with a single section.
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        )
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Hello, Bevy!",
                    TextStyle {
                        font_size: 100.0,
                        ..default()
                    },
                ) // Set the justification of the Text
                .with_text_justify(JustifyText::Center)
                // Set the style of the TextBundle itself.
                .with_style(Style { ..default() }),
                ColorText,
            ));
        });
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::hsl(seconds * 72.0, 1.0, 0.5);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some("#game-canvas".to_owned()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, text_color_system)
        .run();
}
