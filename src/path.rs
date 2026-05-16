use std::collections::{HashMap, HashSet, VecDeque};
use bevy::prelude::*;
use crate::board::{Board, BoardTile};

pub struct Path(VecDeque<(f32, f32)>);

impl Path {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    fn ghost_walkable(tile: BoardTile) -> bool {
        tile != BoardTile::Wall
    }

    fn snap(v: f32, offset: f32, cell: f32) -> f32 {
        ((v - offset) / cell).round() * cell + offset
    }

    fn neighbors(i: usize, j: usize, w: usize, h: usize) -> [Option<(usize, usize)>; 4] {
        [
            if i > 0 { Some((i - 1, j)) } else { None },
            if i + 1 < h { Some((i + 1, j)) } else { None },
            Some((i, if j == 0 { w - 1 } else { j - 1 })),
            Some((i, if j + 1 >= w { 0 } else { j + 1 })),
        ]
    }

    // BFS outward from (i, j) to find the nearest ghost-walkable tile.
    fn nearest_walkable(i: usize, j: usize, board: &Board) -> (usize, usize) {
        if board.try_get(i, j).map_or(false, Self::ghost_walkable) {
            return (i, j);
        }
        let (w, h) = (board.width(), board.height());
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((i, j));
        visited.insert((i, j));
        while let Some((ci, cj)) = queue.pop_front() {
            for (ni, nj) in Self::neighbors(ci, cj, w, h).into_iter().flatten() {
                if visited.insert((ni, nj)) {
                    if board.try_get(ni, nj).map_or(false, Self::ghost_walkable) {
                        return (ni, nj);
                    }
                    queue.push_back((ni, nj));
                }
            }
        }
        (i, j)
    }

    fn bfs(
        start_i: usize,
        start_j: usize,
        end_i: usize,
        end_j: usize,
        board: &Board,
    ) -> Vec<(usize, usize)> {
        if start_i == end_i && start_j == end_j {
            return vec![];
        }
        let (w, h) = (board.width(), board.height());
        let mut queue = VecDeque::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        queue.push_back((start_i, start_j));
        came_from.insert((start_i, start_j), (start_i, start_j));
        'outer: while let Some((i, j)) = queue.pop_front() {
            for (ni, nj) in Self::neighbors(i, j, w, h).into_iter().flatten() {
                if !came_from.contains_key(&(ni, nj))
                    && board.try_get(ni, nj).map_or(false, Self::ghost_walkable)
                {
                    came_from.insert((ni, nj), (i, j));
                    if ni == end_i && nj == end_j {
                        break 'outer;
                    }
                    queue.push_back((ni, nj));
                }
            }
        }
        if !came_from.contains_key(&(end_i, end_j)) {
            return vec![];
        }
        let mut path = Vec::new();
        let mut cur = (end_i, end_j);
        while cur != (start_i, start_j) {
            path.push(cur);
            cur = came_from[&cur];
        }
        path.reverse();
        path
    }

    /// Build a pixel-level path from `start_transform` to the world coordinate `(target_x, target_y)`.
    ///
    /// If the target lands in a wall the nearest walkable tile is used instead.
    /// If the ghost isn't on a cell centre, an initial centering segment is prepended.
    pub fn to_coordinates(
        start_transform: &Transform,
        target_x: f32,
        target_y: f32,
        board: &Board,
        speed: f32,
    ) -> Self {
        let offset = board.offset();
        let cell = board.cell_size();

        let snap_x = Self::snap(start_transform.translation.x, offset, cell);
        let snap_y = Self::snap(start_transform.translation.y, offset, cell);
        let (start_i, start_j) = board.coordinates_to_indeces(snap_x, snap_y);

        let clamped_tx = target_x.clamp(offset, (board.width() as f32 - 1.0) * cell + offset);
        let clamped_ty = target_y.clamp(offset, (board.height() as f32 - 1.0) * cell + offset);
        let (raw_ti, raw_tj) = board.coordinates_to_indeces(clamped_tx, clamped_ty);
        let (end_i, end_j) = Self::nearest_walkable(raw_ti, raw_tj, board);

        let mut result = Self::new();

        // Prepend centering steps if the ghost isn't already on a cell centre.
        let act_x = start_transform.translation.x;
        let act_y = start_transform.translation.y;
        if (act_x - snap_x).abs() > 0.5 || (act_y - snap_y).abs() > 0.5 {
            let dist = (act_x - snap_x).abs() + (act_y - snap_y).abs();
            let steps = (dist / speed).round().max(1.0) as usize;
            let dx = (snap_x - act_x) / steps as f32;
            let dy = (snap_y - act_y) / steps as f32;
            for s in 1..=steps {
                result.push_back((act_x + dx * s as f32, act_y + dy * s as f32));
            }
        }

        if start_i == end_i && start_j == end_j {
            return result;
        }

        let cell_path = Self::bfs(start_i, start_j, end_i, end_j, board);
        if cell_path.is_empty() {
            return result;
        }

        let mut cur_x = snap_x;
        let mut cur_y = snap_y;

        for (ni, nj) in cell_path {
            let (dest_x, dest_y) = board.indeces_to_coordinates(ni, nj);
            let (cur_i, cur_j) = board.coordinates_to_indeces(cur_x, cur_y);
            // Detect horizontal tunnel wrap: same row, column gap > 1.
            let is_wrap = cur_i == ni && cur_j.abs_diff(nj) > 1;

            if is_wrap {
                result.push_back((dest_x, dest_y));
            } else {
                let steps = (cell / speed).round() as usize;
                let dx = (dest_x - cur_x) / steps as f32;
                let dy = (dest_y - cur_y) / steps as f32;
                for s in 1..=steps {
                    result.push_back((cur_x + dx * s as f32, cur_y + dy * s as f32));
                }
            }
            cur_x = dest_x;
            cur_y = dest_y;
        }

        result
    }

    pub fn shortest_to_ghost_spawn(initial_transform: &Transform, board: &Board, speed: f32) -> Self {
        let (target_x, target_y) = crate::utils::get_ghost_spawn_coordinates(board);
        Self::to_coordinates(initial_transform, target_x, target_y, board, speed)
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


}
