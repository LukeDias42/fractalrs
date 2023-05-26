use image::{RgbImage, Rgb};
use std::{process, env};
use stopwatch::Stopwatch;

fn main() {
    let config = Config::build(env::args());
    if !config.benchmark { save(config.recursions) } else { benchmark() };
}

fn benchmark() {
    let mut sw = Stopwatch::new();
    for recursions in 0..=10 {
        sw.restart();
        let mut side = (3 as u32).pow(recursions);
        if side == 1 { side = 3 };
        let sqr = Square { x0: 0, y0: 0, len: side };
        let black = Rgb([0, 0, 0]);
        let square_colors = vec![Rgb([255, 255, 255])];
        let mut img = RgbImage::from_fn(side, side, |_, _| black);
        sierpinski_carpet(sqr, &mut img, &square_colors, recursions);
        println!("Sierpiński carpet with {recursions} steps, image with sides: {side}x{side}, took {}ms", sw.elapsed_ms());
    }
}

fn save(recursions: u32) {
    let mut side = (3 as u32).pow(recursions);
    while side < 729 { side *= 3; };
    let sqr = Square { x0: 0, y0: 0, len: side };
    let deep_navy_blue = Rgb([63, 63, 116]);
    let square_colors = vec![Rgb([255, 188, 66]), Rgb([209, 74, 80])];
    let mut img = RgbImage::from_fn(side, side, |_, _| deep_navy_blue);

    sierpinski_carpet(sqr, &mut img, &square_colors, recursions);
    save_image(img, side)
}

fn sierpinski_carpet(
    sqr: Square, 
    mut img: &mut RgbImage, 
    colors: &[Rgb<u8>], 
    recursion: u32) {
    let color = colors[recursion as usize % colors.len()];
    fill_center_square(&sqr, &mut img, color);

    if recursion == 0 { return };

    let new_len = sqr.len/3;
    sierpinski_carpet(Square::new(sqr.x0,             sqr.y0,             new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0,             sqr.y0 + new_len,   new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0,             sqr.y0 + 2*new_len, new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0 + new_len,   sqr.y0,             new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0 + new_len,   sqr.y0 + 2*new_len, new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0 + 2*new_len, sqr.y0,             new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0 + 2*new_len, sqr.y0 + new_len,   new_len), &mut img, colors, recursion - 1);
    sierpinski_carpet(Square::new(sqr.x0 + 2*new_len, sqr.y0 + 2*new_len, new_len), &mut img, colors, recursion - 1);
}

fn fill_center_square(sqr: &Square, img: &mut RgbImage, color: Rgb<u8>) {
    let inner_len = sqr.len/3;
    for x in 0..inner_len {
        for y in 0..inner_len {
            img.put_pixel(inner_len + sqr.x0 + x, inner_len + sqr.y0 + y, color);
        }
    }
}

fn save_image(img: RgbImage, side: u32) {
    let base_dir = "./";
    let file_name = "sierpinski_carpet";
    let file_ext = ".png";
    let file_path = base_dir.to_string() + file_name + file_ext;
    img.save(&file_path)
        .unwrap_or_else(
            |err| {
                println!("An error occured while trying to save the image: {err}");
                process::exit(1);
            }
        );
    println!("Sierpiński carpet!\nFile: {file_path}\nImage size {side}x{side}");
}

#[derive(Debug)]
struct Square {
    x0: u32,
    y0: u32,
    len: u32
}

impl Square {
    pub fn new(x0: u32, y0: u32, len: u32) -> Square {
        Square { x0, y0, len }
    }
}

struct Config {
    recursions: u32,
    benchmark: bool
}

impl Config {
    pub fn build(
            mut args: impl Iterator<Item = String>
    ) -> Config {
        args.next();

        let recursions = match args.next() {
            Some(arg) => arg.parse().unwrap_or_else(|_| {
                println!("Couldn't parse argument. Using default recursion: 3.");
                3
            }),
            None => {
                println!("Using default recursion amount: 3.");
                3
            }
        };

        let benchmark = env::var("BENCHMARK").is_ok();

        Config {
            recursions,
            benchmark
        }
    }
} 
