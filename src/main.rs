#![allow(dead_code, unused_imports, unused_variables)]
mod game_con;
use game_con::*;
use macroquad::{input, prelude::*, rand, window};

#[macroquad::main(conf)]
async fn main() {
    let mut _score = 0;
    let mut _aspect = 0.0;
    let mut _contr = GameController::default();
    loop {
        _aspect = screen_height() / screen_width();
        clear_background(GRAY);
        _contr.Update();
        draw_text_ex(
            format!("fps: {}", get_fps()).as_str(),
            screen_width() / 2. - 100.,
            screen_height() * 0.5,
            TextParams {
                font_size: 140,
                font_scale_aspect: _aspect,
                color: WHITE,
                ..Default::default()
            }

        );
        next_frame().await;
    }
}
fn conf() -> Conf {
    Conf {
        window_title: String::from("Bob"),
        window_resizable: true,
        fullscreen: true,
        ..Default::default()
    }
}
