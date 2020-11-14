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
        show_org: String::from("Покажи всю фирму"),
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
            print_full_org(&personal);
            continue;
        }

        let indexes_for_slice = get_indexes_to_slice(&input);
        let words: Vec<String> = get_sliced_words(&input, &indexes_for_slice);
        let department_command = format!("{} {}", words[0], words[1]);

        if department_command == commands.show_department {
            print_department_personal(&personal, &words[words.len() - 1]);
            continue;
        }

        if indexes_for_slice.len() > 1 {
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

fn get_sliced_words(string: &String, indexes: &Vec<usize>) -> Vec<String> {
    let mut words: Vec<String> = vec![];
    let input = string.clone();

    let mut start_index: usize = 0;
    for i in indexes {
        words.push(input[start_index..i.clone()].trim().to_string());
        start_index = i.clone();
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

fn print_full_org(personal_map: &HashMap<String, Vec<String>>) {
    let mut all_persons: Vec<String> = vec![];
    
    for i in personal_map.iter() {
        let (_, list) = i;
        let mut value = list.clone();
        all_persons.append(&mut value);
    }

    println!("Весь персонал компании: {:?}", all_persons);
}

fn print_department_personal(personal_map: &HashMap<String, Vec<String>>, department_name: &String) {
    match personal_map.get(department_name) {
        Some(value) => println!("Отдел {}: {:?}", department_name, value),
        None => println!("Отдел не найден")
    }
}