use bevy::prelude::*;

pub(crate) mod homing;
pub(crate) mod player_straight;
pub(crate) mod reflect;
pub(crate) mod straight;

#[derive(Component, Clone, Copy)]
pub(crate) struct StraightBullet {
    pub(crate) speed: f32,
    pub(crate) angle: f32,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct PlayerStraightBullet {
    pub(crate) speed: f32,
    pub(crate) angle: f32,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct HomingBullet {
    pub(crate) speed: f32,
    pub(crate) angle: f32,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct ReflectBullet {
    pub(crate) speed: f32,
    pub(crate) direction: Vec2,
    pub(crate) reflection_remain: u32,
}
