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
        args.next();

        let steps = match args.next() {
            Some(arg) => arg.parse().unwrap_or_else(|_| {
                println!("Couldn't parse argument. Using default steps: 3.");
                3
            }),
            None => {
                println!("Using default steps amount: 3.");
                3
            }
        };

        let color_option = match args.next() {
            Some(arg) => arg.parse().unwrap_or_else(|_| {
                println!("Couldn't parse argument. Using default colors: Classic.");
                1
            }),
            None => {
                println!("Using default colors amount: Classic.");
                1
            }
        };

        let benchmark = env::var("BENCHMARK").is_ok();

        Config {
            steps,
            benchmark,
            color_option
        }
    }
}
