use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}