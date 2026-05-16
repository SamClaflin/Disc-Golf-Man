use bevy::prelude::*;
use crate::enums::Direction;

#[derive(Event)]
pub struct BenDirectionChangedEvent(pub Direction);

#[derive(Event)]
pub struct PowerUpConsumedEvent;
