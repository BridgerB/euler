fn main() {
    let mut sum = 0;
    let x = 2u128.pow(125);
    let s: String = x.to_string();
    // let s2: String = s[2];
    // println!("{s2}");
    for number in s.chars() {
        println!("{number}");
        let foo = number as u128 - 0x30;
        // println!("{foo}");
        sum += foo;
        // println!("{sum}");
    }
    println!("{sum}");
}
