//! Top-level game logic.

use crate::assets::*;
use crate::constants::*;
use crate::end_screen::*;
use crate::menu::*;
use crate::resources::*;
use crate::scenario::*;
use crate::states::*;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::marker::PhantomData;

/// Loads game assets and stores them as resources.
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // // Store assets as resources
    // commands.insert_resource(TracksNormalRes(load_embedded_image(
    //     &asset_server,
    //     "tracks-straight",
    // )));
    // commands.insert_resource(TracksSwitchedRes(load_embedded_image(
    //     &asset_server,
    //     "tracks-pull",
    // )));
    // commands.insert_resource(LeverPlayerNormalRes(load_embedded_image(
    //     &asset_server,
    //     "lever-default",
    // )));
    // commands.insert_resource(LeverPlayerSwitchedRes(load_embedded_image(
    //     &asset_server,
    //     "lever-pull",
    // )));
    // commands.insert_resource(Hostage1Res(load_embedded_image(&asset_server, "hostage-1")));
    // commands.insert_resource(Hostage5Res(load_embedded_image(&asset_server, "hostage-5")));
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

/// Immediately sets the game state to [`GameState::EndScreen`].
fn goto_end(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::EndScreen);
}

/// Adds scenarios to the app.
macro_rules! add_scenarios {
    ( @indexed $index:expr, $app:expr, ) => {
        {
            $app.add_systems(OnEnter(PlayingState(Some($index))), goto_end);
        }
    };

    ( @indexed $index:expr, $app:expr, $first:expr, $( $rest:expr, )* ) => {
        {
            $app.add_systems(OnEnter(PlayingState(Some($index))), $first.setup_fn())
                .add_systems(
                    Update,
                    (
                        $first.update_fn()
                            .run_if(in_state(PlayingState(Some($index)))),
                        $first.handle_click_fn()
                            .run_if(in_state(PlayingState(Some($index)))
                            .and_then(input_just_pressed(MouseButton::Left))),
                    )
                )
                .add_systems(OnExit(PlayingState(Some($index))), $first.cleanup_fn());
        }

        add_scenarios!( @indexed $index + 1usize, $app, $( $rest, )* )
    };

    ( $app:expr, $( $scenario:expr ),+ $(,)? ) => {
        add_scenarios!( @indexed 0usize, $app, $( $scenario, )+ )
    };
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

        // // Add scenario systems
        // add_scenarios!(app, SCENARIO1);

        // Add end screen systems
        app.add_systems(OnEnter(GameState::EndScreen), setup_end_screen);
        app.add_systems(
            Update,
            update_end_screen.run_if(in_state(GameState::EndScreen)),
        );
        app.add_systems(OnExit(GameState::EndScreen), cleanup_end_screen);

        // let scenario1: Scenario<TracksNormalRes, TracksSwitchedRes, LeverPlayerNormalRes, LeverPlayerSwitchedRes, Hostage5Res, Hostage1Res> = Scenario {
        //     text: "A trolley is headed towards a group of five people. You can intervene and pull the lever to switch the tracks so that only one person will be killed. Do you pull the lever?".to_owned(),
        //     duration: 20.0,
        //     hostages_track_a_pos: Vec2::new(530.0, 325.0),
        //     hostages_track_b_pos: Vec2::new(550.0, 230.0),
        //     marker: PhantomData,
        // };

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
