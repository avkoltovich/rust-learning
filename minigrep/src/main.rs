use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Проблема со чтением аргументов: {}", err);
        process::exit(1);
    });

    println!("Ищем {}", config.query);
    println!("В файле {}", config.file_name);

    if let Err(e) = run(config) {
        println!("Ошибка приложения: {}", e);

        process::exit(1);
    }
}

struct Config {
    query: String,
    file_name: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("недостаточно аргументов")
        }

        let query = args[ 1 ].clone();
        let file_name = args[ 2 ].clone();

        Ok(Config { query, file_name })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

        println!("С текстом:\n{}", contents);

        Ok(())
}

