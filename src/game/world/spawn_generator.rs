use crate::{
    game::world::pos::{ChunkPos, Pos},
    util::rng::Rng,
};

pub struct SpawnGenerator {
    seed: i32,
    free: Vec<ChunkPos>,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl SpawnGenerator {
    pub fn new(seed: i32) -> Self {
        SpawnGenerator {
            seed,
            free: Vec::new(),
            x: 0,
            y: 0,
            dx: 0,
            dy: -2,
        }
    }

    /** Should never be called by any other function than next. */
    fn next_chunk_pos(&mut self) -> ChunkPos {
        let new_spawn: ChunkPos;
        if (self.y % 2 == 0) && ((self.y / 2) % 2 != 0) {
            new_spawn = ChunkPos {
                x: self.x - 1,
                y: self.y,
            };
        } else {
            new_spawn = ChunkPos {
                x: self.x,
                y: self.y,
            };
        }

        if (self.x == self.y)
            || (self.x < 0 && self.x == -self.y)
            || (self.x > 0 && self.x == 2 - self.y)
        {
            let new_dx = -self.dy;
            self.dy = self.dx;
            self.dx = new_dx;
        }

        self.x += self.dx;
        self.y += self.dy;

        new_spawn
    }

    fn get_spawn_in_chunk(&self, pos: &ChunkPos) -> Pos {
        let mut rng = Rng::from_parts(&[self.seed, pos.x, pos.y]);
        let x = rng.next_f64();
        let y = rng.next_f64();

        Pos { x, y }
    }

    pub fn next(&mut self) -> Pos {
        let chunk_pos = self.next_chunk_pos();
        let spawn_in_chunk = self.get_spawn_in_chunk(&chunk_pos);

        chunk_pos.to_world().add(&spawn_in_chunk)
    }
}
