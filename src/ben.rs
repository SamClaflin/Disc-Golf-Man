use bevy::prelude::*;
use crate::enums::Direction;
use crate::constants;

#[derive(Component)]
pub struct Ben;

#[derive(Component)]
pub struct BenLives(pub u8);

#[derive(Component)]
pub struct BenAnimationTimer(pub Timer);

#[derive(Component)]
pub struct BenSpeed(pub f32);

#[derive(Component)]
pub struct BenDirection(pub Direction);

#[derive(Component)]
pub struct BenNextDirection(pub Option<Direction>);

#[derive(Resource)]
pub struct BenMaterials {
    pub ben_default: Handle<Image>,
    pub ben_up: Handle<Image>,
    pub ben_right: Handle<Image>,
    pub ben_down: Handle<Image>,
    pub ben_left: Handle<Image>,
}

impl Default for Ben {
    fn default() -> Self { Self }
}

impl Default for BenLives {
    fn default() -> Self { Self(3) }
}

impl Default for BenAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Default for BenSpeed {
    fn default() -> Self { Self(constants::BEN_SPEED_DEFAULT) }
}

impl Default for BenDirection {
    fn default() -> Self { Self(Direction::Right) }
}

impl Default for BenNextDirection {
    fn default() -> Self { Self(None) }
}
