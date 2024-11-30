#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_const_for_fn)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod animation;
mod assets;
mod components;
mod constants;
mod end_screen;
mod game;
mod menu;
mod resources;
mod scenario;
mod states;
mod util;

use crate::game::*;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
