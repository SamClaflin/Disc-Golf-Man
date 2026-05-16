use crate::enums::Direction;

pub const BOARD_CELL_SIZE: f32 = 32.;
pub const BOARD_OFFSET: f32 = 16.;
pub const GHOST_SPEED_DEFAULT: f32 = 2.;
pub const GHOST_SPEED_RESPAWNING: f32 = 16.;
pub const BEN_SPEED_DEFAULT: f32 = 4.;
pub const BEN_DIRECTION_DEFAULT: Direction = Direction::Right;
pub const MAX_FRAMERATE: f64 = 60.;
