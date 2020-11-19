use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[ 1 ];
    let file_name = &args[ 2 ];

    println!("Ищем {}", query);
    println!("В файле {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Произошла ошибка во время чтения файла");

        println!("С текстом:\n{}", contents);
}