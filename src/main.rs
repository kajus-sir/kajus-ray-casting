use macroquad::prelude::*;

mod core;
use core::player::Player;
use core::map::Map;

fn window_conf() -> Conf{
    Conf { 
        window_title: "Ray Casting".to_owned(), 
        window_width: 640, 
        window_height: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new(100.0, 100.0, 0.0);

    let background_color = Color::new(1.0, 1.0, 1.0, 1.0);
    let circle_color= Color::new(255.0, 0.0, 0.0, 255.0);

    loop {
        clear_background(background_color);
        Map::draw_map();
        player.update();
        player.draw_player(10.0, circle_color);
        next_frame().await;
    }
}
