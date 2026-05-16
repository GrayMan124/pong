use macroquad::prelude::*;

mod ball;
mod player;

#[macroquad::main("Pong")]
async fn main() {
    const PLAYER_H: f32 = 40.;
    const PLAYER_W: f32 = 5.;
    const SCR_H: f32 = 480.0;
    const SCR_W: f32 = 480.0;
    const BALL_R: f32 = 5.0;

    let mut player_1 = player::Player::new(PLAYER_W, PLAYER_H, 470.0, 240.0);
    let mut player_2 = player::Player::new(PLAYER_W, PLAYER_H, 10.0, 240.0);
    let mut ball = ball::Ball::new(260., 260., BALL_R);

    let mut score_1: u32 = 0;
    let mut score_2: u32 = 0;

    set_camera(&Camera2D {
        zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
        target: vec2(SCR_W / 2., SCR_H / 2.),
        ..Default::default()
    });
    loop {
        clear_background(BLACK);
        let (font_size, font_scale, font_aspect) = camera_font_scale(10.);
        let text_params = TextParams {
            font_size,
            font_scale,
            font_scale_aspect: font_aspect,
            ..Default::default()
        };
        draw_text_ex(
            &format!("{p2} : {p1}", p2 = score_2, p1 = score_1),
            SCR_W / 2.,
            10.,
            text_params,
        );
        let delta = get_frame_time();

        //handle some stuff
        handle_movement(&mut player_1, &mut player_2, delta);
        check_collisions(&player_1, &player_2, &mut ball);

        if ball.y >= SCR_H || ball.y <= 0. {
            ball.bounce_top_wall();
        }
        ball.update_pos(delta);
        //Handle score
        match ball.check_scored(SCR_W) {
            ball::Scored::P1 => {
                score_1 += 1;
            }
            ball::Scored::P2 => {
                score_2 += 1;
            }
            ball::Scored::Noone => {}
        }
        draw_circle(ball.x, ball.y, ball.r, WHITE);
        draw_rectangle(player_1.x, player_1.y, player_1.w, player_1.h, WHITE);
        draw_rectangle(player_2.x, player_2.y, player_2.w, player_2.h, WHITE);

        next_frame().await
    }
}

fn handle_movement(player1: &mut player::Player, player2: &mut player::Player, delta: f32) {
    if is_key_down(KeyCode::Up) {
        player1.update_move(delta, -1.0)
    } else if is_key_down(KeyCode::Down) {
        player1.update_move(delta, 1.0)
    }

    //handle player 2 movement
    if is_key_down(KeyCode::W) {
        player2.update_move(delta, -1.0)
    } else if is_key_down(KeyCode::S) {
        player2.update_move(delta, 1.0)
    }
}

fn check_collisions(player1: &player::Player, player2: &player::Player, ball: &mut ball::Ball) {
    if player1.check_collision(ball.x, ball.y, ball.r)
        || player2.check_collision(ball.x, ball.y, ball.r)
    {
        ball.dx = ball.dx * -1.3 + 30.;
        if player1.check_collision(ball.x, ball.y, ball.r) {
            ball.dy = match player1.coll_dir(ball.x, ball.y, ball.r) {
                Some(bounce) => match bounce {
                    player::Bounce::Up => ball.dy - 40.,
                    player::Bounce::Down => ball.dy + 40.,
                },
                None => ball.dy,
            };
        } else {
            ball.dy = match player1.coll_dir(ball.x, ball.y, ball.r) {
                Some(bounce) => match bounce {
                    player::Bounce::Up => ball.dy - 40.,
                    player::Bounce::Down => ball.dy + 40.,
                },
                None => ball.dy,
            };
        }
    }
}
