use std::collections::VecDeque;
use bevy::prelude::*;
use crate::board::Board;
use crate::enums::{Direction, CollisionType};
use crate::utils;

pub struct Path(VecDeque<(f32, f32)>);

impl Path {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn shortest_to_transform(initial_transform: &Transform, target_transform: &Transform, board: &Board, speed: f32, collision_type: CollisionType) -> Self {
        fn shortest_to_transform_helper(
            _path: &mut Path,
            _initial_transform: &mut Transform,
            _target_transform: &Transform,
            _board: &Board,
            _speed: f32,
            _collision_type: CollisionType
        ) {
            if utils::did_collide(_initial_transform, _target_transform, _board, _collision_type) {
                return;
            }

            let up_position = _board.get_coordinates(_initial_transform.translation.x, _initial_transform.translation.y, Direction::Up, _speed);
            let right_position = _board.get_coordinates(_initial_transform.translation.x, _initial_transform.translation.y, Direction::Right, _speed);
            let down_position = _board.get_coordinates(_initial_transform.translation.x, _initial_transform.translation.y, Direction::Down, _speed);
            let left_position = _board.get_coordinates(_initial_transform.translation.x, _initial_transform.translation.y, Direction::Left, _speed);

            let mut available_directions: Vec<Direction> = Vec::new();
            if utils::can_move_up(_initial_transform, _board, _speed) && !_path.0.contains(&up_position) {
                available_directions.push(Direction::Up);
            }
            if utils::can_move_right(_initial_transform, _board, _speed) && !_path.0.contains(&right_position) {
                available_directions.push(Direction::Right);
            }
            if utils::can_move_down(_initial_transform, _board, _speed) && !_path.0.contains(&down_position) {
                available_directions.push(Direction::Down);
            }
            if utils::can_move_left(_initial_transform, _board, _speed) && !_path.0.contains(&left_position) {
                available_directions.push(Direction::Left);
            }

            let should_move_up = _initial_transform.translation.y < _target_transform.translation.y;
            let should_move_right = _initial_transform.translation.x < _target_transform.translation.x;
            let should_move_down = _initial_transform.translation.y > _target_transform.translation.y;
            let should_move_left = _initial_transform.translation.x > _target_transform.translation.x;

            let mut move_forward = |direction: Direction| {
                match direction {
                    Direction::Up => _initial_transform.translation.y = up_position.1,
                    Direction::Right => _initial_transform.translation.x = right_position.0,
                    Direction::Down => _initial_transform.translation.y = down_position.1,
                    Direction::Left => _initial_transform.translation.x = left_position.0,
                }

                _path.push_back((_initial_transform.translation.x, _initial_transform.translation.y));
                shortest_to_transform_helper(
                    _path,
                    _initial_transform,
                    _target_transform,
                    _board,
                    _speed,
                    _collision_type
                );
            };

            if should_move_up && available_directions.contains(&Direction::Up) {
                move_forward(Direction::Up)
            } else if should_move_right && available_directions.contains(&Direction::Right) {
                move_forward(Direction::Right)
            } else if should_move_down && available_directions.contains(&Direction::Down) {
                move_forward(Direction::Down)
            } else if should_move_left && available_directions.contains(&Direction::Left) {
                move_forward(Direction::Left)
            } else {
                if let Some(direction) = available_directions.pop() {
                    move_forward(direction);
                }
            }
        }

        let mut path = Self::new();
        shortest_to_transform_helper(&mut path, &mut initial_transform.clone(), target_transform, board, speed, collision_type);
        path
    }

    pub fn shortest_to_ghost_spawn(initial_transform: &Transform, board: &Board, speed: f32) -> Self {
        let (target_x, target_y) = utils::get_ghost_spawn_coordinates(board);
        let (_, temp_y) = board.indeces_to_coordinates(11, 0);
        let mut path = Self::shortest_to_transform(
            initial_transform,
            &Transform {
                translation: Vec3::new(target_x, temp_y, initial_transform.translation.z),
                ..Default::default()
            },
            board,
            speed,
            CollisionType::Exact
        );

        while path.peek_back().unwrap().0 == target_x && path.peek_back().unwrap().1 > target_y {
            let (x, y) = *path.peek_back().unwrap();
            path.push_back((x, y - speed));
        }

        path
    }

    pub fn push_back(&mut self, position: (f32, f32)) {
        self.0.push_back(position);
    }

    pub fn pop_front(&mut self) -> Option<(f32, f32)> {
        self.0.pop_front()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn peek_back(&self) -> Option<&(f32, f32)> {
        self.0.back()
    }
}
