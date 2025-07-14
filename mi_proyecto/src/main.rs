use image::{ImageBuffer, Rgba};

fn main() {
    // Tamaño de la imagen
    let width  = 800;
    let height = 450;
    let mut img: ImageBuffer<Rgba<u8>, _> = 
        ImageBuffer::from_pixel(width, height, Rgba([255,255,255,255]));

 
    let poly1 = vec![
        (165,380),(185,360),(180,330),(207,345),(233,330),
        (230,360),(250,380),(220,385),(205,410),(193,383),
    ];
    let fill1   = Rgba([255,255,  0,255]);
    let border  = Rgba([255,255,255,255]);
    scanline_fill(&poly1, &mut img, fill1);
    draw_edges   (&poly1, &mut img, border);


    let poly2 = vec![
        (321,335),(288,286),(339,251),(374,302),
    ];
    let fill2 = Rgba([  0,  0,255,255]);
    scanline_fill(&poly2, &mut img, fill2);
    draw_edges   (&poly2, &mut img, border);

 
    let poly3 = vec![
        (377,249),(411,197),(436,249),
    ];
    let fill3 = Rgba([255,  0,  0,255]);
    scanline_fill(&poly3, &mut img, fill3);
    draw_edges   (&poly3, &mut img, border);


    let poly4 = vec![
        (413,177),(448,159),(502, 88),(553, 53),(535, 36),(676, 37),
        (660, 52),(750,145),(761,179),(672,192),(659,214),(615,214),
        (632,230),(580,230),(597,215),(552,214),(517,144),(466,180),
    ];
    let fill4   = Rgba([  0,255,  0,255]); 
    scanline_fill(&poly4, &mut img, fill4);
    draw_edges   (&poly4, &mut img, border);


    let hole = vec![
        (682,175),(708,120),(735,148),(739,170),
    ];

    let fill_hole = Rgba([255,255,255,255]);
    scanline_fill(&hole, &mut img, fill_hole);


    img.save("out.bmp").unwrap();
    img.save("out.png").unwrap();
}


fn scanline_fill(
    poly: &[(i32,i32)],
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>
) {
    let y_min = poly.iter().map(|p| p.1).min().unwrap();
    let y_max = poly.iter().map(|p| p.1).max().unwrap();

    for y in y_min..=y_max {
        let mut inters = Vec::new();

        for i in 0..poly.len() {
            let (x0,y0) = poly[i];
            let (x1,y1) = poly[(i+1)%poly.len()];
            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 as f32
                      + (y - y0) as f32 * (x1 - x0) as f32 / (y1 - y0) as f32;
                inters.push(x);
            }
        }
        inters.sort_by(|a,b| a.partial_cmp(b).unwrap());

        for pair in inters.chunks(2) {
            if let [start, end] = pair {
                let xs = start.ceil() as i32;
                let xe = end.floor()  as i32;
                for x in xs..=xe {
                    if x>=0 && y>=0 && (x as u32)<img.width() && (y as u32)<img.height() {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
}

fn draw_edges(
    poly: &[(i32,i32)],
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>
) {
    for i in 0..poly.len() {
        let (x0,y0) = poly[i];
        let (x1,y1) = poly[(i+1)%poly.len()];
        draw_line(x0,y0,x1,y1, img, color);
    }
}


fn draw_line(
    x0: i32, y0: i32, x1: i32, y1: i32,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        if x>=0 && y>=0 && (x as u32)<img.width() && (y as u32)<img.height() {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x==x1 && y==y1 { break; }
        let e2 = err * 2;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 <  dx { err += dx; y += sy; }
    }
}
