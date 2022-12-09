fn main() {
    let mut triangle= vec![
        vec![3],
        vec![7,4],
        vec![2,4,6],
        vec![8,5,9,3]
    ];
    let last_vec = triangle.last().unwrap();
    let second_to_last_number = last_vec[last_vec.len()-2];
    let last_number = last_vec[last_vec.len()-1];
    println!("{second_to_last_number:?}");
    println!("{last_number:?}");

    let second_to_last_vec = &triangle[triangle.len()-2];
    let last_number_second = second_to_last_vec[second_to_last_vec.len()-1];
    println!("{second_to_last_vec:?}");
    println!("{last_number_second:?}");
    if last_number > second_to_last_number {
        last_number_second = last_number_second + last_number;
    } else {
        last_number_second = last_number_second + second_to_last_number;
    }
    println!("{last_number_second:?}");
    
    
}
