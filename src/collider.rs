use crate::in_game::DroppingBlock;
use bevy::prelude::*;
use bevy::sprite::Rect;

/// `BoxCollider` is defined using `Rect` square
#[derive(Component)]
pub struct BoxCollider(pub Rect);

impl BoxCollider {
    pub fn from_square(size: f32) -> Self {
        BoxCollider(Rect {
            min: Vec2::new(-size / 2.0, -size / 2.0),
            max: Vec2::new(size / 2.0, size / 2.0),
        })
    }
}

/// Custom Bevy `Plugin` for handling collisions in the game.
pub struct ColliderPlugin;
impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(box_box_collision);
    }
}

/// Handle collisions between two box colliders
fn box_box_collision(
    mut commands: Commands,
    mut dropped_blocks: Query<(Entity, &BoxCollider, &Transform), Without<DroppingBlock>>,
    mut dropping_block: Query<(Entity, &BoxCollider, &Transform), With<DroppingBlock>>,
) {
    let (dropping_entity, dropping_collider, dropping_transform) =
        dropping_block.get_single().unwrap();

    for (dropped_entity, dropped_collider, dropped_transform) in dropped_blocks.iter() {
        if dropped_transform.translation.x == dropping_transform.translation.x
            && dropped_transform.translation.y == dropping_transform.translation.y
        {
            println!("Collision");
        }
    }
}
