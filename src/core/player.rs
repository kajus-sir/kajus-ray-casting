use std::f64::consts::PI;
use macroquad::prelude::*;

pub struct Player{
    pub x: f64,
    pub y: f64,
    pub rad: f64,
    move_speed: f64,
    turn_speed: f64,
}

impl Player{
    pub fn new(x: f64, y: f64, rad: f64) -> Player{
        Player{
            x, 
            y, 
            rad,
            move_speed: 1.0,
            turn_speed: 0.05,
        }
    }

    pub fn turn_left(&mut self){
        self.rad -= self.turn_speed;
        if self.rad < 0.0 {
            self.rad += 2.0 * PI;
        }
    }

    pub fn turn_right(&mut self){
        self.rad += self.turn_speed;
        if self.rad >= 2.0 * PI {
            self.rad -= 2.0 * PI;
        }
    }

    pub fn move_forward(&mut self){
        self.x += self.rad.cos() * self.move_speed;
        self.y += self.rad.sin() * self.move_speed;
    }

    pub fn move_backwards(&mut self){
        self.x -= self.rad.cos() * self.move_speed;
        self.y -= self.rad.sin() * self.move_speed;
    }

    pub fn update(&mut self){
        if is_key_down(KeyCode::W) {self.move_forward();}
        if is_key_down(KeyCode::A) {self.turn_left();}
        if is_key_down(KeyCode::S) {self.move_backwards();}
        if is_key_down(KeyCode::D) {self.turn_right();}
    }

    pub fn draw_player(&self, radius: f32, color: Color) {
        draw_circle(self.x as f32, self.y as f32, radius, color);

        let line_length: f32 = radius;
        let x2: f32 = self.x as f32 + (self.rad as f32).cos() * line_length;
        let y2: f32 = self.y as f32 + (self.rad as f32).sin() * line_length;

        draw_line(self.x as f32, self.y as f32, x2, y2, 3.0, Color { r: (0.0), g: (0.0), b: (0.0), a: (255.0) });
    }
}