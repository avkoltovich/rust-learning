mod numbers;
mod strings;
mod interface;

use numbers::*;
use strings::*;
use interface::*;

fn main() {
    let list: Vec<i32> = vec![78, 33, 20, 90, 65, 6, 54, 15, 15, 33, 33, 6, 15, 15, 15];
    let word: &str = "apple";

    get_average(&list);
    get_median(&list);
    get_number_count(&list);
    get_pig_latin_string(&word);
    start_crm();
}
