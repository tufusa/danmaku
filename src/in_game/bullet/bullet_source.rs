use bevy::prelude::*;

use super::Bullet;

#[derive(Component)]
pub(crate) struct BulletSource<B: Bullet> {
    pub(crate) angle: f32,
    pub(crate) bullet: B,
}
