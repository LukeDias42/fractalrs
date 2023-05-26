use std::env;
use input::Config;
use sierpinski_carpet;

fn main() {
    let config = Config::build(env::args());
    if !config.benchmark { 
        sierpinski_carpet::create(config.steps, config.color_option) 
    } else {
        sierpinski_carpet::benchmark() 
    };
}

