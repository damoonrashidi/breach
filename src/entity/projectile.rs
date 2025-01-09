use super::{Collidable, Entity};
use crate::{geometry::Pos, render::Render};
use std::fmt::Debug;

pub trait Projectile: Entity + Render + Debug + Collidable {
    fn dmg(&self) -> u32;
    fn pos(&self) -> &Pos;
}
