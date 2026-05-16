use bevy::prelude::*;

#[derive(Component)]
pub struct PowerUp;

#[derive(Component)]
pub struct PowerUpAnimationTimer(pub Timer);

#[derive(Resource)]
pub struct PowerUpMaterials {
    pub material_1: Handle<Image>,
    pub material_2: Handle<Image>
}
