fn main() {
    println!("Hello, world!");
    let text = String::from("nemo I am going to found a");
    let text2 = "I am going to found a nemo";
    finding_nemo(text);
    finding_nemo2(text2);
}

fn finding_nemo(text:String) {
    // Split a String
    let text_arr:Vec<&str> = text.split(" ").collect();
    let mut count : i32 = 1;
    for element in text_arr.iter(){
        if element.to_lowercase() == "nemo" {
            break;
        }
        count += 1;
    }
    println!("Nemo is found at: {}", count);
}


fn finding_nemo2(text:&str){
    let text_arr: Vec<&str> = text.split_whitespace().collect();
    let count = text_arr.iter().position(|&x| x.to_lowercase() == "nemo");

    match count {
        Some(index)=> print!("Found 'nemo' at index: {}", index+1),
        None => print!("'Nemo' is not found in the given string"),
    }
}
// ########## |&x| ##############
// |...| is the syntax for defining a closure in Rust.
// &x is the parameter of the closure. It's saying that the closure takes a reference to x as its argument. 
// The & indicates a reference.


// ######### match - same like switch #########
// Some(index) is the first arm of the match, handling the case where count is a Some variant
// None is the second arm of the match, handling the case where count is a None variant