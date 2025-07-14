use raylib::prelude::*;

fn main() {
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Poligon-4")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Vértices del polígono 4
        let poly4 = [
            Vector2::new(413.0, 177.0),
            Vector2::new(448.0, 159.0),
            Vector2::new(502.0,  88.0),
            Vector2::new(553.0,  53.0),
            Vector2::new(535.0,  36.0),
            Vector2::new(676.0,  37.0),
            Vector2::new(660.0,  52.0),
            Vector2::new(750.0, 145.0),
            Vector2::new(761.0, 179.0),
            Vector2::new(672.0, 192.0),
            Vector2::new(659.0, 214.0),
            Vector2::new(615.0, 214.0),
            Vector2::new(632.0, 230.0),
            Vector2::new(580.0, 230.0),
            // (aquí puedes seguir agregando si faltan vértices)
        ];

        // Relleno (elige el color deseado, por ejemplo Color::GREEN)
        for i in 1..poly4.len()-1 {
            d.draw_triangle(poly4[0], poly4[i], poly4[i+1], /*Color::TU_COLOR_AQUÍ*/);
        }

        // Borde blanco
        for i in 0..poly4.len() {
            let a = poly4[i];
            let b = poly4[(i + 1) % poly4.len()];
            d.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, Color::WHITE);
        }
    }
}
