use bevy::prelude::*;

use crate::{app_state::AppState, in_game::delta};

impl Plugin for super::DeltaUpdateSystems {
    fn build(&self, app: &mut App) {
        app.add_systems((delta::control,).in_set(OnUpdate(AppState::InGame)));
    }
}
