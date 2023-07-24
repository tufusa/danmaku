use bevy::prelude::*;

use crate::in_game::{enemy, InGame};

use super::Phase;

#[derive(Component)]
pub(crate) struct Phase1;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemys: Vec<Vec3> = vec![
        Vec3::new(-150., 350., 0.),
        Vec3::new(-60., 350., 0.),
        Vec3::new(60., 350., 0.),
        Vec3::new(150., 350., 0.),
    ];

    enemys.iter().for_each(|translation| {
        enemy::normal1::spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            *translation,
            (InGame, Phase1),
        );
    });
}

pub(crate) fn check_clear(enemy_query: Query<&Phase1>, mut next_state: ResMut<NextState<Phase>>) {
    if !enemy_query.is_empty() {
        return;
    }

    next_state.set(Phase::Phase2);
}

pub(crate) fn cleanup(mut commands: Commands, phase_entity_query: Query<Entity, With<Phase1>>) {
    phase_entity_query.for_each(|entity| {
        commands.entity(entity).despawn();
    })
}
