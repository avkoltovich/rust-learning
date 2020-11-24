use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let ad = "John Daggett, 341 King Road, Plymouth MA
Alice Ford, 22 East Broadway, Richmond VA
Sal Carpenter, 73 6th Street, Boston MA";

    let result = by_state(ad);

    println!("{:?}", result)
}

fn by_state(str: &str) -> String {
    let statesMap: HashMap<&str, &str> = [
        ("AZ", "Arizona"),
        ("CA", "California"),
        ("ID", "Idaho"),
        ("IN", "Indiana"),
        ("MA", "Massachusetts"),
        ("OK", "Oklahoma"),
        ("PA", "Pennsylvania"),
        ("VA", "Virginia")
    ].iter().cloned().collect();
}
