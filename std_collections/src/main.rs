fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Третий элемент вектора: {}", third);

    match v.get(1) {
        Some(third) => println!("Третий элемент вектора: {}", third),
        None => println!("Нет значения"),
    }
}
