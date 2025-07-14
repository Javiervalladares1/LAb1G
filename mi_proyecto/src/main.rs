use raylib::prelude::*;

fn main() {
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Poligon-3")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Vértices del polígono 3
        let poly3 = [
            Vector2::new(377.0, 249.0),
            Vector2::new(411.0, 197.0),
            Vector2::new(436.0, 249.0),
        ];

        // Relleno rojo
        d.draw_triangle(poly3[0], poly3[1], poly3[2], Color::RED);

        // Borde blanco
        for i in 0..poly3.len() {
            let a = poly3[i];
            let b = poly3[(i + 1) % poly3.len()];
            d.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, Color::WHITE);
        }
    }
}
