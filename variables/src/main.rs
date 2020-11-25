use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let str = "John Daggett, 341 King Road, Plymouth MA
Alice Ford, 22 East Broadway, Richmond VA
Sal Carpenter, 73 6th Street, Boston MA";

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

    let mut states_in_string: Vec<&str> = str
        .lines()
        .map(|line| &line[line.len() - 2..line.len()])
        .collect();
    states_in_string.sort();
    states_in_string.dedup();

    let result_vec: Vec<String> = states_in_string
        .iter()
        .map(|item| {
            let mut complete_string = statesMap[item].to_owned() + "\n";
            let filtered_strings: Vec<String> = str
                .lines()
                .filter(|line| line.contains(item))
                .map(|line| "..... ".to_owned() + line + "\n")
                .map(|line| line.replace(item, statesMap[item]))
                .collect();
            for line in filtered_strings {
                complete_string.push_str(&line[..])
            }
            complete_string
        })
        .collect();

    let mut result_string = String::new();

    for index in 0..result_vec.len() {
        let mut string_to_push = result_vec.get(index).unwrap().to_owned();
        if index > 0 {
            string_to_push.insert_str(0, " ");
        }
        result_string.push_str(&string_to_push[..]);
    }

    println!("{:?}", result_string)
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
    todo!();
}
