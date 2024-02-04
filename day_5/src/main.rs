//mod to import the rust file
mod another_file;
use std::collections::HashSet;
// if we want to use the function in the file 'file_name::func_name'
fn main() {
    println!("Hello, world!");
    let counts: i32 = count_sock_pairs("DAIJOBUOIIII");
    let counts2: i32 = count_sock_improved("DAIJOBUOIIII");
    println!("myfunc: {},improved fun {}", counts, counts2);
    another_file::my_checking_function();
}

fn count_sock_pairs(socks_str:&str)-> i32{
    
    let mut count:i32 = 0;
    // if socks_str.len() == 0{
    //     return count;
    // }
    let mut already_counted:Vec<char> = vec![];
    for n in socks_str.chars(){
        if  already_counted.iter().any(|&t| t==n) { // |&t| t==n
            let index:usize = already_counted.iter().position(|x| *x == n).unwrap(); // |x| *x == n
            already_counted.remove(index);
            count += 1;   
        }
        else {
            already_counted.push(n)
        } 
    }
    return count;
}

fn count_sock_improved(socks_str:&str)-> i32{
    let mut count:i32 = 0;
    let mut already_counted:HashSet<char> = HashSet::new();

    for sock in socks_str.chars(){
        // This version uses a HashSet for already_counted and uses the take method for HashSet, 
        // which removes and returns the value if it exists
        match already_counted.take(&sock) {
            Some(_)=>{ 
                count += 1;
            }
            None => {
                already_counted.insert(sock);
            }
        }
    }
    return count;
}