use raylib::prelude::*;

fn main() {
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Poligon-2")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Vértices del polígono 2
        let poly2 = [
            Vector2::new(321.0, 335.0),
            Vector2::new(288.0, 286.0),
            Vector2::new(339.0, 251.0),
            Vector2::new(374.0, 302.0),
        ];

        // Relleno azul (fan triangulation)
        for i in 1..poly2.len()-1 {
            d.draw_triangle(poly2[0], poly2[i], poly2[i+1], Color::BLUE);
        }

        // Borde blanco
        for i in 0..poly2.len() {
            let a = poly2[i];
            let b = poly2[(i + 1) % poly2.len()];
            d.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, Color::WHITE);
        }
    }
}
