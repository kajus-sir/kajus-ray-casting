use macroquad::prelude::*;

mod core;
use core::player::Player;
use core::map::Map;
use core::config;

fn window_conf() -> Conf{
    Conf { 
        window_title: "Ray Casting".to_owned(), 
        window_width: config::WINDOW_WIDTH, 
        window_height: config::WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new(vec2(50.0,100.0));

    let circle_color= Color::new(255.0, 0.0, 0.0, 255.0);

    loop {
        Map::draw_map();
        player.update();
        player.draw_player(config::PLAYER_RAD, circle_color);
        next_frame().await;
    }
}
