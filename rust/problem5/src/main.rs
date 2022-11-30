fn main() {
    for number in 1..10000000000{
        if checker(number) {
            // println!("found");
            println!("{number}");
            break;
        }
    }
}

fn checker(foo: u64) -> bool {
    for number in 3..21 {
        if number < 20 && foo % number == 0 {
            continue 
        } else if number == 20  && foo % number == 0 {
            return true;
        } else {
            return false;
        } 
    }
    return false;
}
