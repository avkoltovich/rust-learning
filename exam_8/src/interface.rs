use std::io;
use std::collections::HashMap;

struct Commands {
    stop: String,
    add: String,
    show_org: String,
    show_department: String,
}

pub fn start_crm() {
    let mut personal: HashMap<String, Vec<String>> = HashMap::new();
    let commands = Commands {
        stop: String::from("Выйти"),
        add: String::from("Добавь"),
        show_org: String::from("Покажи фирму"),
        show_department: String::from("Покажи отдел"),
    };
    
    loop {
        println!("Введите команду:");

        let input: String = get_input_string();

        if input == commands.stop {
            println!("Всего доброго!");
            break;
        }

        if input == commands.show_org {
            println!("Покажу всю фирму, но позже");
            continue;
        }

        if input == commands.show_department {
            println!("Покажу весь отдел, но позже");
            continue;
        }

        let indexes_of_spaces = get_indexes_to_slice(&input);

        if indexes_of_spaces.len() > 1 {
            let words: Vec<String> = get_sliced_words(&input, indexes_of_spaces);

            if words[0] == commands.add {
            store_personal(&mut personal, &words);
            println!("{:?}", personal);
            }
        }

        continue;
    }
}

fn get_input_string() -> String {
    let mut input: String = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn get_indexes_to_slice(string: &String) -> Vec<usize> {
    let input = string.clone();
    let bytes = input.as_bytes();
    let mut indexes_of_space: Vec<usize> = vec![];

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            indexes_of_space.push(i);
        }
    }
    indexes_of_space.push(input.len());

    return indexes_of_space;
}

fn get_sliced_words(string: &String, indexes: Vec<usize>) -> Vec<String> {
    let mut words: Vec<String> = vec![];
    let input = string.clone();

    let mut start_index: usize = 0;
    for i in indexes {
        words.push(input[start_index..i].trim().to_string());
        start_index = i;
    }

    return words;
}

fn store_personal(personal_map: &mut HashMap<String, Vec<String>>, command_line: &Vec<String>) {
    let words = command_line.clone();

    let name: String = words[1].clone();
    let department: String = words[3].clone();

    if personal_map.contains_key(&department) {
        if let Some(x) = personal_map.get_mut(&department) {
            x.push(name);
            println!("Успешно добавлен!")
        }
    } else {
        personal_map.insert(department, vec![name]);
        println!("Успешно добавлен!")
    }
}