use std::vec;

fn main() {
    println!("Hello, world!");
    let my_vector : Vec<&str> =[
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",     
        "--xx--x--ox--",
        "--xx--x--ox--"
    ].to_vec();

    let my_vector_2: Vec<&str> = vec![
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];

    let my_vector_3 : Vec<&str> = vec![
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
    ];
    let result1: [i32;2] = barbecue_skewer(my_vector);
    print!("{:?}",result1);

    let result2: [i32;2] = barbecue_skewer(my_vector_2);
    print!("{:?}",result2);

    let result3: [i32;2] = barbecue_skewer(my_vector_3);
    print!("{:?}",result3);


}

fn barbecue_skewer(my_arr:Vec<&str>)-> [i32;2]{
    let mut veg_skewer_count : i32 = 0;
    let mut non_veg_skewer_count : i32 = 0;

    for element in my_arr.iter() {
        if let Some(_index) = element.find('x'){
            non_veg_skewer_count += 1;
        }else{
            veg_skewer_count += 1
        }
    }
    return [veg_skewer_count, non_veg_skewer_count];

}
