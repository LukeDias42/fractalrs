use image::{Rgb, RgbImage};
use std::{cmp, process};
use stopwatch::Stopwatch;

pub fn create(steps: u32, color_option: u32) {
    let colors = Colors::get_colors(color_option);
    let file_path = get_image_path(steps, &colors.name);
    if image_exists(&file_path) {
        println!("Image with that configuration already exists on this folder.");
        return;
    }

    let side = cmp::max(743, (3 as u32).pow(steps));
    let carpet = sierpinski_carpet(steps, side, colors);
    save_image(carpet, side, file_path)
}

fn sierpinski_carpet(steps: u32, side: u32, colors: Colors) -> RgbImage {
    let mut carpet = RgbImage::from_fn(side, side, |_, _| colors.background);
    let sqr = Square { x0: 0, y0: 0, len: side };
    let mut queue = vec![(sqr, steps)];

    while !queue.is_empty() {
        let (sqr, steps) = queue.pop().unwrap();
        let color = colors.colors[steps as usize % colors.colors.len()];
        fill_center_square(&sqr, &mut carpet, color);

        if steps == 0 { continue };

        let new_len = sqr.len/3;
        for i in 0..=2 {
            for j in 0..=2 {
                if i == 1 && j ==1 { continue; }
                queue.push(
                    (Square::new(
                        sqr.x0 + i*new_len, 
                        sqr.y0 + j*new_len, 
                        new_len), 
                    steps - 1));
            }
        }
    }

    carpet
}

fn fill_center_square(sqr: &Square, img: &mut RgbImage, color: Rgb<u8>) {
    let inner_len = sqr.len/3;
    for x in 0..inner_len {
        for y in 0..inner_len {
            img.put_pixel(inner_len + sqr.x0 + x, inner_len + sqr.y0 + y, color);
        }
    }
}

fn get_image_path(steps: u32, color_name: &str) -> String {
    let base_dir = "./";
    let base_file_name = "sierpinski_carpet";
    let file_ext = ".png";
    format!("{base_dir}{base_file_name}-{steps}-{color_name}{file_ext}")
}

fn image_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
}

fn save_image(img: RgbImage, side: u32, file_path: String) {
    img.save(&file_path)
        .unwrap_or_else(
            |err| {
                println!("An error occured while trying to save the image: {err}");
                process::exit(1);
            }
        );
    println!("Sierpiński carpet!\nFile: {file_path}\nImage size {side}x{side}");
}

struct Square {
    x0: u32,
    y0: u32,
    len: u32
}

impl Square {
    fn new(x0: u32, y0: u32, len: u32) -> Square {
        Square { x0, y0, len }
    }
}

struct Colors {
    name: String,
    background: Rgb<u8>,
    colors: Vec<Rgb<u8>>
}

impl Colors {
    fn get_colors(option: u32) -> Colors {
        match option {
            1 => Colors { 
                name: String::from("Classic_Contrast"), 
                background: Rgb([0, 0, 0]), 
                colors: vec![Rgb([255, 255, 255])]},
            2 => Colors { 
                name: String::from("Inverted_Classic"), 
                background: Rgb([255, 255, 255]), 
                colors: vec![Rgb([0, 0, 0])]},
            3 => Colors {
                name: String::from("Gold_Velvet"),
                background: Rgb([63, 63, 116]),
                colors: vec![Rgb([255, 188, 66]), Rgb([209, 74, 80])],
            },
            4 => Colors {
                name: String::from("Earthy_Tones"),
                background: Rgb([85, 98, 112]),
                colors: vec![Rgb([255, 193, 87]), Rgb([54, 179, 126]), Rgb([194, 62, 62])],
            },
            5 => Colors {
                name: String::from("Contrast_Harmony"),
                background: Rgb([33, 33, 33]),
                colors: vec![Rgb([239, 108, 0]), Rgb([181, 181, 181])],
            },
            6 => Colors {
                name: String::from("Bold_Neutrals"),
                background: Rgb([64, 64, 64]),
                colors: vec![Rgb([255, 145, 0]), Rgb([255, 188, 66]), Rgb([0, 0, 0])],
            },
            _ => Colors {
                name: String::from("Sophisticated_Duo"),
                background: Rgb([48, 63, 159]),
                colors: vec![Rgb([244, 143, 177]), Rgb([242, 203, 68])],
            },
        }
    }
}

pub fn benchmark() {
    let mut sw = Stopwatch::new();
    let iterations = 100;
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("-=-=-=-=-= Sierpiński carpet x{iterations} -=-=-=-=-=");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    for steps in 0..10 {
        sw.restart();
        let side = cmp::max((3 as u32).pow(steps), 3);
        for _ in 0..iterations {
            let colors = Colors { 
                name: String::from("Classic_Contrast"), 
                background: Rgb([0, 0, 0]), 
                colors: vec![Rgb([255, 255, 255])]};
            sierpinski_carpet(
                steps,
                side,
                colors);
        }
        println!("Steps: {steps}; Size: {side:>5}x{side:<5}; Avarage: {}ms", sw.elapsed_ms()/iterations);
    }
}

