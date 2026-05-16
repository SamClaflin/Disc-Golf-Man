use bevy::prelude::*;

#[derive(Resource)]
pub struct DotMaterial {
    pub handle: Handle<Image>
}

#[derive(Component)]
pub struct Dot;
