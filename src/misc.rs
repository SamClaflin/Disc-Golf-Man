use bevy::prelude::*;

#[derive(Resource)]
pub struct SoundMaterials {
    pub background_sound: Handle<AudioSource>,
    pub slurp_sound: Handle<AudioSource>,
    pub tj_death_sound: Handle<AudioSource>,
    pub ghost_death_sound: Handle<AudioSource>
}

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct BackgroundMusicTimer(pub Timer);

#[derive(Resource)]
pub struct FontMaterial {
    pub handle: Handle<Font>
}

#[derive(Component)]
pub struct StartMessage;

#[derive(Component)]
pub struct RestartMessage;

#[derive(Component)]
pub struct EndMessage;

#[derive(Resource)]
pub struct EndMessageText(pub String);

impl Default for EndMessageText {
    fn default() -> Self {
        Self(String::new())
    }
}
