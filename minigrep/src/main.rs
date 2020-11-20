use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Ищем {}", config.query);
    println!("В файле {}", config.file_name);

    let contents = fs::read_to_string(config.file_name)
        .expect("Произошла ошибка во время чтения файла");

        println!("С текстом:\n{}", contents);
}

struct Config {
    query: String,
    file_name: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[ 1 ].clone();
        let file_name = args[ 2 ].clone();

        Config { query, file_name }
    }
}

