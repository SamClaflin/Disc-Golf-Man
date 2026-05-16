use bevy::prelude::*;
use crate::enums::Direction;

#[derive(Event)]
pub struct TJDirectionChangedEvent(pub Direction);

#[derive(Event)]
pub struct PowerUpConsumedEvent;
