use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Проблема со чтением аргументов: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка приложения: {}", e);

        process::exit(1);
    }
}