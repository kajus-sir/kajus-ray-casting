use macroquad::prelude::*;

use crate::core::config;
use crate::core::map::Map;
pub struct Ray;

impl Ray{
    pub fn cast_ray(start_pos: Vec2, dir: Vec2 ) -> f32{
        let mut distance = 0.0;
        let mut current_pos: Vec2 = start_pos;

        loop {
            current_pos += config::RAY_SPEED * dir;
            distance += config::RAY_SPEED;

            let tile = ivec2((current_pos.x / config::TILE_SIZE) as i32, (current_pos.y / config::TILE_SIZE) as i32);
            if Map::is_wall(tile.x as usize, tile.y as usize) {
                break;
            }
        }
        distance
    }

    pub fn cast_rays(player_pos: Vec2, player_dir: Vec2, fov: f32) -> Vec<f32>{
        let mut rays: Vec<f32> = Vec::new();
        let num_rays = config::WINDOW_WIDTH as usize;

        let player_angle = player_dir.y.atan2(player_dir.x);
    
        for i in 0..num_rays {
            let angle_offset = (i as f32 / num_rays as f32) * fov - (fov / 2.0);
            let ray_angle = player_angle + angle_offset;
        
            let ray_dir = vec2(ray_angle.cos(), ray_angle.sin());
        
            let distance = Self::cast_ray(player_pos, ray_dir);
            rays.push(distance);
        }

        rays
    }

}