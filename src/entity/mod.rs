pub mod enemies;
pub mod enemy;
pub mod player;
pub mod projectile;
pub mod projectiles;

use std::fmt::Debug;

use crate::{geometry::Rect, state::State};

pub trait Entity: Debug {
    fn id(&self) -> String;
    fn update(&mut self, state: &State);
}

pub trait Collidable: Entity {
    fn hitbox(&self) -> Rect;
    fn on_hit(&mut self, other: &dyn Collidable, state: &mut State);
    fn as_any(&self) -> &dyn std::any::Any;
}
