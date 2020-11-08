fn main() {
    let temperature = temperature_converter_to_fahrenheit(32);
    println!("Температура по Фаренгейту: {}F", temperature);

    generator_fibonacci(10);
}

fn temperature_converter_to_fahrenheit(celsius: i32) -> f64 {
    let float_celsius = celsius as f64;
    float_celsius * 9.0 / 5.0 + float_celsius
}

fn generator_fibonacci(count: i32) {
    let mut prev_one = 0;
    let mut prev_two = 1;
    let mut i = 2;
    println!("{}", prev_one);
    println!("{}", prev_two);

    while i < count {
        let current_value = prev_one + prev_two;
        prev_one = prev_two;
        prev_two = current_value;
        println!("{}", current_value);
        i += 1
    }
}