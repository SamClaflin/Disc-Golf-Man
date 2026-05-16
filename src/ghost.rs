use bevy::prelude::*;
use crate::path::Path;
use crate::constants;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Component)]
pub enum AttackState {
    Attacking,
    Scared
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Component)]
pub enum ReleaseState {
    Caged,
    Releasing,
    Released,
    Respawning
}

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct GhostPath(pub Path);

#[derive(Component)]
pub struct GhostSpeed(pub f32);

#[derive(Resource)]
pub struct GhostScareTimer(pub Timer);

impl Default for GhostScareTimer {
    fn default() -> Self {
        GhostScareTimer(Timer::from_seconds(10., TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct GhostReleaseTimer(pub Timer);

impl Default for GhostReleaseTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5., TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct GhostChain(pub u8);

impl Default for GhostChain {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Ghost {
    fn default() -> Self { Self }
}

impl Default for GhostPath {
    fn default() -> Self { Self(Path::new()) }
}

impl Default for GhostSpeed {
    fn default() -> Self { Self(constants::GHOST_SPEED_DEFAULT) }
}

impl Default for AttackState {
    fn default() -> Self { Self::Attacking }
}

impl Default for ReleaseState {
    fn default() -> Self { Self::Caged }
}

#[derive(Component)]
pub struct Caleb;

#[derive(Resource)]
pub struct CalebMaterials {
    pub default_material: Handle<Image>,
    pub scared_material: Handle<Image>
}

#[derive(Component)]
pub struct Harris;

#[derive(Resource)]
pub struct HarrisMaterials {
    pub default_material: Handle<Image>,
    pub scared_material: Handle<Image>
}

#[derive(Component)]
pub struct Claflin;

#[derive(Resource)]
pub struct ClaflinMaterials {
    pub default_material: Handle<Image>,
    pub scared_material: Handle<Image>
}

#[derive(Component)]
pub struct Samson;

#[derive(Resource)]
pub struct SamsonMaterials {
    pub default_material: Handle<Image>,
    pub scared_material: Handle<Image>
}
