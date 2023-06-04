use input::Config;
use std::env;

fn main() {
    let config = Config::build(env::args());
    if config.benchmark {
        sierpinski_carpet::benchmark()
    } else {
        sierpinski_carpet::create(config.steps, config.color_option)
    };
}
