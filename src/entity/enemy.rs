use crate::render::Render;

use super::Entity;

pub trait Enemy: Entity + Render {
    fn is_alive(&self) -> bool;
    fn dmg(&self) -> u32;
}
