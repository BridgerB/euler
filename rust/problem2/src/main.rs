fn main() {
    let mut x = 2;
    let mut y = 3;
    let mut old_max = 0;
    let mut new_max = 0;
    let mut sum = 0;
    while x < 4000000 {
        if x % 2 == 0 {
           sum += x; 
        }
        old_max = y;
        new_max = x + y;
        x = old_max;
        y = new_max;
    } 
    println!("{sum}");
}
