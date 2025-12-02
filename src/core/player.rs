use std::f64::consts::PI;
use macroquad::prelude::*;

struct Player{
    x: f64,
    y: f64,
    rad: f64,
    move_speed: f64,
    turn_speed: f64,
}

impl Player{
    fn new(x: f64, y: f64, rad: f64) -> Player{
        Player{
            x, 
            y, 
            rad,
            move_speed: 0.1,
            turn_speed: 0.1,
        }
    }

    pub fn turn_left(&mut self){
        self.rad -= turn_speed;
        if(self.rad < 0){
            self.rad += 2 * PI;
        }
    }

    pub fn turn_right(&mut self){
        self.rad += self.turn_speed;
        if(self.rad >= 2 * PI){
            self.rad -= 2 * PI;
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
        if(key_is_down(KeyCode::W)){self.move_forward()}
        if(key_is_down(KeyCode::A)){self.turn_left()}
        if(key_is_down(KeyCode::S)){self.move_backwards()}
        if(key_is_down(KeyCode::D)){self.turn_right()}
    }
}