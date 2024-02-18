use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let s_my_str = comb_letter_phone_num("23");
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

    let mut result: Vec<String> = vec![];
    let mut my_num = 0;
    if my_str.len() == 1 {
        match my_str.parse::<i32>() {
            Ok(num) => {
                my_num = num;
            }
            Err(_) => {
                println!("Failed to parse integer");
            }
        }

        if let Some(value) = map.get(&my_num) {
            for n in value {
                let s = n.to_string();
                result.push(s);
            }
        }
    } else if my_str.is_empty() {
    }
    return result;
}

// let string_vec: Vec<&str> = value
//     .iter()
//     .map(|s| s.to_string())
//     .map(|n| n.as_str())
//     .collect();
// result.clone_from_slice(&string_vec);

// for n in mystr.chars() {
//     let temp_num = n as i32;
//     if let Some(value) = map.get(&temp_num) {
//         for i in value {}
//     }
// }
