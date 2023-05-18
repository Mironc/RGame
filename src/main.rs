use std;
use macroquad::{rand,input, prelude::*, window};
#[macroquad::main(conf)]
async fn main() {
    let mut _score = 0;
    let mut input: Input = Input::default();
    let (mut x, mut y) = (screen_width() / 2.0,screen_height() / 2.0);
    let r = 70.;
    
    loop {
        input.update();
        let circle = Circle::new(x, y, r);
        if input.left_click {
            let mouse_circ = Circle::new(input.mouse_x, input.mouse_y, 1.);

            if mouse_circ.overlaps(&circle) 
            {
                _score += 1;
                (x,y) = (rand::gen_range(0.0 + r, window::screen_width() - r) ,rand::gen_range(0.0 + r,window::screen_height() - r));
                println!("x = {}",x);
                println!("y = {}",y);
            }
        }
        clear_background(GRAY);
        draw_circle(x, y, r, RED);
        next_frame().await;
    }
}
struct Input {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub left_click: bool,
    pub right_click: bool
}
#[allow(dead_code)]
impl Input {
    fn update(&mut self) {
        (self.mouse_x, self.mouse_y) = input::mouse_position();
        self.left_click = input::is_mouse_button_down(MouseButton::Left);
        self.right_click = input::is_mouse_button_down(MouseButton::Right);
    }
    fn default() -> Input {
        Input {
            mouse_x: 0.0,
            mouse_y: 0.0,
            left_click: false,
            right_click: false,
        }
    }
}
fn conf() -> Conf {
    Conf {
        window_title: String::from("Bob"),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}
