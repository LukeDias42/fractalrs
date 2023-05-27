use std::env;

pub struct Config {
    pub steps: u32,
    pub benchmark: bool,
    pub color_option: u32
}

impl Config {
    pub fn build(
            mut args: impl Iterator<Item = String>
    ) -> Config {
        if env::var("BENCHMARK").is_ok() {
            println!("Running benchnark.");
            return Config { benchmark: true, steps: 0, color_option: 0 };
        }

        args.next();
        let steps = match args.next() {
            Some(arg) => arg.parse().unwrap_or_else(|_| {
                println!("Couldn't parse steps. Using default steps: 6.");
                6
            }),
            None => {
                println!("Using default steps: 6.");
                6
            }
        };

        let color_option = match args.next() {
            Some(arg) => arg.parse().unwrap_or_else(|_| {
                println!("Couldn't parse color option. Using default colors: Classic.");
                1
            }),
            None => {
                println!("Using default colors: Classic.");
                1
            }
        };

        Config {
            steps,
            benchmark: false,
            color_option
        }
    }
}
