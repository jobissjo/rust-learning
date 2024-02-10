fn main() {
    println!("Hello, world!");
    let num:i32 = next_prime(3);
    print!("{}",num)
}


fn next_prime(number:i32)->i32{
    let mut current_number = number;
    while !is_prime_fn(current_number) {
        current_number += 1;
    }
    return current_number;
}

fn is_prime_fn(number:i32)->bool{
    if number<=1 {
        return false;
    }
    let mut is_prime:bool= true;
    let sqrt_num:i32 = (number as f64).sqrt() as i32;
    for i in 2..sqrt_num/2{
        if number % i == 0{
            is_prime = false;
        }
    }
    return is_prime;
}