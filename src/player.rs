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
    Middle,
}

impl Player {
    pub fn new(w: f32, h: f32, x: f32, y: f32) -> Player {
        Player {
            w: w,
            h: h,
            x: x,
            y: y,
            speed: PLAYER_SPEED,
        }
    }

    pub fn update_move(&mut self, delta: f32, dir: f32) {
        self.y += self.speed * dir * delta;
    }

    pub fn check_collision(&self, x: f32, y: f32, r: f32) -> bool {
        if x + r >= self.x && x - r < self.x + self.w && y >= self.y && y < self.y + self.h {
            true
        } else {
            false
        }
    }

    pub fn coll_dir(&self, x: f32, y: f32, r: f32) -> Bounce {
        if x + r >= self.x && x - r < self.x + self.w && y >= self.y && y < self.y + self.h {
            if y >= self.y && y < self.y + self.h / 2.0 {
                Bounce::Up
            } else {
                Bounce::Down
            }
        } else {
            Bounce::Middle
        }
    }
}
