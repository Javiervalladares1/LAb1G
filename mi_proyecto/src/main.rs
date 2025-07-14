use raylib::prelude::*;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Poligon-1")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);


        let poly1 = [
            Vector2::new(165.0, 380.0),
            Vector2::new(185.0, 360.0),
            Vector2::new(180.0, 330.0),
            Vector2::new(207.0, 345.0),
            Vector2::new(233.0, 330.0),
            Vector2::new(230.0, 360.0),
            Vector2::new(250.0, 380.0),
            Vector2::new(220.0, 385.0),
            Vector2::new(205.0, 410.0),
            Vector2::new(193.0, 383.0),
        ];


        for i in 1..poly1.len()-1 {
            d.draw_triangle(poly1[0], poly1[i], poly1[i+1], Color::YELLOW);
        }


        for i in 0..poly1.len() {
            let a = poly1[i];
            let b = poly1[(i + 1) % poly1.len()];
            d.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, Color::WHITE);
        }
    }
}
