use bevy::prelude::*;
use crate::ben::BenMaterials;
use crate::board::{Board, BoardTile};
use crate::enums::{CollisionType, Direction};
use crate::dot::Dot;
use crate::power_up::{PowerUp, PowerUpAnimationTimer};

pub fn is_centered_horizontally(transform: &Transform, board: &Board) -> bool {
    (transform.translation.x - board.offset()) % board.cell_size() == 0.
}

pub fn is_centered_vertically(transform: &Transform, board: &Board) -> bool {
    (transform.translation.y - board.offset()) % board.cell_size() == 0.
}

pub fn can_move_up(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_y = transform.translation.y + speed;
    let (i, j) = board.coordinates_to_indeces(transform.translation.x, new_y);
    let new_tile = board.try_get(i, j);
    is_centered_horizontally(transform, board)
        && new_tile.is_some()
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_right(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_x = transform.translation.x + speed;
    let (i, j) = board.coordinates_to_indeces(new_x - speed + board.cell_size(), transform.translation.y);
    let new_tile = board.try_get(i, j);
    is_centered_vertically(transform, board)
        && new_tile.is_some()
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_down(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_y = transform.translation.y - speed;
    let (i, j) = board.coordinates_to_indeces(transform.translation.x, new_y + speed - board.cell_size());
    let new_tile = board.try_get(i, j);
    is_centered_horizontally(transform, board)
        && new_tile.is_some()
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_left(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_x = transform.translation.x - speed;
    let (i, j) = board.coordinates_to_indeces(new_x, transform.translation.y);
    let new_tile = board.try_get(i, j);
    is_centered_vertically(transform, board)
        && new_tile.is_some()
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn did_collide(a_transform: &Transform, b_transform: &Transform, board: &Board, collision_type: CollisionType) -> bool {
    match collision_type {
        CollisionType::Approximate => {
            a_transform.translation.x == b_transform.translation.x && (a_transform.translation.y - b_transform.translation.y).abs() <= board.cell_size()
            || a_transform.translation.y == b_transform.translation.y && (a_transform.translation.x - b_transform.translation.x).abs() <= board.cell_size()
        },
        CollisionType::Exact => {
            a_transform.translation.x == b_transform.translation.x && a_transform.translation.y == b_transform.translation.y
        }
    }
}

pub fn get_ghost_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.width() as f32 * board.cell_size() / 2.;
    let (_, y) = board.indeces_to_coordinates(14, 0);
    (x, y)
}

pub fn get_ben_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.width() as f32 * board.cell_size() / 2.;
    let (_, y) = board.indeces_to_coordinates(23, 0);
    (x, y)
}

pub fn get_caleb_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.width() as f32 * board.cell_size() / 2.;
    let (_, y) = board.indeces_to_coordinates(11, 0);
    (x, y)
}

pub fn get_harris_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.cell_size() * board.width() as f32 / 2. - board.cell_size() * 2.;
    let (_, y) = board.indeces_to_coordinates(14, 0);
    (x, y)
}

pub fn get_claflin_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.width() as f32 * board.cell_size() / 2.;
    let (_, y) = board.indeces_to_coordinates(14, 0);
    (x, y)
}

pub fn get_samson_spawn_coordinates(board: &Board) -> (f32, f32) {
    let x = board.cell_size() * board.width() as f32 / 2. + board.cell_size() * 2.;
    let (_, y) = board.indeces_to_coordinates(14, 0);
    (x, y)
}

pub fn init_dots_and_power_ups(
    commands: &mut Commands,
    board: &Board,
    dot_material: Handle<Image>,
    power_up_material: Handle<Image>
) {
    for i in 0..board.height() {
        for j in 0..board.width() {
            let (x, y) = board.indeces_to_coordinates(i, j);
            match board.try_get(i, j).unwrap() {
                BoardTile::Dot => {
                    commands.spawn((
                        Dot,
                        Sprite {
                            image: dot_material.clone(),
                            custom_size: Some(Vec2::new(board.cell_size() / 2., board.cell_size() / 2.)),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(x, y, 2.)),
                    ));
                },
                BoardTile::PowerUp => {
                    commands.spawn((
                        PowerUp,
                        PowerUpAnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
                        Sprite {
                            image: power_up_material.clone(),
                            custom_size: Some(Vec2::new(board.cell_size(), board.cell_size())),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(x, y, 2.)),
                    ));
                },
                _ => continue
            }
        }
    }
}

pub fn update_ben_sprite(
    sprite: &mut Sprite,
    direction: Direction,
    ben_materials: &BenMaterials
) {
    sprite.image = match direction {
        Direction::Up => ben_materials.ben_up.clone(),
        Direction::Right => ben_materials.ben_right.clone(),
        Direction::Down => ben_materials.ben_down.clone(),
        Direction::Left => ben_materials.ben_left.clone(),
    };
}
