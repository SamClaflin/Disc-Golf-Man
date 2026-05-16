use bevy::prelude::*;

#[derive(Component)]
pub struct Score(pub u64);

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource)]
pub struct PointValues {
    pub dot: u64,
    pub power_up: u64,
    pub first_ghost: u64,
    pub second_ghost: u64,
    pub third_ghost: u64,
    pub fourth_ghost: u64,
}

impl Default for PointValues {
    fn default() -> Self {
        Self {
            dot: 10,
            power_up: 50,
            first_ghost: 200,
            second_ghost: 400,
            third_ghost: 800,
            fourth_ghost: 1600,
        }
    }
}
