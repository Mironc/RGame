use macroquad::prelude::*;

pub trait BaseMethods {
    fn draw(& mut self);
    fn update(& mut self);
    fn start(& mut self){}
}
pub struct Component
{
    pub x: f32,
    pub y: f32, 
}
#[allow(dead_code)]
impl Component
{
    pub fn push(mut self,x: f32, y: f32)
    {
        self.x += x;
        self.y += y;
    }
    pub fn set_pos(mut self,x:f32,y:f32)
    {
        self.x = x;
        self.y = y;
    }
    pub fn default() -> Component
    {
        Component{x: 0.0,y: 0.0}
    }
}