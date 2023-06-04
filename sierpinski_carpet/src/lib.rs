use image::{Rgb, RgbImage};
use std::{cmp, process};
use stopwatch::Stopwatch;

/// Creates a file containing an image with the specified sierpinski carpet configuration.
/// If the file with a certain configuration has already been created, does not create it again.
///
/// # Panics
/// If the amount of steps is too large, your computer might not be able to handle creating
/// an image large enough and storing it.
pub fn create(steps: u32, color_option: u32) {
    let colors = Colors::get_colors(color_option);
    let file_path = get_image_path(steps, &colors.name);
    if image_exists(&file_path) {
        println!("Image with that configuration already exists on this folder.");
        return;
    }

    let side = cmp::max(743, (3u32).pow(steps));
    let carpet = sierpinski_carpet(steps, side, colors);
    save_image(carpet, side, file_path)
}

fn sierpinski_carpet(steps: u32, side: u32, colors: Colors) -> RgbImage {
    let mut carpet = RgbImage::from_fn(side, side, |_, _| colors.background);
    let sqr = Square {
        starting_point: (0, 0),
        len: side,
    };
    let mut stack = vec![(sqr, steps)];

    while !stack.is_empty() {
        let (sqr, steps) = stack.pop().unwrap();
        let layer_color = colors.get_layer_color(steps as usize);
        fill_center_square(&sqr, &mut carpet, layer_color);

        if steps == 0 { continue; };

        let new_len = sqr.len / 3;
        for i in 0..=8 {
            if i == 4 { continue; }
            stack.push((
                Square {
                    starting_point: (
                        sqr.starting_point.0 + (i % 3) * new_len, 
                        sqr.starting_point.1 + (i / 3) * new_len
                    ),
                    len: new_len
                },
                steps - 1,
            ));
        }
    }

    carpet
}

fn fill_center_square(sqr: &Square, img: &mut RgbImage, color: Rgb<u8>) {
    let inner_len = sqr.len / 3;
    for x in 0..inner_len {
        for y in 0..inner_len {
            img.put_pixel(
                inner_len + sqr.starting_point.0 + x, 
                inner_len + sqr.starting_point.1 + y, 
                color);
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
    img.save(&file_path).unwrap_or_else(|err| {
        println!("An error occured while trying to save the image: {err}");
        process::exit(1);
    });
    println!("Sierpiński carpet!\nFile: {file_path}\nImage size {side}x{side}");
}

/// Represents a Square
/// The Starting point is the upper left corner
struct Square {
    starting_point: ( u32, u32 ),
    len: u32,
}

struct Colors {
    name: String,
    background: Rgb<u8>,
    layers: Vec<Rgb<u8>>,
}

impl Colors {
    fn get_layer_color(&self, steps: usize) -> Rgb<u8> {
        self.layers[steps % self.layers.len()]
    }
}

impl Colors {
    fn get_colors(option: u32) -> Colors {
        match option {
            1 => Colors {
                name: String::from("Classic_Contrast"),
                background: Rgb([0, 0, 0]),
                layers: vec![Rgb([255, 255, 255])],
            },
            2 => Colors {
                name: String::from("Inverted_Classic"),
                background: Rgb([255, 255, 255]),
                layers: vec![Rgb([0, 0, 0])],
            },
            3 => Colors {
                name: String::from("Gold_Velvet"),
                background: Rgb([63, 63, 116]),
                layers: vec![Rgb([255, 188, 66]), Rgb([209, 74, 80])],
            },
            4 => Colors {
                name: String::from("Earthy_Tones"),
                background: Rgb([85, 98, 112]),
                layers: vec![Rgb([255, 193, 87]), Rgb([54, 179, 126]), Rgb([194, 62, 62])],
            },
            5 => Colors {
                name: String::from("Contrast_Harmony"),
                background: Rgb([33, 33, 33]),
                layers: vec![Rgb([239, 108, 0]), Rgb([181, 181, 181])],
            },
            6 => Colors {
                name: String::from("Bold_Neutrals"),
                background: Rgb([64, 64, 64]),
                layers: vec![Rgb([255, 145, 0]), Rgb([255, 188, 66]), Rgb([0, 0, 0])],
            },
            _ => Colors {
                name: String::from("Sophisticated_Duo"),
                background: Rgb([48, 63, 159]),
                layers: vec![Rgb([244, 143, 177]), Rgb([242, 203, 68])],
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
        let side = cmp::max((3_u32).pow(steps), 3);
        for _ in 0..iterations {
            let colors = Colors {
                name: String::from("Classic_Contrast"),
                background: Rgb([0, 0, 0]),
                layers: vec![Rgb([255, 255, 255])],
            };
            sierpinski_carpet(steps, side, colors);
        }
        println!(
            "Steps: {steps}; Size: {side:>5}x{side:<5}; Avarage: {}ms",
            sw.elapsed_ms() / iterations
        );
    }
}
