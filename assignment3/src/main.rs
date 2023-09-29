// take 2 arrays as input and find median of the combined array
fn find_median(arr1: &[i32], arr2: &[i32]) -> f32 {
    let mut combined_array = arr1.to_vec();
    combined_array.extend_from_slice(arr2);
    combined_array.sort();
    let len = combined_array.len();
    if len % 2 == 0 {
        let mid = len / 2;
        return (combined_array[mid - 1] + combined_array[mid]) as f32 / 2.0;
    } else {
        return combined_array[len / 2] as f32;
    }
}

fn main() {

    //1. Even length combined arrray
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = [4, 5, 6];
    println!(
        "Even combined array length median = {}",
        find_median(&arr1, &arr2)
    );

    // 2. Odd length combined array
    let arr3: [i32; 3] = [1, 2, 3];
    let arr4: [i32; 2] = [4, 5];
    println!(
        "Odd combined array length median = {}",
        find_median(&arr3, &arr4)
    );

    // 3. One Empty array
    let arr5: [i32; 0] = [];
    let arr6: [i32; 3] = [1, 3, 2];
    println!("One empty array median = {}", find_median(&arr5, &arr6));
}
