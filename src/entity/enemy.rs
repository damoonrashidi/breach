use crate::render::Render;

use super::{Collidable, Entity};

pub trait Enemy: Entity + Render + Collidable {
    fn is_alive(&self) -> bool;
    fn dmg(&self) -> u32;
}
