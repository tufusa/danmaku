use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::force_despawn,
        bullets::{PlayerStraightBullet, StraightBullet},
    },
};

impl Plugin for super::BulletDespawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                force_despawn::<StraightBullet>,
                force_despawn::<PlayerStraightBullet>,
            )
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}
