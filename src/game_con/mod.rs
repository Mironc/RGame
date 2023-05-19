#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]
mod component;

use component::*;
use macroquad::{audio::*, input, math, prelude::*, rand, window};

pub struct GameController {
    pub transform: Component,
    hard_curve: f64,
    size: f32,
    default_size: f32,
    color: Color,
    hitsound: Sound,
}
impl BaseMethods for GameController {
    fn draw(&mut self) {
        draw_circle(self.transform.x, self.transform.y, self.size, GOLD);
        println!(
            "x = {},y = {}, size = {},hard = {}",
            self.transform.x, self.transform.y, self.size, self.hard_curve
        );
    }
    fn update(&mut self) {
        if (input::is_mouse_button_pressed(MouseButton::Left)) {
            let (x, y) = input::mouse_position();
            let mouse_check = Circle { x, y, r: 0.01 };
            let circle = Circle {
                x: self.transform.x,
                y: self.transform.y,
                r: self.size,
            };
            if circle.overlaps(&mouse_check) {
                self.transform.x = rand::gen_range(
                    0.0 + self.default_size,
                    window::screen_width() - self.default_size,
                );
                self.transform.y = rand::gen_range(
                    0.0 + self.default_size,
                    window::screen_height() - self.default_size,
                );
                play_sound_once(self.hitsound);
                self.size = self.default_size;
            }
        }
        self.hardcur_increment();
        self.size -= get_frame_time() * self.hard_curve as f32;
    }
}
impl GameController {
    pub fn Update(&mut self) {
        self.update();
        self.draw();
    }
    pub fn default(hitsound: Sound) -> GameController {
        GameController {
            transform: Component {
                x: screen_width() / 2.,
                y: screen_height() / 2.,
            },
            hard_curve: 1.0,
            size: 70.0,
            default_size: 70.0,
            color: WHITE,
            hitsound: hitsound,
        }
    }
    fn hardcur_increment(&mut self) {
        self.hard_curve = get_time().sin() + get_time() / 2.0 as f64;
    }
}
