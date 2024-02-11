fn main() {
    println!("Hello, world!");
    let mut my_arr1 :[i32;9] = [1,4,6,4,9,0,0,0,0];
    let my_arr2 :[i32;4] = [2,3,5,11];
    let mid = my_arr1.len() - my_arr2.len();
    for n in mid..my_arr1.len(){
        my_arr1[n] = my_arr2[n-mid];
    }
    merge_sort(&mut my_arr1);
    print!("{:?}", my_arr1)
}

fn merge_sort(arr:&mut [i32]){
    let len = arr.len();
    if len <= 1{
        return;
    }
    let mid = len/2;
    let (left, right) = arr.split_at_mut(mid);
    
    merge_sort(left);
    merge_sort(right);

    let mut result = Vec::with_capacity(len);
    let (mut left_index, mut right_index) = (0,0);

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        }else{
            result.push(right[right_index]);
            right_index += 1;
        }   
    }
    // &left[left_index..] -- This is a slice of the left slice starting from the left_index until the end. 
    // It represents the remaining unprocessed elements of the left slice.
    result.extend_from_slice(&left[left_index..]);
    result.extend_from_slice(&right[right_index..]);
    
    // sorted elements back to the original array
    arr.copy_from_slice(&result);

}
