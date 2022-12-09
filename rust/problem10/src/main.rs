fn main() {
    let mut sum = 0;
    let mut count = 0;
    for i in 1..2000001 {
        if is_prime(i) == true {
            count += 1;
            sum += i;
            println!("{i}:{sum}:{count}")

        }
    }
    println!("{sum}")
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let upperLimit = (n as f64).sqrt() as u64 + 1;
    for a in 2..upperLimit {
    // for a in 2..n{
        if n % a == 0 {
            return false;
        }
    }
    true
}
