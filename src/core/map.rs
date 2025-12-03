use macroquad::prelude::*;
pub struct Map{

}

impl Map{
    const MAP:[[i32; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    pub const TILE_SIZE: f32 = 64.0;

    pub fn is_wall(x: usize, y: usize) -> bool{
        if x >= 10 || y >= 10 {
            return true;
        }
        Self::MAP[y][x] == 1
    }

    pub fn draw_map(){
        for y in 0..10{
            for x in 0..10{
                if Self::is_wall(x,y) {
                    draw_rectangle(
                        (x as f32) * Self::TILE_SIZE as f32, 
                        (y as f32) * Self::TILE_SIZE as f32,
                        Self::TILE_SIZE as f32, Self::TILE_SIZE as f32, 
                        Color::new(0.1, 0.1, 0.1,1.0)   
                    );
                }
            }
        }
    }
}