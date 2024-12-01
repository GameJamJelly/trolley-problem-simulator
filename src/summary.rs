//! Game summary systems.

use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;

/// Updates the game summary at the end of the original scenario.
pub fn update_summary_original(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.people_saved += 5;
    } else {
        summary.people_killed += 5;
        summary.people_saved += 1;
    }
}

/// Updates the game summary at the end of the age scenario.
pub fn update_summary_age(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.people_saved += 10;
    } else {
        summary.people_killed += 10;
        summary.people_saved += 1;
    }
}

/// Updates the game summary at the end of the clone scenario.
pub fn update_summary_clone(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.killed_hitler = true;
    } else {
        summary.people_saved += 1;
    }
}

/// Updates the game summary at the end of the cliff scenario.
pub fn update_summary_cliff(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_saved += 6;
    } else {
        summary.people_killed += 6;
        summary.killed_hitler = true;
    }
}

/// Updates the game summary at the end of the cool hat scenario.
pub fn update_summary_cool_hat(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.people_saved += 5;
    } else {
        summary.people_killed += 5;
        summary.people_saved += 1;
        summary.got_cool_hat = true;
    }
}

/// Updates the game summary at the end of the victim scenario.
pub fn update_summary_victim(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_saved += 1;
    } else {
        summary.people_killed += 1;
        summary.caused_preventable_tragedy = true;
    }
}

/// Updates the game summary at the end of the Darwinism scenario.
pub fn update_summary_darwinism(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 5;
        summary.people_saved += 1;
        summary.enforced_darwinism = true;
    } else {
        summary.people_killed += 1;
        summary.people_saved += 5;
    }
}

/// Updates the game summary at the end of the loop scenario.
pub fn update_summary_loop(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.people_saved += 5;
    } else {
        summary.people_killed += 5;
        summary.people_saved += 1;
        summary.did_sick_loop = true;
    }
}

/// Updates the game summary at the end of the professors scenario.
pub fn update_summary_professors(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.people_saved += 5;
    } else {
        summary.people_killed += 5;
        summary.people_saved += 1;
    }
}

/// Updates the game summary at the end of the loan forgiveness scenario.
pub fn update_summary_loan_forgiveness(
    mut summary: ResMut<GameSummary>,
    lever: Res<State<LeverState>>,
) {
    if lever.pulled() {
        summary.people_saved += 5;
    } else {
        summary.people_killed += 5;
        summary.caused_preventable_tragedy = true;
    }
}

/// Updates the game summary at the end of the lobster scenario.
pub fn update_summary_lobster(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.lobsters_saved += 5;
    } else {
        summary.people_saved += 1;
        summary.lobsters_killed += 5;
    }
}

/// Updates the game summary at the end of the shopping cart scenario.
pub fn update_summary_shopping_cart(
    mut summary: ResMut<GameSummary>,
    lever: Res<State<LeverState>>,
) {
    if lever.pulled() {
        summary.returned_shopping_cart = true;
    }
}

/// Updates the game summary at the end of the born lever puller scenario.
pub fn update_summary_born_lever_puller(
    mut summary: ResMut<GameSummary>,
    lever: Res<State<LeverState>>,
) {
    if lever.pulled() {
        summary.people_killed += 1;
        summary.caused_preventable_tragedy = true;
    } else {
        summary.people_saved += 1;
    }
}

/// Updates the game summary at the end of the double it scenario.
pub fn update_summary_double_it(mut summary: ResMut<GameSummary>, lever: Res<State<LeverState>>) {
    if lever.pulled() {
        summary.people_killed += 1;
    } else {
        summary.people_saved += 1;
        summary.doubled_it = true;
    }
}

/// Updates the game summary at the end of the Thomas the tank engine scenario.
pub fn update_summary_thomas_the_tank_engine(mut summary: ResMut<GameSummary>) {
    summary.people_killed += 5;
    summary.watched_thomas_kill_people = true;
}

/// Updates the game summary at the end of the YouTube prank scenario.
pub fn update_summary_youtube_prank(
    mut summary: ResMut<GameSummary>,
    lever: Res<State<LeverState>>,
) {
    if lever.pulled() {
        summary.people_saved += 5;
        summary.did_viral_prank = true;
    } else {
        summary.people_killed += 5;
    }
}

/// Updates the game summary at the end of the self scenario.
pub fn update_summary_self(mut summary: ResMut<GameSummary>, jumping: Res<SelfJumping>) {
    if jumping.jumped() {
        summary.people_killed += 1;
        summary.killed_self = true;
    } else {
        summary.people_saved += 1;
    }
}
