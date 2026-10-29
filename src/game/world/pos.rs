use crate::game::world::CHUNK_SIZE;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn add(&self, b: &Pos) -> Self {
        Pos {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn to_world(&self) -> Pos {
        Pos {
            x: (self.x * CHUNK_SIZE) as f64,
            y: (self.y * CHUNK_SIZE) as f64,
        }
    }
}
