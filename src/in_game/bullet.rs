pub(crate) mod bullet_source;
pub(crate) mod bullet_spawn_clock;
pub(crate) mod bullet_spawn_event_writer;
pub(crate) mod collision;
pub(crate) mod new_bullet;

use bevy::prelude::*;

use crate::in_game::delta::Delta;

use self::new_bullet::NewBullet;

pub(crate) trait Bullet: Component + Copy + Clone {
    fn damage() -> u32;

    fn run(
        bullet_query: Query<(&mut Self, &mut Transform)>,
        player_query: Query<&Transform, (With<Delta>, Without<Self>)>,
        time: Res<Time>,
    );

    fn spawn(
        commands: Commands,
        new_bullet_event: EventReader<NewBullet<Self>>,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    );

    fn despawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn force_despawn<B: Bullet>(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<B>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let max = Vec2 {
        x: window.width() / 2.,
        y: window.height() / 2.,
    };

    bullet_query.iter().for_each(|(entity, transform)| {
        let (x, y) = (transform.translation.x, transform.translation.y);

        if x < -max.x || max.x < x || y < -max.y || max.y < y {
            commands.entity(entity).despawn();
        }
    });
}
