fn main() {
    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;
    for num in 1..101 {
        let mut x = num*num;
        println!("x: {x}, sum: {sum}");
        sum += x; 
        sum2 += num;
    }
    println!("{sum}");
    println!("{sum2}");
    let answer = sum2*sum2-sum;
    println!("{answer}");
}
