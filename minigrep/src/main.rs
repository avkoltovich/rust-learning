use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Проблема со чтением аргументов: {}", err);
        process::exit(1);
    });

    println!("Ищем {}", config.query);
    println!("В файле {}", config.file_name);

    if let Err(e) = minigrep::run(config) {
        println!("Ошибка приложения: {}", e);

        process::exit(1);
    }
}