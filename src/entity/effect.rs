use super::Entity;
use crate::render::Render;
use std::fmt::Debug;

pub trait Effect: Entity + Render + Debug {
    fn is_done(&self) -> bool;
}
