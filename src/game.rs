//! Top-level game logic.

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

/// Sets the playing state once [`GameState::Playing`] is entered.
fn set_playing_state(mut next_playing_state: ResMut<NextState<PlayingState>>) {
    next_playing_state.set(PlayingState(Some(0)));
}

/// Unsets the playing state once [`GameState::Playing`] is exited.
fn unset_playing_state(mut next_playing_state: ResMut<NextState<PlayingState>>) {
    next_playing_state.set(PlayingState(None));
}

/// Detects when a scenario change is requested.
fn detect_scenario_change(
    scenario_change: Res<State<ScenarioChangeState>>,
    mut next_scenario_change: ResMut<NextState<ScenarioChangeState>>,
    playing_state: Res<State<PlayingState>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
    mut next_lever_state: ResMut<NextState<LeverState>>,
) {
    if ***scenario_change {
        next_scenario_change.set(ScenarioChangeState(false));
        next_playing_state.set(PlayingState(Some(playing_state.unwrap() + 1)));
        next_lever_state.set(LeverState::Normal);
    }
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
            .insert_state(PlayingState(None))
            .insert_state(LeverState::Normal)
            .insert_state(ScenarioChangeState(false));

        // Add game setup and menu screen systems.
        // Because the first two are chained, the user cannot possibly proceed
        // before all assets are loaded, since the "Play" button won't be shown
        // until then.
        app.add_systems(Startup, (setup_game, setup_menu_screen).chain());
        app.add_systems(
            Update,
            update_menu_screen.run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(OnExit(GameState::InMenu), cleanup_menu_screen);

        // Add playing systems
        app.add_systems(OnEnter(GameState::Playing), set_playing_state);
        app.add_systems(
            Update,
            detect_scenario_change.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(OnExit(GameState::Playing), unset_playing_state);

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
                        .tracks_normal_texture("tracks-straight")
                        .tracks_switched_texture("tracks-pull")
                        .lever_normal_texture("lever-default")
                        .lever_switched_texture("lever-pull")
                        .hostages_track_a_normal_texture("hostage-5")
                        .hostages_track_b_normal_texture("hostage-1")
                        .build())
                .build(),
        );
    }
}
