//! Game states.

use bevy::prelude::*;

/// The state of the game.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// The game is initializing.
    #[default]
    Initializing,
    /// The player is in the menu screen.
    InMenu,
    /// The player is playing the game scenarios.
    Playing,
    /// The player is at the end screen.
    EndScreen,
}

/// The index of the active scenario.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Deref, DerefMut)]
pub struct ScenarioIndexState(pub Option<usize>);

/// The state of the lever.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum LeverState {
    /// The lever has not been pulled.
    #[default]
    Normal,
    /// The lever has been pulled.
    Pulled,
}

impl LeverState {
    /// Returns whether the lever state is pulled.
    pub const fn pulled(&self) -> bool {
        matches!(self, Self::Pulled)
    }
}

/// The state of a scheduled animation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AnimationState {
    /// Waiting to start the animation.
    #[default]
    Waiting,
    /// The animation is running.
    Running,
    /// The animation has completed.
    Complete,
}

/// The index of the active animation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Deref, DerefMut)]
pub struct AnimationIndexState(pub Option<usize>);

/// The index of the active animation node.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Deref, DerefMut)]
pub struct AnimationNodeIndexState(pub Option<usize>);
