const PLAYER_SPEED: f32 = 100.;

pub struct Player {
    pub w: f32,
    pub h: f32,
    pub x: f32,
    pub y: f32,
    speed: f32,
}

pub enum Bounce {
    Up,
    Down,
}

impl Player {
    pub fn new(w: f32, h: f32, x: f32, y: f32) -> Player {
        Player {
            w,
            h,
            x,
            y,
            speed: PLAYER_SPEED,
        }
    }

    pub fn update_move(&mut self, delta: f32, dir: f32) {
        self.y += self.speed * dir * delta;
    }

    pub fn check_collision(&self, x: f32, y: f32, r: f32) -> bool {
        x + r >= self.x && x - r < self.x + self.w && y >= self.y && y < self.y + self.h
    }

    pub fn coll_dir(&self, x: f32, y: f32, r: f32) -> Option<Bounce> {
        if x + r >= self.x && x - r < self.x + self.w && y >= self.y && y < self.y + self.h {
            if y >= self.y && y < self.y + self.h / 2.0 {
                Some(Bounce::Up)
            } else {
                Some(Bounce::Down)
            }
        } else {
            None
        }
    }
}
