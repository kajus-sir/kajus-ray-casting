use macroquad::prelude::*;

fn window_conf() -> Conf{
    Conf { 
        window_title: "Ray Casting".to_owned(), 
        window_width: 1080, 
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main{window_conf}]
async fn main() {
    let background_color= Color::new(0.1, 0.1, 0.1, 1.0);
    let circle_color= Color::new(255.0, 0.0, 0.0, 255.0);

    loop {
        clear_background(background_color);

        draw_circle(100.0, 100.0, 100.0, circle_color);

        next_frame().await;
    }
}
