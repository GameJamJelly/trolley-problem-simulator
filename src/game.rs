//! Top-level game logic.

use crate::animation::*;
use crate::assets::*;
use crate::constants::*;
use crate::end_screen::*;
use crate::menu::*;
use crate::resources::*;
use crate::scenario::*;
use crate::states::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

/// Loads game assets and stores them as resources.
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Store assets as resources
    commands.insert_resource(TrolleyFrontRes(load_embedded_image(
        &asset_server,
        "trolley-front",
    )));
    commands.insert_resource(TrolleyTurnRes(load_embedded_image(
        &asset_server,
        "trolley-turn",
    )));
    commands.insert_resource(TrolleySideRes(load_embedded_image(
        &asset_server,
        "trolley-side",
    )));

    // UI camera
    commands.spawn(Camera2dBundle::default());
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
        app.insert_state(GameState::InMenu)
            .insert_state(ScenarioIndexState(None))
            .insert_state(LeverState::Normal);

        // Add game setup and menu screen systems.
        // Because the first two are chained, the user cannot possibly proceed
        // before all assets are loaded, since the "Play" button won't be shown
        // until then.
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

        // Add scenarios
        app.add_plugins(
            ScenarioCollectionPlugin::builder()
                .scenario(
                    Scenario::builder()
                        .text("A trolley is headed towards a group of five people. You can intervene and pull the lever to switch the tracks so that only one person will be killed. Do you pull the lever?")
                        .duration(20.0)
                        .hostages_track_a_pos(Vec2::new(530.0, 325.0))
                        .hostages_track_b_pos(Vec2::new(550.0, 230.0))
                        .tracks_normal_texture("tracks-default")
                        .tracks_switched_texture("tracks-switched")
                        .lever_normal_texture("lever-default")
                        .lever_switched_texture("lever-pull")
                        .hostages_track_a_normal_texture("hostage-5")
                        .hostages_track_b_normal_texture("hostage-1")
                        .animation(
                            Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                                .on_lever_state(LeverState::Normal)
                                .node(AnimationNode::new(6.0, Transform::from_translation(Vec3::new(900.0, 445.0, 0.0)))))
                        .animation(
                            Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                                .on_lever_state(LeverState::Pulled)
                                .node(AnimationNode::new(1.0, Transform::from_translation(Vec3::new(400.0, 190.0, 0.0))))
                                .node(AnimationNode::new(5.0, Transform::from_translation(Vec3::new(900.0, 260.0, 0.0)))))
                        .build())
                .build(),
        );
    }
}
