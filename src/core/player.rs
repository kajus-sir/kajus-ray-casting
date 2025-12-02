struct Player{
    x: f64,
    y: f64,
    deg: f64,
    move_speed: f64,
    turn_speed: f64,
}

impl Player{
    fn new(x: f64, y: f64, deg: f64) -> Player{
        Player(x, y, deg)
    }
}