fn main() {
    let str = "ab   ba   cd";
    let result = reverse_words("ab   ba   cd");

    println!("{:#?}", result)
}

fn reverse_words(str: &str) -> String {  
    let reversed_words = str
        .split_whitespace()
        .map(|word| word
            .chars()
            .rev()
            .collect::<String>()
            )
        .collect::<Vec<String>>();

    let whitespaces: Vec<usize> = str
        .chars()
        .enumerate()
        .filter(|(_, symbol)| symbol == &' ')
        .map(|(index, _)| index)
        .collect();

    let mut template_vec = vec!['_'; str.len()];

    for space_index in whitespaces {
        template_vec[space_index] = ' ';
    }

    let mut template_string = template_vec.iter().collect::<String>();

    for word in reversed_words {
        let mut template = String::new();
        for _ in 0..word.len() {
            template.push('_');
        }

        template_string = template_string.replace(&template[..], &word[..]);
    }

    template_string.to_string()
}