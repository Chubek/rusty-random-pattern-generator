
use tiny_skia::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let seed_arg = &args[1];

    let seed = seed_arg.parse::<u64>().expect("Error parsing seed");

    let triangle = crate_triangle(seed);

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bicubic,
        1.0,
        Transform::from_row(1.5, -0.4, 0.0, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_circle(200.0, 200.0, 180.0).unwrap();

    let mut pixmap = Pixmap::new(400, 400).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap.save_png(format!("{}-pattern.png", seed)).unwrap();
}

fn crate_triangle(hash: u64) -> Pixmap {
    let mut rng = ChaCha20Rng::seed_from_u64(hash);

    let r: u8 = rng.gen_range(0..255);
    let g: u8 = rng.gen_range(0..255);
    let b: u8 = rng.gen_range(0..255);


    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;

    let start: u8 = rng.gen_range(0..10);
    let end: u8 = rng.gen_range(10..25);

    let start_second: u8 = rng.gen_range(25..50);
    let end_second: u8 = rng.gen_range(50..75);

    let start_third: u8 = rng.gen_range(10..20);
    let end_third: u8 = rng.gen_range(0..20);

    let mut pb = PathBuilder::new();
    pb.move_to(start as f32, end as f32);
    pb.line_to(start_second as f32, end_second as f32);
    pb.line_to(start_third as f32, end_third as f32);   

    let path = pb.finish().unwrap();

    let mut pixmap = Pixmap::new(20, 20).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap
}