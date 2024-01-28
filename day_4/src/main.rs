fn main() {
    println!("Hello, world!");
    let running_days:Vec<i32> = vec![6,5,4,3,2,1,0];
    // clone the vector
    // let progressed_days = progress_days(running_days.clone());

    // Borrowing 
    // borrowing it as a reference (&Vec<i32>) is generally a more efficient option
    let progressed_days = progress_days(&running_days);
    println!("Progressed Days: {}", progressed_days);

    // just deep going with for loop

    //in range increase number with 2
    for i in (0..=10).step_by(2) {
        println!("{}", i);
    }

    // ownership issue with running_days. It seems like progress_days function is taking ownership of running_days, 
    // and then you're trying to use running_days again in the subsequent code.

    for (index, kms) in running_days.iter().enumerate(){
        println!("In Day {}, running kms is {}", index, kms);
    }

    // reverse a range
    for i in (0..=5).rev(){
        println!("{}", i);
    }
    
}

fn progress_days(running_days: &Vec<i32>)-> i32{
    let mut progress_count :i32 =0;
    for i in 0..running_days.len()-1 {
        if (running_days[i+1] - running_days[i]) >= 1 {
            progress_count += 1;
        }
    }
    return progress_count;
}

// #### for Loop structure #######
