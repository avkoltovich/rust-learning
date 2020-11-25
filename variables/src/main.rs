use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let str = "John Pulsett, 321 King Street, Palmouth MA
Alisa Gord, 22 Prin Broadway, Georges VA
Oreste Thulas, 11354 East Bridge Road, Pensa OK
Perry Falpas, 420 Land Road, Beaver Halls PA
Erica Adamson, 200 Station Road, Westbury MA
Paulo Sims, 8A River Street, Richmond VA
Ann Wildon, 334 Shore Parkway, Hill View CA
Al Carpenter, 730 3rd Street, Boston MA";

    let states_map: HashMap<&str, &str> = [
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
            let mut complete_string = states_map[item].to_owned() + "\n";
            let mut filtered_strings: Vec<String> = str
                .lines()
                .filter(|line| line.contains(item))
                .map(|line| "..... ".to_owned() + line + "\n")
                .map(|line| line.replace(item, states_map[item]))
                .collect();

            filtered_strings.sort();

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

    result_string = result_string.replace(",", "");
    result_string = result_string[..result_string.len() - 1].to_string();

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
