const BALL_SPEED: f32 = 80.;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub dx: f32,
    pub dy: f32,
}

pub enum Scored {
    P1,
    P2,
    Noone,
}

impl Ball {
    pub fn new(x: f32, y: f32, r: f32) -> Ball {
        Ball {
            x: x,
            y: y,
            r: r,
            dx: BALL_SPEED,
            dy: 0.,
        }
    }
    pub fn update_pos(&mut self, delta: f32) {
        self.x += self.dx * delta;
        self.y += self.dy * delta;
    }
    pub fn bounce_top_wall(&mut self) {
        self.dy *= -1.;
    }
    pub fn check_scored(&mut self, scr_w: f32) -> Scored {
        if self.x <= 0. {
            self.reset(Scored::P1);
            Scored::P1
        } else if self.x >= scr_w {
            self.reset(Scored::P2);
            Scored::P2
        } else {
            Scored::Noone
        }
    }
    fn reset(&mut self, scored: Scored) {
        self.x = match scored {
            Scored::P1 => 120.,
            Scored::P2 => 380.,
            Scored::Noone => 0.0,
        };
        self.y = 0.0;
        self.dx = BALL_SPEED;
        self.dy = 0.;
    }
}
