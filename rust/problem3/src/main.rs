fn main() {
    for number in 2..600851475143 {
        
        if 600851475143 % number == 0 && is_prime(number) {
            println!("{number}");
            //todo: try to remoe lines below.
            if number > 5000 {
                break;
            }
        } 
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}
