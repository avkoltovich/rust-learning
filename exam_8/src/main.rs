use std::collections::HashMap;

fn main() {
    let list: Vec<i32> = vec![78, 33, 20, 90, 65, 6, 54, 15, 15, 33, 33, 6, 15, 15, 15];
    let word = String::from("first");

    get_average(&list);
    get_median(&list);
    get_number_count(&list);
    get_pig_latin_string(&word);
}

fn get_average(list: &Vec<i32>) {
    let mut sum: i32 = 0;
    let list_length: &usize = &list.len();

    for i in list {
        sum += i;
    }

    let average = sum / *list_length as i32;

    println!("Среднее арифметическое: {}", average)
}

fn get_median(list: &Vec<i32>) {
    let median: i32;
    let mut list_copy: Vec<i32> = list.clone();
    list_copy.dedup();
    list_copy.sort();

    let list_length: &usize = &list.len();

    if list_length % 2 == 0 {
        let first_index = list_length / 2;
        let second_index = first_index + 1;

        median = (list_copy[first_index] + list_copy[second_index]) / 2;
    } else {
        let mut index: f32 = (list_length / 2) as f32;
        index = index.ceil();
        let index_i32 = index as usize;

        median = list_copy[index_i32];
    }

    println!("Среднее Медианное: {}", median);
}

fn get_number_count(list: &Vec<i32>) {
    let mut map: HashMap<&i32, i32> = HashMap::new();
    let mut biggest_value: &i32 = &0;
    let mut biggest_key: &i32 = &0;

    for i in list {
        if map.contains_key(i) {
            if let Some(x) = map.get_mut(&i) {
                *x += 1;
            }
        } else {
            map.insert(i, 1);
        }
    }

    for (key, val) in map.iter() {
        if val > &biggest_value {
            biggest_value = val;
            biggest_key = key;
        }
    }

    println!("Чаще всего встречается: {}", biggest_key);
}

fn get_pig_latin_string(string: &String) {}
