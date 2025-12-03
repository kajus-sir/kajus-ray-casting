use macroquad::prelude::*;

use crate::core::map::Map;
pub struct Player{
    pub pos: Vec2,
    pub dir: Vec2,
    pub fov: f32,

    move_speed: f32,
    turn_speed: f32,
}

impl Player{
    pub fn new(pos: Vec2) -> Player{
        Player{
            pos, 
            dir: vec2(1.0,0.0),
            fov: 60f32.to_radians(),
            move_speed: 2.0,
            turn_speed: 0.1,
        }
    }

    pub fn rotate(&mut self, p_dir: Vec2, angle:f32){
        let cos = angle.cos();
        let sin = angle.sin();

        let new_dir = vec2(
            p_dir.x * cos - p_dir.y * sin,
            p_dir.x * sin + p_dir.y * cos,
        );

        self.dir = new_dir;
    }

    pub fn move_forward(&mut self){

        let new_pos = self.pos + self.dir * self.move_speed;

        let tile_pos = vec2(new_pos.x / Map::TILE_SIZE, new_pos.y / Map::TILE_SIZE);
       
        if !Map::is_wall(tile_pos.x as usize, tile_pos.y as usize){
            self.pos = new_pos;
        }

    }

    pub fn update(&mut self){
        if is_key_down(KeyCode::W) {self.move_forward();}
        if is_key_down(KeyCode::A) {self.rotate(self.dir, self.turn_speed * -1.0);}
        if is_key_down(KeyCode::D) {self.rotate(self.dir, self.turn_speed);}
    }

    pub fn draw_player(&self, radius: f32, color: Color) {
        draw_circle(self.pos.x as f32, self.pos.y as f32, radius, color);

        let line_length: f32 = radius;
        let x2: f32 = self.pos.x as f32 + self.dir.x * line_length;
        let y2: f32 = self.pos.y as f32 + self.dir.y * line_length;

        draw_line(self.pos.x as f32, self.pos.y as f32, x2, y2, 3.0, Color { r: (0.0), g: (0.0), b: (0.0), a: (1.0) });
    }
}