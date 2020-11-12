use std::io;
use std::collections::HashMap;

pub fn start_crm() {
    let mut personal: HashMap<&str, Vec<&str>> = HashMap::new();
    
    loop {
        println!("Welcome to Waliot!");

        let mut input: String = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let bytes = input.as_bytes();
        let mut indexes_of_space: Vec<usize> = vec![];

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                indexes_of_space.push(i);
            }
        }

        indexes_of_space.push(input.len());

        let mut words: Vec<&str> = vec![];

        let mut start_index: usize = 0;
        for i in indexes_of_space {
            words.push(input[start_index..i].trim());
            start_index = i;
        }

        let name = words[1].clone();
        let department = words[3].clone();

        if personal.contains_key(department) {
            if let Some(x) = personal.get_mut(department) {
                x.push(name);
                println!("Hello!")
            }
        } else {
            personal.insert(department, vec![name]);
        }

        println!("{:?}", personal);
    }
}