use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config;
use crate::in_game::{
    self,
    bullet::{self, new_bullet::NewBullet},
};

use super::StraightBullet;

impl StraightBullet {
    pub(crate) fn new(speed: f32) -> Self {
        return Self { speed, angle: 0. };
    }
}

impl bullet::Bullet for StraightBullet {
    fn damage() -> u32 {
        let damage = 1;
        damage
    }

    fn run(
        mut bullet_query: Query<(&mut Self, &mut Transform)>,
        _player_query: Query<&Transform, (With<in_game::delta::Delta>, Without<StraightBullet>)>,
        time: Res<Time>,
    ) {
        bullet_query.iter_mut().for_each(|(bullet, mut transform)| {
            transform.translation +=
                bullet.speed * Vec2::from_angle(bullet.angle).extend(0.) * time.delta_seconds();
        })
    }

    fn spawn(
        mut commands: Commands,
        mut new_bullet_event: EventReader<NewBullet<StraightBullet>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        new_bullet_event.iter().for_each(|new_bullet| {
            let mut bullet = new_bullet.bullet;
            bullet.angle = new_bullet.angle;
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(config::bullets::Straight::SHAPE.into()).into(),
                    material: materials.add(ColorMaterial::from(config::bullets::Straight::COLOR)),
                    transform: Transform {
                        translation: new_bullet.translation.extend(0.),
                        scale: config::bullets::Straight::SIZE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(bullet)
                .insert(in_game::InGame);
        });
    }
}
