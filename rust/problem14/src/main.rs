fn main() {
    let mut largest_chain_number = 0;
    let mut largest_chain = 0;
    // for mut number in 2..1001 {
    for mut number in 1u128..5000000{
        // println!("{number}");
        let mut chain_count = 1;
        let number_copy = number;
        // println!("{number_copy}");
        // let mut my_vector = vec![0,1];
        while number > 1 {
            if number % 2 == 0 {
                number = number/2;
            } else {
                number = number*3+1;
            }
            // println!("{number}");
            chain_count += 1;
        }
        if chain_count > largest_chain {
            largest_chain_number = number_copy;
            largest_chain = chain_count;
            
            println!("{largest_chain_number}, {largest_chain}");
            // println!("{number_copy}, {x}")
        }

        // println!("{number_copy}, {chain_count}");
        // println!("***");


    }
    println!("{largest_chain_number}, {largest_chain}");
}
