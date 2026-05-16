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
    pub cherry: u64,
    pub strawberry: u64,
    pub orange: u64,
    pub apple: u64,
    pub melon: u64,
    pub flagship: u64,
    pub bell: u64,
    pub key: u64,
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
            cherry: 100,
            strawberry: 300,
            orange: 500,
            apple: 700,
            melon: 1000,
            flagship: 2000,
            bell: 3000,
            key: 5000,
        }
    }
}
