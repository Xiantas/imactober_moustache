use bevy::prelude::*;

#[derive(Component)]
pub struct FurPiece {
    pub sheared: bool,
    pub velocity: Vec3,
}
