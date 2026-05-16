use bevy::prelude::*;
use crate::enums::Direction;
use crate::constants;

#[derive(Component)]
pub struct TJ;

#[derive(Component)]
pub struct TJAnimationTimer(pub Timer);

#[derive(Component)]
pub struct TJSpeed(pub f32);

#[derive(Component)]
pub struct TJDirection(pub Direction);

#[derive(Component)]
pub struct TJNextDirection(pub Option<Direction>);

#[derive(Resource)]
pub struct TJMaterials {
    pub tj_default: Handle<Image>,
    pub tj_up: Handle<Image>,
    pub tj_right: Handle<Image>,
    pub tj_down: Handle<Image>,
    pub tj_left: Handle<Image>,
}

impl Default for TJ {
    fn default() -> Self { Self }
}

impl Default for TJAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Default for TJSpeed {
    fn default() -> Self { Self(constants::TJ_SPEED_DEFAULT) }
}

impl Default for TJDirection {
    fn default() -> Self { Self(Direction::Right) }
}

impl Default for TJNextDirection {
    fn default() -> Self { Self(None) }
}
