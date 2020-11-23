fn main() {
    let arr_a = &[-3, 4, 2];
    let arr_b = &[1, 2, 3];

    let result = solve("CODe");

    println!("{:?}", result)
}

fn solve(s: &str) -> String {
    let mut upper_count = 0;
    let mut lower_count = 0;

    let string = s.to_string();

    for symbol in string.chars() {
        if symbol.is_ascii_lowercase() {
            lower_count += 1;
        } else {
            upper_count += 1;
        }
    }

    if lower_count >= upper_count {
        return string.to_lowercase();
    } else {
        return string.to_uppercase();
    }
}
