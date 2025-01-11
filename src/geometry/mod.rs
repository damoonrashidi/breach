#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos(pub f32, pub f32);

impl Pos {
    #[must_use]
    pub fn transpose(&self, Pos(dx, dy): Pos) -> Pos {
        Pos(self.0 + dx, self.1 + dy)
    }

    #[must_use]
    pub fn angle(&self, other: &Pos) -> f32 {
        let delta_x = other.0 - self.0;
        let delta_y = other.1 - self.1;

        delta_y.atan2(delta_x)
    }
}

impl From<Pos> for (u16, u16) {
    fn from(value: Pos) -> Self {
        (value.0 as u16, value.1 as u16)
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub pos: Pos,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    #[must_use]
    pub fn new(pos: &Pos, w: f32, h: f32) -> Self {
        Self { pos: *pos, w, h }
    }

    #[must_use]
    pub fn center(&self) -> Pos {
        Pos(
            (self.pos.0 + self.pos.0 + self.w) / 2.,
            (self.pos.1 + self.pos.1 + self.h) / 2.,
        )
    }

    #[must_use]
    pub fn contains(&self, Pos(x, y): &Pos) -> bool {
        (self.pos.0..self.pos.0 + self.w).contains(x)
            && (self.pos.0..self.pos.0 + self.h).contains(y)
    }

    #[must_use]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.pos.0 < other.pos.0 + other.w
            && self.pos.0 + self.w > other.pos.0
            && self.pos.1 < other.pos.1 + other.h
            && self.pos.1 + self.h > other.pos.1
    }
}
