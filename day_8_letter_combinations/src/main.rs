use std::{collections::HashMap};

fn main() {
    println!("Hello, world!");
    let s_my_str = comb_letter_phone_num("2");
    println!("{:?}", s_my_str);
}

fn comb_letter_phone_num(my_str: &str) -> Vec<String> {
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    let mut mychar: char = 'a';

    for n in 2..=9 {
        if n == 7 || n == 9 {
            for _j in 1..=4 {
                map.entry(n).or_insert(Vec::new()).push(mychar);
                mychar = (mychar as u8 + 1) as char;
            }
        } else {
            for _j in 1..4 {
                map.entry(n).or_insert(Vec::new()).push(mychar);
                mychar = (mychar as u8 + 1) as char;
            }
        }
    }

    // let mut result: Vec<String> = vec![];
    let mut combinations = Vec::new();
    let mut all_the_letters: Vec<Vec<String>> = vec![];

    if my_str.len() >= 1 {
        let new_map = map.clone();
        for num in my_str.chars() {
            let temp_result = give_individual_letter(num, new_map.clone());
            all_the_letters.push(temp_result);
        }
    }
    generate_all_the_combinations(&all_the_letters, 0, String::new(), &mut combinations);
    return combinations;
}

fn give_individual_letter(my_char: char, map: HashMap<i32, Vec<char>>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    match my_char.to_string().parse::<i32>() {
        Ok(num) => {
            if let Some(value) = map.get(&num) {
                for n in value {
                    let s = n.to_string();
                    result.push(s);
                }
            }
        }
        Err(_e) => {
            println!("Failed to parse integer");
        }
    }
    return result;
}

fn generate_all_the_combinations(
    array: &Vec<Vec<String>>,
    index: usize,
    prefix: String,
    combinations: &mut Vec<String>,
) {
    if index == array.len() {
        combinations.push(prefix);
        return;
    }

    for c in &array[index] {
        
        let new_prefix = format!("{}{}", prefix, c);
        generate_all_the_combinations(array, index+1, new_prefix, combinations)
        
    }
}
