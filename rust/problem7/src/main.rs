fn main() {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    while x != 10001 {
        y += 1;
        if is_prime(y) {
            x += 1;
        }
    }
    println!("{y}")
}

fn is_prime(n: u64) -> bool {
    if n <=1 {
        return false;
    }
    for number2 in 2..n {
        if n % number2 == 0 {
            return false;
        }
    }
    true
}
