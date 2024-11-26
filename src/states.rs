//! Game states.

use bevy::prelude::*;

/// The state of the game.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// The player is in the menu screen.
    #[default]
    InMenu,
    /// The player is playing the game scenarios.
    Playing,
    /// The player is at the end screen.
    EndScreen,
}

/// The state of the game being played.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Deref, DerefMut)]
pub struct PlayingState(pub Option<usize>);

/// The state of the lever.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum LeverState {
    /// The lever has not been pulled.
    #[default]
    Normal,
    /// The lever has been pulled.
    Pulled,
}

/// The state of a changing scenario.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Deref, DerefMut)]
pub struct ScenarioChangeState(pub bool);
