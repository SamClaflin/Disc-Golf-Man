mod board;
mod tj;
mod enums;
mod dot;
mod ghost;
mod utils;
mod score;
mod events;
mod power_up;
mod path;
mod constants;
mod misc;

use std::time::Duration;
use bevy::prelude::*;
use bevy::winit::{WinitSettings, UpdateMode};
use bevy::render::settings::{Backends, WgpuSettings, RenderCreation};
use bevy::render::RenderPlugin;
use ghost::{
    Ghost,
    GhostPath,
    AttackState,
    ReleaseState,
    GhostSpeed,
    Sean,
    SeanMaterials,
    Julie,
    JulieMaterials,
    Claflin,
    ClaflinMaterials,
    Sakshi,
    SakshiMaterials,
    GhostScareTimer,
    GhostReleaseTimer,
    GhostChain
};
use board::Board;
use tj::{TJ, TJAnimationTimer, TJSpeed, TJDirection, TJNextDirection, TJMaterials};
use enums::{Direction, GameState, CollisionType};
use dot::{Dot, DotMaterial};
use score::{Score, PointValues};
use events::{TJDirectionChangedEvent, PowerUpConsumedEvent};
use power_up::{PowerUp, PowerUpMaterials, PowerUpAnimationTimer};
use path::Path;

fn main() {
    let board = Board::new(constants::BOARD_CELL_SIZE, constants::BOARD_OFFSET);
    let scale = 0.8;
    let window_width = (board.width() as f32 * board.cell_size()) * scale;
    let window_height = (board.height() as f32 * board.cell_size() + 32.) * scale;

    App::new()
        // Plugins
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Disc-Golf-Man".to_string(),
                    resolution: (window_width, window_height).into(),
                    resizable: false,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
        )

        // Resources
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1. / constants::MAX_FRAMERATE)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1. / constants::MAX_FRAMERATE)),
        })
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_resource(board)
        .init_resource::<PointValues>()
        .init_resource::<GhostScareTimer>()
        .init_resource::<GhostReleaseTimer>()
        .init_resource::<GhostChain>()
        .init_resource::<EndMessageText>()

        // Events
        .add_event::<TJDirectionChangedEvent>()
        .add_event::<PowerUpConsumedEvent>()

        // State
        .init_state::<GameState>()

        // Startup
        .add_systems(Startup, setup)

        // Game start (Wait state)
        .add_systems(Update, wait_for_game_start.run_if(in_state(GameState::Wait)))

        // Mainloop (Default state)
        .add_systems(Update, (
            tj_controller_system,
            tj_movement_system.after(tj_controller_system),
            tj_dot_collision_system.after(tj_movement_system),
            tj_power_up_collision_system.after(tj_movement_system),
            tj_ghost_collision_system.after(tj_movement_system),
            tj_animation_system,
            scare_ghosts_system,
            ghost_movement_system,
            sean_animation_system,
            julie_animation_system,
            claflin_animation_system,
            sakshi_animation_system,
            win_system,
            ghost_release_system,
            ghost_respawn_system.after(tj_ghost_collision_system),
        ).run_if(in_state(GameState::Default)))

        // Game end
        .add_systems(Update, (
            display_end_message_system,
            wait_for_restart_system.after(display_end_message_system),
        ).run_if(in_state(GameState::End)))

        // Reset state
        .add_systems(OnEnter(GameState::Reset), (
            reset_score_system,
            reset_tj_system,
            reset_sean_system,
            reset_julie_system,
            reset_claflin_system,
            reset_sakshi_system,
            reset_dots_and_power_ups_system,
            reset_ghost_release_timer,
            reset_background_music,
            reset_end_message_text,
            restart_game_system,
        ).chain())

        // Miscellaneous (always running)
        .add_systems(Update, (
            power_up_animation_system,
            score_system,
            background_music_system,
        ))

        .run();
}

use misc::EndMessageText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    // Camera
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: board.width() as f32 * board.cell_size(),
                height: board.height() as f32 * board.cell_size() + 32.,
            },
            ..OrthographicProjection::default_2d()
        },
        Transform::from_translation(Vec3::new(
            board.width() as f32 * board.cell_size() / 2.,
            board.height() as f32 * board.cell_size() / 2.,
            1000.
        )),
    ));

    // Board
    commands.spawn((
        Sprite {
            image: asset_server.load("board.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            board.width() as f32 * board.cell_size() / 2.,
            board.height() as f32 * board.cell_size() / 2.,
            1.
        )),
    ));

    // Dots and power-ups
    let dot_material = DotMaterial {
        handle: asset_server.load("disc.png")
    };
    let power_up_materials = PowerUpMaterials {
        material_1: asset_server.load("basket.png"),
        material_2: asset_server.load("basket.png"),
    };
    utils::init_dots_and_power_ups(&mut commands, &board, dot_material.handle.clone(), power_up_materials.material_1.clone());
    commands.insert_resource(dot_material);
    commands.insert_resource(power_up_materials);

    // TJ
    let tj_materials = TJMaterials {
        tj_up: asset_server.load("tj/tj_up.png"),
        tj_right: asset_server.load("tj/tj_right.png"),
        tj_down: asset_server.load("tj/tj_down.png"),
        tj_left: asset_server.load("tj/tj_left.png"),
        tj_closed_up: asset_server.load("tj/tj_closed_up.png"),
        tj_closed_right: asset_server.load("tj/tj_closed_right.png"),
        tj_closed_down: asset_server.load("tj/tj_closed_down.png"),
        tj_closed_left: asset_server.load("tj/tj_closed_left.png"),
    };
    let (tj_init_x, tj_init_y) = utils::get_tj_spawn_coordinates(&board);
    commands.spawn((
        TJ,
        TJDirection::default(),
        TJAnimationTimer::default(),
        TJSpeed::default(),
        TJNextDirection::default(),
        Sprite {
            image: tj_materials.tj_closed_right.clone(),
            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
            ..default()
        },
        Transform::from_translation(Vec3::new(tj_init_x, tj_init_y, 10.)),
    ));
    commands.insert_resource(tj_materials);

    // Sean
    let sean_materials = SeanMaterials {
        default_material: asset_server.load("ghosts/sean.png"),
        scared_material: asset_server.load("ghosts/sean_scared.png"),
    };
    let (sean_init_x, sean_init_y) = utils::get_sean_spawn_coordinates(&board);
    commands.spawn((
        Ghost,
        Sean,
        AttackState::Attacking,
        ReleaseState::Released,
        GhostPath::default(),
        GhostSpeed::default(),
        Sprite {
            image: sean_materials.default_material.clone(),
            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
            ..default()
        },
        Transform::from_translation(Vec3::new(sean_init_x, sean_init_y, 9.)),
    ));
    commands.insert_resource(sean_materials);

    // Julie
    let julie_materials = JulieMaterials {
        default_material: asset_server.load("ghosts/julie.png"),
        scared_material: asset_server.load("ghosts/julie_scared.png"),
    };
    let (julie_init_x, julie_init_y) = utils::get_julie_spawn_coordinates(&board);
    commands.spawn((
        Ghost,
        Julie,
        AttackState::Attacking,
        ReleaseState::Caged,
        GhostPath::default(),
        GhostSpeed::default(),
        Sprite {
            image: julie_materials.default_material.clone(),
            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
            ..default()
        },
        Transform::from_translation(Vec3::new(julie_init_x, julie_init_y, 9.)),
    ));
    commands.insert_resource(julie_materials);

    // Claflin
    let claflin_materials = ClaflinMaterials {
        default_material: asset_server.load("ghosts/sam_c.png"),
        scared_material: asset_server.load("ghosts/sam_c_scared.png"),
    };
    let (claflin_init_x, claflin_init_y) = utils::get_claflin_spawn_coordinates(&board);
    commands.spawn((
        Ghost,
        Claflin,
        AttackState::Attacking,
        ReleaseState::Caged,
        GhostPath::default(),
        GhostSpeed::default(),
        Sprite {
            image: claflin_materials.default_material.clone(),
            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
            ..default()
        },
        Transform::from_translation(Vec3::new(claflin_init_x, claflin_init_y, 9.)),
    ));
    commands.insert_resource(claflin_materials);

    // Sakshi
    let sakshi_materials = SakshiMaterials {
        default_material: asset_server.load("ghosts/sakshi.png"),
        scared_material: asset_server.load("ghosts/sakshi_scared.png"),
    };
    let (sakshi_init_x, sakshi_init_y) = utils::get_sakshi_spawn_coordinates(&board);
    commands.spawn((
        Ghost,
        Sakshi,
        AttackState::Attacking,
        ReleaseState::Caged,
        GhostPath::default(),
        GhostSpeed::default(),
        Sprite {
            image: sakshi_materials.default_material.clone(),
            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
            ..default()
        },
        Transform::from_translation(Vec3::new(sakshi_init_x, sakshi_init_y, 9.)),
    ));
    commands.insert_resource(sakshi_materials);

    // Score
    let font_material = misc::FontMaterial {
        handle: asset_server.load("font.ttf")
    };
    commands.spawn((
        Score::default(),
        Text2d::new(""),
        TextFont {
            font: font_material.handle.clone(),
            font_size: 35.,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(
            board.width() as f32 * board.cell_size() / 2.,
            board.height() as f32 * board.cell_size(),
            100.
        )),
    ));
    commands.insert_resource(font_material);

    // Sounds
    commands.insert_resource(misc::SoundMaterials {
        background_sound: asset_server.load("sounds/battle.mp3"),
        slurp_sound: asset_server.load("sounds/rattling_chains.mp3"),
        tj_death_sound: asset_server.load("sounds/its_delicious.mp3"),
        ghost_death_sound: asset_server.load("sounds/evil_laugh.mp3")
    });

    // Background music timer
    commands.insert_resource(misc::BackgroundMusicTimer(Timer::from_seconds(87., TimerMode::Once)));
}

fn wait_for_game_start(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<misc::StartMessage>>,
    keys: Res<ButtonInput<KeyCode>>,
    board: Res<Board>,
    font_material: Res<misc::FontMaterial>
) {
    if query.is_empty() {
        commands.spawn((
            misc::StartMessage,
            Text2d::new("Press space to start"),
            TextFont {
                font: font_material.handle.clone(),
                font_size: 35.,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(
                board.width() as f32 * board.cell_size() / 2.,
                board.height() as f32 * board.cell_size() / 2. + 256.,
                100.
            )),
        ));
    } else {
        if keys.just_pressed(KeyCode::Space) {
            let start_message_entity = query.single();
            commands.entity(start_message_entity).despawn();
            next_state.set(GameState::Default);
        }
    }
}

fn tj_controller_system(
    mut query: Query<&mut TJNextDirection, With<TJ>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut tj_next_direction = query.single_mut();
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        tj_next_direction.0 = Some(Direction::Up);
    } else if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        tj_next_direction.0 = Some(Direction::Right);
    } else if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
        tj_next_direction.0 = Some(Direction::Down);
    } else if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        tj_next_direction.0 = Some(Direction::Left);
    }
}

fn tj_movement_system(
    mut query: Query<(&mut Transform, &mut TJNextDirection, &mut TJDirection, &TJSpeed), With<TJ>>,
    mut direction_changed_event: EventWriter<TJDirectionChangedEvent>,
    board: Res<Board>
) {
    let (mut transform, mut tj_next_direction, mut tj_direction, tj_speed) = query.single_mut();
    let speed = tj_speed.0;
    let can_move_up = utils::can_move_up(&transform, &board, speed);
    let can_move_right = utils::can_move_right(&transform, &board, speed);
    let can_move_down = utils::can_move_down(&transform, &board, speed);
    let can_move_left = utils::can_move_left(&transform, &board, speed);

    let next_direction = tj_next_direction.0;
    if next_direction.is_some() {
        let initial_direction = tj_direction.0;
        match next_direction.unwrap() {
            Direction::Up => {
                if can_move_up {
                    tj_direction.0 = Direction::Up;
                }
            },
            Direction::Right => {
                if can_move_right {
                    tj_direction.0 = Direction::Right;
                }
            },
            Direction::Down => {
                if can_move_down {
                    tj_direction.0 = Direction::Down;
                }
            },
            Direction::Left => {
                if can_move_left {
                    tj_direction.0 = Direction::Left;
                }
            },
        }

        if tj_direction.0 != initial_direction {
            direction_changed_event.send(TJDirectionChangedEvent(tj_direction.0));
            tj_next_direction.0 = None;
        }
    }

    let direction = tj_direction.0;
    let (target_x, target_y) = board.get_coordinates(transform.translation.x, transform.translation.y, direction, speed);
    if can_move_up && direction == Direction::Up
    || can_move_right && direction == Direction::Right
    || can_move_down && direction == Direction::Down
    || can_move_left && direction == Direction::Left {
        transform.translation.x = target_x;
        transform.translation.y = target_y;
    }
}

fn tj_animation_system(
    mut query: Query<(&mut Sprite, &mut TJAnimationTimer, &TJDirection), With<TJ>>,
    mut event_direction_changed: EventReader<TJDirectionChangedEvent>,
    time: Res<Time>,
    tj_materials: Res<TJMaterials>
) {
    let (mut sprite, mut tj_animation_timer, tj_direction) = query.single_mut();

    for event in event_direction_changed.read() {
        utils::update_tj_sprite(&mut sprite, event.0, &tj_materials);
        return;
    }

    let timer = &mut tj_animation_timer.0;
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    let closed_image = utils::get_tj_closed_image(tj_direction.0, &tj_materials);
    if sprite.image != closed_image {
        sprite.image = closed_image;
    } else {
        utils::update_tj_sprite(&mut sprite, tj_direction.0, &tj_materials);
    }
}

fn tj_dot_collision_system(
    mut commands: Commands,
    tj_query: Query<&Transform, With<TJ>>,
    dot_query: Query<(Entity, &Transform), With<Dot>>,
    mut score_query: Query<&mut Score>,
    board: Res<Board>,
    point_values: Res<PointValues>,
) {
    let tj_transform = tj_query.single();
    if utils::is_centered_horizontally(tj_transform, &board) && utils::is_centered_vertically(tj_transform, &board) {
        for (dot_entity, dot_transform) in dot_query.iter() {
            if dot_transform.translation.x == tj_transform.translation.x && dot_transform.translation.y == tj_transform.translation.y {
                commands.entity(dot_entity).despawn();
                score_query.single_mut().0 += point_values.dot;
                break;
            }
        }
    }
}

fn tj_power_up_collision_system(
    mut commands: Commands,
    tj_query: Query<&Transform, With<TJ>>,
    power_up_query: Query<(Entity, &Transform), With<PowerUp>>,
    mut score_query: Query<&mut Score>,
    mut power_up_consumed_event: EventWriter<PowerUpConsumedEvent>,
    mut ghost_chain: ResMut<GhostChain>,
    board: Res<Board>,
    point_values: Res<PointValues>,
    sound_materials: Res<misc::SoundMaterials>,
    mut commands_audio: Commands,
) {
    let tj_transform = tj_query.single();
    if utils::is_centered_horizontally(tj_transform, &board) && utils::is_centered_vertically(tj_transform, &board) {
        for (power_up_entity, power_up_transform) in power_up_query.iter() {
            if power_up_transform.translation.x == tj_transform.translation.x && power_up_transform.translation.y == tj_transform.translation.y {
                commands.entity(power_up_entity).despawn();
                score_query.single_mut().0 += point_values.power_up;
                ghost_chain.0 = 0;
                power_up_consumed_event.send(PowerUpConsumedEvent);
                commands_audio.spawn(AudioPlayer::new(sound_materials.slurp_sound.clone()));
                break;
            }
        }
    }
}

fn tj_ghost_collision_system(
    mut next_state: ResMut<NextState<GameState>>,
    tj_query: Query<&Transform, With<TJ>>,
    mut ghost_query: Query<(&Transform, &AttackState, &mut ReleaseState, &mut GhostPath), With<Ghost>>,
    mut score_query: Query<&mut Score>,
    music_query: Query<Entity, With<misc::BackgroundMusic>>,
    mut ghost_chain: ResMut<GhostChain>,
    mut end_message_text: ResMut<EndMessageText>,
    board: Res<Board>,
    point_values: Res<PointValues>,
    sound_materials: Res<misc::SoundMaterials>,
    mut commands: Commands,
) {
    let tj_transform = tj_query.single();
    let mut points = 0;
    for (ghost_transform, attack_state, mut release_state, mut ghost_path) in ghost_query.iter_mut() {
        if utils::did_collide(ghost_transform, tj_transform, &board, CollisionType::Approximate) {
            match attack_state {
                AttackState::Attacking => {
                    next_state.set(GameState::End);
                    end_message_text.0 = "Diagnosis: Skill Issue".to_string();
                    for entity in music_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    commands.spawn(AudioPlayer::new(sound_materials.tj_death_sound.clone()));
                },
                AttackState::Scared => {
                    if *release_state == ReleaseState::Respawning {
                        continue;
                    }

                    *release_state = ReleaseState::Respawning;
                    points += match ghost_chain.0 {
                        0 => point_values.first_ghost,
                        1 => point_values.second_ghost,
                        2 => point_values.third_ghost,
                        _ => point_values.fourth_ghost
                    };
                    ghost_chain.0 += 1;
                    ghost_path.0.clear();
                    commands.spawn(AudioPlayer::new(sound_materials.ghost_death_sound.clone()));
                }
            }
        }
    }

    score_query.single_mut().0 += points;
}

fn power_up_animation_system(
    mut query: Query<(&mut Sprite, &mut PowerUpAnimationTimer), With<PowerUp>>,
    power_up_materials: Res<PowerUpMaterials>,
    time: Res<Time>
) {
    for (mut sprite, mut power_up_animation_timer) in query.iter_mut() {
        let timer = &mut power_up_animation_timer.0;
        timer.tick(time.delta());
        if !timer.finished() {
            continue;
        }

        sprite.image = if sprite.image == power_up_materials.material_1 {
            power_up_materials.material_2.clone()
        } else {
            power_up_materials.material_1.clone()
        };
    }
}

fn scare_ghosts_system(
    mut query: Query<&mut AttackState, With<Ghost>>,
    mut power_up_consumed_event: EventReader<PowerUpConsumedEvent>,
    mut ghost_scare_timer: ResMut<GhostScareTimer>,
    time: Res<Time>
) {
    let timer = &mut ghost_scare_timer.0;

    for _ in power_up_consumed_event.read() {
        timer.reset();
        for mut attack_state in query.iter_mut() {
            if *attack_state == AttackState::Attacking {
                *attack_state = AttackState::Scared;
            }
        }
    }

    let mut scared = false;
    for attack_state in query.iter() {
        if *attack_state == AttackState::Scared {
            scared = true;
            break;
        }
    }

    if scared {
        timer.tick(time.delta());
        if timer.finished() {
            for mut attack_state in query.iter_mut() {
                if *attack_state == AttackState::Scared {
                    *attack_state = AttackState::Attacking;
                }
            }
            timer.reset();
        }
    }
}

fn ghost_movement_system(
    mut ghost_query: Query<(&mut Transform, &mut GhostPath, &GhostSpeed, &ReleaseState), With<Ghost>>,
    tj_query: Query<&Transform, (With<TJ>, Without<Ghost>)>,
    board: Res<Board>,
) {
    let tj_transform = tj_query.single();
    for (mut ghost_transform, mut ghost_path, ghost_speed, release_state) in ghost_query.iter_mut() {
        if *release_state != ReleaseState::Released {
            continue;
        }

        if let Some((x, y)) = ghost_path.0.pop_front() {
            ghost_transform.translation.x = x;
            ghost_transform.translation.y = y;
        } else {
            ghost_path.0 = Path::shortest_to_transform(
                &ghost_transform,
                tj_transform,
                &board,
                ghost_speed.0,
                CollisionType::Approximate
            );
        }
    }
}

fn sean_animation_system(
    mut query: Query<(&mut Sprite, &AttackState), With<Sean>>,
    sean_materials: Res<SeanMaterials>
) {
    let (mut sprite, attack_state) = query.single_mut();
    sprite.image = match attack_state {
        AttackState::Attacking => sean_materials.default_material.clone(),
        AttackState::Scared => sean_materials.scared_material.clone(),
    };
}

fn julie_animation_system(
    mut query: Query<(&mut Sprite, &AttackState), With<Julie>>,
    julie_materials: Res<JulieMaterials>
) {
    let (mut sprite, attack_state) = query.single_mut();
    sprite.image = match attack_state {
        AttackState::Attacking => julie_materials.default_material.clone(),
        AttackState::Scared => julie_materials.scared_material.clone(),
    };
}

fn claflin_animation_system(
    mut query: Query<(&mut Sprite, &AttackState), With<Claflin>>,
    claflin_materials: Res<ClaflinMaterials>
) {
    let (mut sprite, attack_state) = query.single_mut();
    sprite.image = match attack_state {
        AttackState::Attacking => claflin_materials.default_material.clone(),
        AttackState::Scared => claflin_materials.scared_material.clone(),
    };
}

fn sakshi_animation_system(
    mut query: Query<(&mut Sprite, &AttackState), With<Sakshi>>,
    sakshi_materials: Res<SakshiMaterials>
) {
    let (mut sprite, attack_state) = query.single_mut();
    sprite.image = match attack_state {
        AttackState::Attacking => sakshi_materials.default_material.clone(),
        AttackState::Scared => sakshi_materials.scared_material.clone(),
    };
}

fn score_system(
    mut query: Query<(&mut Text2d, &Score)>
) {
    let (mut text, score) = query.single_mut();
    **text = format!("Score: {}", score.0);
}

fn win_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut end_message_text: ResMut<EndMessageText>,
    query: Query<&Dot>,
) {
    if query.is_empty() {
        next_state.set(GameState::End);
        end_message_text.0 = "You Win! Now, go play\nsome real disc golf.".to_string();
    }
}

fn ghost_release_system(
    mut ghost_release_timer: ResMut<GhostReleaseTimer>,
    mut query: Query<(&mut ReleaseState, &mut Transform, &GhostSpeed), With<Ghost>>,
    board: Res<Board>,
    time: Res<Time>
) {
    let mut any_caged = false;
    let mut currently_releasing = false;
    for (release_state, _, _) in query.iter() {
        match *release_state {
            ReleaseState::Caged => any_caged = true,
            ReleaseState::Releasing => currently_releasing = true,
            _ => { }
        }
    }

    if !any_caged && !currently_releasing {
        return;
    }

    for (mut release_state, mut ghost_transform, ghost_speed) in query.iter_mut() {
        match *release_state {
            ReleaseState::Caged => {
                if currently_releasing {
                    continue;
                }

                let timer = &mut ghost_release_timer.0;
                timer.tick(time.delta());
                if !timer.finished() {
                    return;
                }

                *release_state = ReleaseState::Releasing;
                timer.reset();
                return;
            },
            ReleaseState::Releasing => {
                let x_target = board.width() as f32 * board.cell_size() / 2.;
                if ghost_transform.translation.x < x_target {
                    ghost_transform.translation.x += ghost_speed.0;
                    return;
                } else if ghost_transform.translation.x > x_target {
                    ghost_transform.translation.x -= ghost_speed.0;
                    return;
                }

                let (_, y_target) = board.indeces_to_coordinates(11, 0);
                if ghost_transform.translation.y < y_target {
                    ghost_transform.translation.y += ghost_speed.0;
                    return;
                } else {
                    *release_state = ReleaseState::Released;
                    return;
                }
            },
            _ => continue
        }
    }
}

fn ghost_respawn_system(
    mut query: Query<(&mut Transform, &mut ReleaseState, &mut GhostPath, &mut AttackState), With<Ghost>>,
    board: Res<Board>
) {
    for (mut transform, mut release_state, mut ghost_path, mut attack_state) in query.iter_mut() {
        if *release_state != ReleaseState::Respawning {
            continue;
        }

        if let Some((x, y)) = ghost_path.0.pop_front() {
            let (target_x, target_y) = utils::get_ghost_spawn_coordinates(&board);
            transform.translation.x = x;
            transform.translation.y = y;

            if transform.translation.x == target_x && transform.translation.y == target_y {
                *release_state = ReleaseState::Caged;
                *attack_state = AttackState::Attacking;
            }
        }
        else {
            if !utils::is_centered_horizontally(&transform, &board) {
                transform.translation.x -= transform.translation.x % board.cell_size();
            }
            if !utils::is_centered_vertically(&transform, &board) {
                transform.translation.y -= transform.translation.y % board.cell_size();
            }

            ghost_path.0 = Path::shortest_to_ghost_spawn(&transform, &board, constants::GHOST_SPEED_RESPAWNING);
        }
    }
}

fn background_music_system(
    mut background_music_timer: ResMut<misc::BackgroundMusicTimer>,
    sound_materials: Res<misc::SoundMaterials>,
    mut commands: Commands,
    time: Res<Time>
) {
    let timer = &mut background_music_timer.0;
    if timer.elapsed_secs() == 0. {
        commands.spawn((AudioPlayer::new(sound_materials.background_sound.clone()), misc::BackgroundMusic));
    }

    timer.tick(time.delta());
    if timer.finished() {
        timer.reset();
    }
}

fn reset_score_system(
    mut query: Query<&mut Score>
) {
    query.single_mut().0 = 0;
}

fn reset_tj_system(
    mut query: Query<(&mut Transform, &mut TJDirection, &mut Sprite), With<TJ>>,
    board: Res<Board>,
    tj_materials: Res<TJMaterials>
) {
    let (mut transform, mut tj_direction, mut sprite) = query.single_mut();

    let (x, y) = utils::get_tj_spawn_coordinates(&board);
    transform.translation.x = x;
    transform.translation.y = y;

    tj_direction.0 = constants::TJ_DIRECTION_DEFAULT;

    sprite.image = tj_materials.tj_closed_right.clone();
}

fn reset_sean_system(
    mut query: Query<(&mut Transform, &mut AttackState, &mut ReleaseState, &mut GhostPath, &mut Sprite), With<Sean>>,
    board: Res<Board>,
    sean_materials: Res<SeanMaterials>
) {
    let (mut transform, mut attack_state, mut release_state, mut ghost_path, mut sprite) = query.single_mut();

    let (x, y) = utils::get_sean_spawn_coordinates(&board);
    transform.translation.x = x;
    transform.translation.y = y;

    *attack_state = AttackState::Attacking;
    *release_state = ReleaseState::Released;

    ghost_path.0.clear();

    sprite.image = sean_materials.default_material.clone();
}

fn reset_julie_system(
    mut query: Query<(&mut Transform, &mut AttackState, &mut ReleaseState, &mut GhostPath, &mut Sprite), With<Julie>>,
    board: Res<Board>,
    julie_materials: Res<JulieMaterials>
) {
    let (mut transform, mut attack_state, mut release_state, mut ghost_path, mut sprite) = query.single_mut();

    let (x, y) = utils::get_julie_spawn_coordinates(&board);
    transform.translation.x = x;
    transform.translation.y = y;

    *attack_state = AttackState::Attacking;
    *release_state = ReleaseState::Caged;

    ghost_path.0.clear();

    sprite.image = julie_materials.default_material.clone();
}

fn reset_claflin_system(
    mut query: Query<(&mut Transform, &mut AttackState, &mut ReleaseState, &mut GhostPath, &mut Sprite), With<Claflin>>,
    board: Res<Board>,
    claflin_materials: Res<ClaflinMaterials>
) {
    let (mut transform, mut attack_state, mut release_state, mut ghost_path, mut sprite) = query.single_mut();

    let (x, y) = utils::get_claflin_spawn_coordinates(&board);
    transform.translation.x = x;
    transform.translation.y = y;

    *attack_state = AttackState::Attacking;
    *release_state = ReleaseState::Caged;

    ghost_path.0.clear();

    sprite.image = claflin_materials.default_material.clone();
}

fn reset_sakshi_system(
    mut query: Query<(&mut Transform, &mut AttackState, &mut ReleaseState, &mut GhostPath, &mut Sprite), With<Sakshi>>,
    board: Res<Board>,
    sakshi_materials: Res<SakshiMaterials>
) {
    let (mut transform, mut attack_state, mut release_state, mut ghost_path, mut sprite) = query.single_mut();

    let (x, y) = utils::get_sakshi_spawn_coordinates(&board);
    transform.translation.x = x;
    transform.translation.y = y;

    *attack_state = AttackState::Attacking;
    *release_state = ReleaseState::Caged;

    ghost_path.0.clear();

    sprite.image = sakshi_materials.default_material.clone();
}

fn reset_dots_and_power_ups_system(
    mut commands: Commands,
    dot_query: Query<Entity, With<Dot>>,
    power_up_query: Query<Entity, With<PowerUp>>,
    board: Res<Board>,
    dot_material: Res<DotMaterial>,
    power_up_materials: Res<PowerUpMaterials>
) {
    for dot_entity in dot_query.iter() {
        commands.entity(dot_entity).despawn();
    }

    for power_up_entity in power_up_query.iter() {
        commands.entity(power_up_entity).despawn();
    }

    utils::init_dots_and_power_ups(&mut commands, &board, dot_material.handle.clone(), power_up_materials.material_1.clone());
}

fn reset_ghost_release_timer(
    mut ghost_release_timer: ResMut<GhostReleaseTimer>
) {
    ghost_release_timer.0.reset();
}

fn reset_background_music(
    mut background_music_timer: ResMut<misc::BackgroundMusicTimer>,
) {
    background_music_timer.0 = Timer::from_seconds(87., TimerMode::Once);
}

fn reset_end_message_text(
    mut commands: Commands,
    query: Query<Entity, With<misc::EndMessage>>
) {
    if !query.is_empty() {
        let end_message_entity = query.single();
        commands.entity(end_message_entity).despawn();
    }
}

fn restart_game_system(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Default);
}

fn wait_for_restart_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<misc::RestartMessage>>,
    font_material: Res<misc::FontMaterial>,
    keys: Res<ButtonInput<KeyCode>>,
    board: Res<Board>
) {
    if query.is_empty() {
        commands.spawn((
            misc::RestartMessage,
            Text2d::new("Press space to restart"),
            TextFont {
                font: font_material.handle.clone(),
                font_size: 35.,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(
                board.width() as f32 * board.cell_size() / 2.,
                board.height() as f32 * board.cell_size() / 2. + 256.,
                100.
            )),
        ));
    } else {
        if keys.just_pressed(KeyCode::Space) {
            let restart_message_entity = query.single();
            commands.entity(restart_message_entity).despawn();
            next_state.set(GameState::Reset);
        }
    }
}

fn display_end_message_system(
    mut commands: Commands,
    query: Query<Entity, With<misc::EndMessage>>,
    end_message_text: Res<EndMessageText>,
    font_material: Res<misc::FontMaterial>,
    board: Res<Board>
) {
    if query.is_empty() {
        commands.spawn((
            misc::EndMessage,
            Text2d::new(end_message_text.0.as_str()),
            TextFont {
                font: font_material.handle.clone(),
                font_size: 35.,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(
                board.width() as f32 * board.cell_size() / 2.,
                board.height() as f32 * board.cell_size() / 2.,
                100.
            )),
        ));
    }
}

